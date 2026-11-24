//! Hardware-accelerated H.264 MP4 encoder using Windows Media Foundation (`IMFSinkWriter`)
//! and D3D11 Video Processor (GPU crop, aspect-preserving scale/letterbox, and BGRA -> NV12).
use crate::gpu::GpuContext;
use crate::ipc::{Fps, Quality};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::ptr::null;
use pk::wide::to_wide;
use pk_windows::Win32::Foundation::RECT as WinRect;
use pk_windows::Win32::Graphics::Direct3D11::{
    D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE, D3D11_BOX, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT,
    D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE, D3D11_VIDEO_PROCESSOR_CONTENT_DESC, D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC,
    D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0, D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0,
    D3D11_VIDEO_PROCESSOR_STREAM, D3D11_VPIV_DIMENSION_TEXTURE2D, D3D11_VPOV_DIMENSION_TEXTURE2D, ID3D11Texture2D, ID3D11VideoProcessor,
    ID3D11VideoProcessorEnumerator, ID3D11VideoProcessorOutputView,
};
use pk_windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_NV12, DXGI_RATIONAL, DXGI_SAMPLE_DESC};
use pk_windows::Win32::Media::MediaFoundation::{
    IMF2DBuffer, IMFAttributes, IMFMediaType, IMFSinkWriter, MF_LOW_LATENCY, MF_MT_AVG_BITRATE, MF_MT_FRAME_RATE, MF_MT_FRAME_SIZE,
    MF_MT_INTERLACE_MODE, MF_MT_MAJOR_TYPE, MF_MT_MPEG2_PROFILE, MF_MT_PIXEL_ASPECT_RATIO, MF_MT_SUBTYPE,
    MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, MF_SINK_WRITER_D3D_MANAGER, MF_SINK_WRITER_STATISTICS, MF_TRANSCODE_CONTAINERTYPE,
    MFCreateAttributes, MFCreateDXGISurfaceBuffer, MFCreateMediaType, MFCreateSample, MFCreateSinkWriterFromURL, MFMediaType_Video,
    MFTranscodeContainerType_MPEG4, MFVideoFormat_ARGB32, MFVideoFormat_H264, MFVideoFormat_NV12, MFVideoInterlace_Progressive,
    eAVEncH264VProfile_Main,
};
use windows_core::{Interface, PCWSTR};
use windows_sys::Win32::Foundation::SYSTEMTIME;
use windows_sys::Win32::Storage::FileSystem::{
    CreateDirectoryW, DeleteFileW, FILE_ATTRIBUTE_HIDDEN, GetFileAttributesExW, GetFileExInfoStandard, MOVEFILE_REPLACE_EXISTING,
    MoveFileExW, SetFileAttributesW, WIN32_FILE_ATTRIBUTE_DATA,
};
use windows_sys::Win32::System::SystemInformation::GetLocalTime;
use windows_sys::Win32::System::Threading::{GetCurrentProcessId, Sleep};
use windows_sys::Win32::UI::Shell::FOLDERID_Videos;

const RING_SIZE: usize = 4;

fn snipsnap_videos_dir() -> Option<String> {
    let videos = pk::env::known_folder(&FOLDERID_Videos).or_else(|| pk::env::var("USERPROFILE").map(|u| format!("{u}\\Videos")))?;
    let dir = format!("{videos}\\SnipSnap");
    let wdir = to_wide(&dir);
    unsafe { CreateDirectoryW(wdir.as_ptr(), null()) };
    Some(dir)
}

/// Builds `%USERPROFILE%\Videos\SnipSnap\SnipSnap_YYYY-MM-DD_HH-MM-SS.mp4` and ensures the
/// folder exists.
pub fn make_output_path() -> Option<String> {
    let dir = snipsnap_videos_dir()?;
    let mut st: SYSTEMTIME = unsafe { core::mem::zeroed() };
    unsafe { GetLocalTime(&mut st) };
    Some(format!("{dir}\\SnipSnap_{:04}-{:02}-{:02}_{:02}-{:02}-{:02}.mp4", st.wYear, st.wMonth, st.wDay, st.wHour, st.wMinute, st.wSecond))
}

/// Hidden staging `.mp4` path in `Videos\SnipSnap` so `IMFSinkWriter` and the hardware H.264 MFT
/// can finish initializing while Armed, then atomically rename on stop or delete on cancel.
fn make_staging_path(gen_id: u32) -> Option<Vec<u16>> {
    let dir = snipsnap_videos_dir()?;
    let pid = unsafe { GetCurrentProcessId() };
    Some(to_wide(&format!("{dir}\\.snipsnap-staging-{pid}-{gen_id}.mp4")))
}

/// Retries `DeleteFileW` briefly in case Media Foundation's async byte-stream worker or an AV
/// minifilter still holds a transient handle for a few milliseconds after `IMFSinkWriter` drops.
fn delete_with_retry(wpath: &[u16]) {
    for _ in 0..15 {
        if unsafe { DeleteFileW(wpath.as_ptr()) } != 0 {
            return;
        }
        unsafe { Sleep(15) };
    }
}

/// Retries `MoveFileExW` briefly for the same transient handle-close / AV-scan window after
/// `IMFSinkWriter::Finalize` + drop.
fn move_with_retry(src: &[u16], dst: &[u16]) -> bool {
    for _ in 0..20 {
        if unsafe { MoveFileExW(src.as_ptr(), dst.as_ptr(), MOVEFILE_REPLACE_EXISTING) } != 0 {
            return true;
        }
        unsafe { Sleep(15) };
    }
    false
}

struct StagingGuard<'a> {
    wpath: &'a [u16],
    active: bool,
}

impl Drop for StagingGuard<'_> {
    fn drop(&mut self) {
        if self.active {
            delete_with_retry(self.wpath);
        }
    }
}

fn pack_u64(hi: u32, lo: u32) -> u64 {
    ((hi as u64) << 32) | (lo as u64)
}

enum Pipeline {
    /// Hardware D3D11 Video Processor: GPU crop + scale/letterbox + BGRA -> NV12.
    Nv12 {
        vp_enum: ID3D11VideoProcessorEnumerator,
        vp: ID3D11VideoProcessor,
        /// Owned GPU BGRA texture so `CreateVideoProcessorInputView` works across all drivers.
        staging: Option<(ID3D11Texture2D, u32, u32)>,
        ring: Vec<(ID3D11Texture2D, ID3D11VideoProcessorOutputView)>,
        ring_idx: usize,
    },
    /// Fallback when a GPU lacks D3D11 video processor NV12 output: GPU crop/copy to ARGB32.
    Bgra { ring: Vec<ID3D11Texture2D>, ring_idx: usize },
}

pub struct VideoEncoder {
    writer: Option<IMFSinkWriter>,
    stream_idx: u32,
    out_w: u32,
    out_h: u32,
    min_interval: i64,
    frame_dur: i64,
    pipeline: Pipeline,
    wpath: Vec<u16>,
    committed: bool,
    frames: u64,
    first_pts: Option<i64>,
    last_pts: i64,
    last_out: Option<ID3D11Texture2D>,
}

// Created in MTA with a multithread-protected D3D11 device; every access is serialized by a Mutex.
unsafe impl Send for VideoEncoder {}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        // Close IMFSinkWriter first so its file handle releases before removing the staging file.
        self.writer = None;
        if !self.committed {
            delete_with_retry(&self.wpath);
        }
    }
}

impl VideoEncoder {
    /// Pre-initializes `IMFSinkWriter`, the hardware H.264 encoder MFT, and the D3D11 Video
    /// Processor NV12 ring onto a hidden staging file so pressing Record starts instantaneously.
    pub fn new_staged(
        gpu: &GpuContext,
        src_w: u32,
        src_h: u32,
        out_w: u32,
        out_h: u32,
        quality: Quality,
        fps: Fps,
        gen_id: u32,
    ) -> Option<Self> {
        let wpath = make_staging_path(gen_id)?;
        // Declared before `writer` so if any step returns `None`, `writer` drops first and then
        // `StagingGuard` removes the 0-byte staging file.
        let mut guard = StagingGuard { wpath: &wpath, active: false };

        let mut attrs: Option<IMFAttributes> = None;
        unsafe { MFCreateAttributes(&mut attrs, 4) }.ok()?;
        let attrs = attrs?;
        unsafe {
            let _ = attrs.SetUnknown(&MF_SINK_WRITER_D3D_MANAGER, &gpu.mf_dxgi_manager);
            let _ = attrs.SetUINT32(&MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, 1);
            let _ = attrs.SetUINT32(&MF_LOW_LATENCY, 1);
            let _ = attrs.SetGUID(&MF_TRANSCODE_CONTAINERTYPE, &MFTranscodeContainerType_MPEG4);
        }

        let writer = unsafe { MFCreateSinkWriterFromURL(PCWSTR(wpath.as_ptr()), None, &attrs) }.ok()?;
        guard.active = true;
        unsafe { SetFileAttributesW(wpath.as_ptr(), FILE_ATTRIBUTE_HIDDEN) };

        let fps_num = fps.fps();
        // Output H.264 stream type.
        let out_type: IMFMediaType = unsafe { MFCreateMediaType() }.ok()?;
        unsafe {
            out_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video).ok()?;
            out_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_H264).ok()?;
            out_type.SetUINT32(&MF_MT_AVG_BITRATE, quality.bitrate(out_w, out_h, fps)).ok()?;
            out_type.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32).ok()?;
            out_type.SetUINT64(&MF_MT_FRAME_SIZE, pack_u64(out_w, out_h)).ok()?;
            out_type.SetUINT64(&MF_MT_FRAME_RATE, pack_u64(fps_num, 1)).ok()?;
            out_type.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, pack_u64(1, 1)).ok()?;
            let _ = out_type.SetUINT32(&MF_MT_MPEG2_PROFILE, eAVEncH264VProfile_Main.0 as u32);
        }
        let stream_idx = unsafe { writer.AddStream(&out_type) }.ok()?;

        // Try hardware NV12 pipeline first; fall back to ARGB32 if unavailable.
        let pipeline = if let Some(nv12) = Self::init_nv12(gpu, &writer, stream_idx, src_w, src_h, out_w, out_h, fps_num) {
            nv12
        } else {
            Self::init_bgra(gpu, &writer, stream_idx, out_w, out_h, fps_num)?
        };

        unsafe { writer.BeginWriting() }.ok()?;
        guard.active = false;
        drop(guard);

        Some(Self {
            writer: Some(writer),
            stream_idx,
            out_w,
            out_h,
            min_interval: fps.min_interval_100ns(),
            frame_dur: fps.frame_dur_100ns(),
            pipeline,
            wpath,
            committed: false,
            frames: 0,
            first_pts: None,
            last_pts: 0,
            last_out: None,
        })
    }

    fn init_nv12(
        gpu: &GpuContext,
        writer: &IMFSinkWriter,
        stream_idx: u32,
        src_w: u32,
        src_h: u32,
        out_w: u32,
        out_h: u32,
        fps_num: u32,
    ) -> Option<Pipeline> {
        let vd = gpu.video_device.as_ref()?;
        let _vc = gpu.video_context.as_ref()?;

        let in_type: IMFMediaType = unsafe { MFCreateMediaType() }.ok()?;
        unsafe {
            in_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video).ok()?;
            in_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_NV12).ok()?;
            in_type.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32).ok()?;
            in_type.SetUINT64(&MF_MT_FRAME_SIZE, pack_u64(out_w, out_h)).ok()?;
            in_type.SetUINT64(&MF_MT_FRAME_RATE, pack_u64(fps_num, 1)).ok()?;
            in_type.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, pack_u64(1, 1)).ok()?;
            writer.SetInputMediaType(stream_idx, &in_type, None).ok()?;
        }

        let content = D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
            InputFrameFormat: D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE,
            InputFrameRate: DXGI_RATIONAL { Numerator: fps_num, Denominator: 1 },
            InputWidth: src_w.max(2),
            InputHeight: src_h.max(2),
            OutputFrameRate: DXGI_RATIONAL { Numerator: fps_num, Denominator: 1 },
            OutputWidth: out_w,
            OutputHeight: out_h,
            ..Default::default()
        };
        let vp_enum = unsafe { vd.CreateVideoProcessorEnumerator(&content) }.ok()?;
        let vp = unsafe { vd.CreateVideoProcessor(&vp_enum, 0) }.ok()?;

        let desc = D3D11_TEXTURE2D_DESC {
            Width: out_w,
            Height: out_h,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_NV12,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: (D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let ov_desc = D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
            ViewDimension: D3D11_VPOV_DIMENSION_TEXTURE2D,
            Anonymous: D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
                Texture2D: pk_windows::Win32::Graphics::Direct3D11::D3D11_TEX2D_VPOV { MipSlice: 0 },
            },
        };

        let mut ring = Vec::with_capacity(RING_SIZE);
        for _ in 0..RING_SIZE {
            let mut tex: Option<ID3D11Texture2D> = None;
            unsafe { gpu.device.CreateTexture2D(&desc, None, Some(&mut tex)) }.ok()?;
            let tex = tex?;
            let mut ov: Option<ID3D11VideoProcessorOutputView> = None;
            unsafe { vd.CreateVideoProcessorOutputView(&tex, &vp_enum, &ov_desc, Some(&mut ov)) }.ok()?;
            ring.push((tex, ov?));
        }

        Some(Pipeline::Nv12 { vp_enum, vp, staging: None, ring, ring_idx: 0 })
    }

    fn init_bgra(gpu: &GpuContext, writer: &IMFSinkWriter, stream_idx: u32, out_w: u32, out_h: u32, fps_num: u32) -> Option<Pipeline> {
        let in_type: IMFMediaType = unsafe { MFCreateMediaType() }.ok()?;
        unsafe {
            in_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video).ok()?;
            in_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_ARGB32).ok()?;
            in_type.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32).ok()?;
            in_type.SetUINT64(&MF_MT_FRAME_SIZE, pack_u64(out_w, out_h)).ok()?;
            in_type.SetUINT64(&MF_MT_FRAME_RATE, pack_u64(fps_num, 1)).ok()?;
            in_type.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, pack_u64(1, 1)).ok()?;
            writer.SetInputMediaType(stream_idx, &in_type, None).ok()?;
        }

        let desc = D3D11_TEXTURE2D_DESC {
            Width: out_w,
            Height: out_h,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_RENDER_TARGET.0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let mut ring = Vec::with_capacity(RING_SIZE);
        for _ in 0..RING_SIZE {
            let mut tex: Option<ID3D11Texture2D> = None;
            unsafe { gpu.device.CreateTexture2D(&desc, None, Some(&mut tex)) }.ok()?;
            ring.push(tex?);
        }
        Some(Pipeline::Bgra { ring, ring_idx: 0 })
    }

    /// Encodes one captured BGRA GPU texture cropped to `crop` (in texture coordinates),
    /// scaling/letterboxing into `(out_w, out_h)` and writing with timestamp `rel_time_100ns`.
    pub fn write_frame(&mut self, gpu: &GpuContext, src: &ID3D11Texture2D, crop: (i32, i32, i32, i32), rel_time_100ns: i64) -> bool {
        let mut src_desc = D3D11_TEXTURE2D_DESC::default();
        unsafe { src.GetDesc(&mut src_desc) };
        let tex_w = src_desc.Width as i32;
        let tex_h = src_desc.Height as i32;
        if tex_w <= 0 || tex_h <= 0 {
            return false;
        }

        // Clamp crop rect to the actual texture bounds (survives mid-recording window resizes).
        let sx0 = crop.0.clamp(0, tex_w - 1);
        let sy0 = crop.1.clamp(0, tex_h - 1);
        let sx1 = crop.2.clamp(sx0 + 1, tex_w);
        let sy1 = crop.3.clamp(sy0 + 1, tex_h);
        let sw = (sx1 - sx0) as u32;
        let sh = (sy1 - sy0) as u32;

        // Compute timestamp relative to first captured frame; drop frames exceeding the chosen FPS cap.
        let pts = match self.first_pts {
            None => {
                self.first_pts = Some(rel_time_100ns);
                0
            }
            Some(t0) => {
                let p = (rel_time_100ns - t0).max(0);
                if p <= self.last_pts + self.min_interval {
                    return true;
                }
                p
            }
        };
        let dur = if self.frames == 0 { self.frame_dur } else { (pts - self.last_pts).clamp(self.frame_dur, 10_000_000) };

        let out_tex = match &mut self.pipeline {
            Pipeline::Nv12 { vp_enum, vp, staging, ring, ring_idx } => {
                let Some(vd) = gpu.video_device.as_ref() else { return false };
                let Some(vc) = gpu.video_context.as_ref() else { return false };

                // Copy the cropped region onto an owned D3D11 texture with SHADER_RESOURCE |
                // RENDER_TARGET bind flags so VideoProcessorInputView works on every GPU driver.
                let need_new = match staging {
                    Some((_, w, h)) => *w != sw || *h != sh,
                    None => true,
                };
                if need_new {
                    let sdesc = D3D11_TEXTURE2D_DESC {
                        Width: sw,
                        Height: sh,
                        MipLevels: 1,
                        ArraySize: 1,
                        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                        Usage: D3D11_USAGE_DEFAULT,
                        BindFlags: (D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
                        CPUAccessFlags: 0,
                        MiscFlags: 0,
                    };
                    let mut stex: Option<ID3D11Texture2D> = None;
                    if unsafe { gpu.device.CreateTexture2D(&sdesc, None, Some(&mut stex)) }.is_err() {
                        return false;
                    }
                    *staging = stex.map(|t| (t, sw, sh));
                }
                let Some((stex, _, _)) = staging.as_ref() else { return false };
                let src_box = D3D11_BOX { left: sx0 as u32, top: sy0 as u32, front: 0, right: sx1 as u32, bottom: sy1 as u32, back: 1 };
                unsafe { gpu.context.CopySubresourceRegion(stex, 0, 0, 0, 0, src, 0, Some(&src_box)) };

                let iv_desc = D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
                    FourCC: 0,
                    ViewDimension: D3D11_VPIV_DIMENSION_TEXTURE2D,
                    Anonymous: D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
                        Texture2D: pk_windows::Win32::Graphics::Direct3D11::D3D11_TEX2D_VPIV { MipSlice: 0, ArraySlice: 0 },
                    },
                };
                let mut iv = None;
                if unsafe { vd.CreateVideoProcessorInputView(stex, &*vp_enum, &iv_desc, Some(&mut iv)) }.is_err() {
                    return false;
                }
                let Some(iv) = iv else { return false };

                // Aspect-preserving destination rect (centered letterbox if window aspect changed).
                let (dx0, dy0, dx1, dy1) = fit_dest(sw, sh, self.out_w, self.out_h);
                let src_rect = WinRect { left: 0, top: 0, right: sw as i32, bottom: sh as i32 };
                let dst_rect = WinRect { left: dx0, top: dy0, right: dx1, bottom: dy1 };
                let full_rect = WinRect { left: 0, top: 0, right: self.out_w as i32, bottom: self.out_h as i32 };

                let (tex, ov) = &ring[*ring_idx];
                *ring_idx = (*ring_idx + 1) % ring.len();

                let streams = [D3D11_VIDEO_PROCESSOR_STREAM {
                    Enable: true.into(),
                    pInputSurface: core::mem::ManuallyDrop::new(Some(iv)),
                    ..Default::default()
                }];
                unsafe {
                    vc.VideoProcessorSetStreamSourceRect(&*vp, 0, true, Some(&src_rect));
                    vc.VideoProcessorSetStreamDestRect(&*vp, 0, true, Some(&dst_rect));
                    vc.VideoProcessorSetOutputTargetRect(&*vp, true, Some(&full_rect));
                    let ok = vc.VideoProcessorBlt(&*vp, ov, 0, &streams).is_ok();
                    let [stream] = streams;
                    let _ = core::mem::ManuallyDrop::into_inner(stream.pInputSurface);
                    if !ok {
                        return false;
                    }
                }
                tex.clone()
            }
            Pipeline::Bgra { ring, ring_idx } => {
                let tex = &ring[*ring_idx];
                *ring_idx = (*ring_idx + 1) % ring.len();
                let copy_w = sw.min(self.out_w);
                let copy_h = sh.min(self.out_h);
                let src_box = D3D11_BOX {
                    left: sx0 as u32,
                    top: sy0 as u32,
                    front: 0,
                    right: sx0 as u32 + copy_w,
                    bottom: sy0 as u32 + copy_h,
                    back: 1,
                };
                unsafe {
                    gpu.context.CopySubresourceRegion(tex, 0, 0, 0, 0, src, 0, Some(&src_box));
                }
                tex.clone()
            }
        };

        if !self.submit_texture(&out_tex, pts, dur) {
            return false;
        }
        self.last_out = Some(out_tex);
        self.last_pts = pts;
        self.frames += 1;
        true
    }

    fn submit_texture(&self, tex: &ID3D11Texture2D, pts: i64, dur: i64) -> bool {
        let Some(writer) = self.writer.as_ref() else {
            return false;
        };
        let Ok(buf) = (unsafe { MFCreateDXGISurfaceBuffer(&ID3D11Texture2D::IID, tex, 0, false) }) else {
            return false;
        };
        if let Ok(buf2d) = buf.cast::<IMF2DBuffer>()
            && let Ok(len) = unsafe { buf2d.GetContiguousLength() }
        {
            let _ = unsafe { buf.SetCurrentLength(len) };
        }
        let Ok(sample) = (unsafe { MFCreateSample() }) else {
            return false;
        };
        unsafe {
            sample.AddBuffer(&buf).is_ok()
                && sample.SetSampleTime(pts).is_ok()
                && sample.SetSampleDuration(dur).is_ok()
                && writer.WriteSample(self.stream_idx, &sample).is_ok()
        }
    }

    /// Live written size estimate in bytes (reads the real growing file size on disk, falling back
    /// to Media Foundation's processed byte counter before the first disk flush).
    pub fn bytes_written(&self) -> u64 {
        let mut data = MaybeUninit::<WIN32_FILE_ATTRIBUTE_DATA>::zeroed();
        if unsafe { GetFileAttributesExW(self.wpath.as_ptr(), GetFileExInfoStandard, data.as_mut_ptr().cast()) } != 0 {
            let d = unsafe { data.assume_init() };
            let disk = ((d.nFileSizeHigh as u64) << 32) | (d.nFileSizeLow as u64);
            if disk > 0 {
                return disk;
            }
        }
        if let Some(writer) = self.writer.as_ref() {
            let mut stats = MF_SINK_WRITER_STATISTICS { cb: size_of::<MF_SINK_WRITER_STATISTICS>() as u32, ..Default::default() };
            if unsafe { writer.GetStatistics(self.stream_idx, &mut stats) }.is_ok() {
                return stats.qwByteCountProcessed;
            }
        }
        0
    }

    pub fn frame_count(&self) -> u64 {
        self.frames
    }

    /// Finalizes the MP4 container, closes the sink writer, un-hides and atomically renames the
    /// staging file to `Videos\SnipSnap\SnipSnap_YYYY-MM-DD_HH-MM-SS.mp4`.
    pub fn finalize(mut self, elapsed_100ns: i64) -> Option<(String, u64)> {
        if self.frames == 0 {
            return None; // Drop deletes the staging file.
        }
        if elapsed_100ns > self.last_pts + self.frame_dur * 2
            && let Some(last) = self.last_out.as_ref()
        {
            let _ = self.submit_texture(last, elapsed_100ns, self.frame_dur);
        }
        let writer = self.writer.take()?;
        let ok = unsafe { writer.Finalize() }.is_ok();
        drop(writer);
        if !ok {
            return None;
        }
        let size = self.bytes_written();
        let final_path = make_output_path()?;
        let final_w = to_wide(&final_path);
        let _ = unsafe { SetFileAttributesW(self.wpath.as_ptr(), 0x80) }; // FILE_ATTRIBUTE_NORMAL
        if !move_with_retry(&self.wpath, &final_w) {
            return None;
        }
        self.committed = true;
        Some((final_path, size))
    }
}

/// Centers a `(sw, sh)` frame inside `(ow, oh)` preserving aspect ratio (letterbox on resize).
fn fit_dest(sw: u32, sh: u32, ow: u32, oh: u32) -> (i32, i32, i32, i32) {
    if sw == 0 || sh == 0 {
        return (0, 0, ow as i32, oh as i32);
    }
    let lhs = (sw as u64) * (oh as u64);
    let rhs = (ow as u64) * (sh as u64);
    let (dw, dh) = if lhs == rhs {
        (ow, oh)
    } else if lhs > rhs {
        let h = ((ow as u64 * sh as u64 + sw as u64 / 2) / sw as u64).min(oh as u64) as u32;
        (ow, h.max(2))
    } else {
        let w = ((oh as u64 * sw as u64 + sh as u64 / 2) / sh as u64).min(ow as u64) as u32;
        (w.max(2), oh)
    };
    let x = ((ow - dw) / 2) as i32;
    let y = ((oh - dh) / 2) as i32;
    (x, y, x + dw as i32, y + dh as i32)
}
