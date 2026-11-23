//! Windows.Graphics.Capture session driving `VideoEncoder` from a `HWND` or `HMONITOR`.
use crate::encode::VideoEncoder;
use crate::gpu::GpuContext;
use crate::ipc::{Fps, Quality, RecKind, RecTarget, WM_APP_REC_ENDED, output_size};
use alloc::string::String;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use pk::sync::Mutex;
use windows::Foundation::TypedEventHandler;
use windows::Graphics::Capture::{Direct3D11CaptureFramePool, GraphicsCaptureItem, GraphicsCaptureSession};
use windows::Graphics::DirectX::DirectXPixelFormat;
use windows::Graphics::SizeInt32;
use windows::Win32::Foundation::HWND as WinHwnd;
use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;
use windows::Win32::Graphics::Gdi::HMONITOR as WinHmon;
use windows::Win32::System::WinRT::Direct3D11::IDirect3DDxgiInterfaceAccess;
use windows::Win32::System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop;
use windows_core::Interface;
use windows_sys::Win32::Foundation::{HWND, RECT};
use windows_sys::Win32::Graphics::Gdi::{GetMonitorInfoW, MONITORINFO};
use windows_sys::Win32::UI::WindowsAndMessaging::PostMessageW;

struct Inner {
    gpu: GpuContext,
    encoder: Option<VideoEncoder>,
    kind: RecKind,
    /// Crop rect relative to the monitor texture (for `RecKind::Rect`).
    crop: (i32, i32, i32, i32),
    src_w: u32,
    src_h: u32,
    last_size: SizeInt32,
}

pub struct CaptureSession {
    item: GraphicsCaptureItem,
    pool: Direct3D11CaptureFramePool,
    session: GraphicsCaptureSession,
    inner: Arc<Mutex<Inner>>,
    started: Arc<AtomicBool>,
    stopped: Arc<AtomicBool>,
    bytes: Arc<AtomicU64>,
    quality: Quality,
    fps: Fps,
}

// WinRT capture objects created in MTA (`CreateFreeThreaded`) and serialized via UI / worker handoff.
unsafe impl Send for CaptureSession {}

impl CaptureSession {
    /// Pre-builds the entire WGC + D3D11 Video Processor + Media Foundation H.264 pipeline during
    /// Armed dead time without calling `StartCapture()` yet, so `begin()` fires instantaneously.
    pub fn prepare(gpu: GpuContext, target: RecTarget, quality: Quality, fps: Fps, notify_hwnd: HWND, gen_id: u32) -> Option<Self> {
        let interop = windows_core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>().ok()?;
        let kind = target.kind();

        let item: GraphicsCaptureItem = match kind {
            RecKind::Window => unsafe { interop.CreateForWindow(WinHwnd(target.hwnd as *mut _)) }.ok()?,
            RecKind::Rect | RecKind::Full => unsafe { interop.CreateForMonitor(WinHmon(target.monitor as *mut _)) }.ok()?,
        };

        let item_size = item.Size().ok()?;
        if item_size.Width <= 0 || item_size.Height <= 0 {
            return None;
        }

        // Compute crop rect inside the WGC surface and the source dimensions for scaling.
        let (crop, src_w, src_h) = match kind {
            RecKind::Rect => {
                let mut mi: MONITORINFO = unsafe { core::mem::zeroed() };
                mi.cbSize = size_of::<MONITORINFO>() as u32;
                let mon_r = if unsafe { GetMonitorInfoW(target.monitor as *mut _, &mut mi) } != 0 {
                    mi.rcMonitor
                } else {
                    RECT { left: 0, top: 0, right: item_size.Width, bottom: item_size.Height }
                };
                let x0 = (target.rect.left - mon_r.left).clamp(0, item_size.Width - 1);
                let y0 = (target.rect.top - mon_r.top).clamp(0, item_size.Height - 1);
                let x1 = (target.rect.right - mon_r.left).clamp(x0 + 1, item_size.Width);
                let y1 = (target.rect.bottom - mon_r.top).clamp(y0 + 1, item_size.Height);
                ((x0, y0, x1, y1), (x1 - x0) as u32, (y1 - y0) as u32)
            }
            RecKind::Window | RecKind::Full => ((0, 0, item_size.Width, item_size.Height), item_size.Width as u32, item_size.Height as u32),
        };

        let (out_w, out_h) = output_size(src_w, src_h, quality);
        let encoder = VideoEncoder::new_staged(&gpu, src_w, src_h, out_w, out_h, quality, fps, gen_id)?;

        let pool =
            Direct3D11CaptureFramePool::CreateFreeThreaded(&gpu.winrt_device, DirectXPixelFormat::B8G8R8A8UIntNormalized, 2, item_size)
                .ok()?;
        let session = pool.CreateCaptureSession(&item).ok()?;
        // Hide Windows' yellow capture border on Win11 (SnipSnap draws its own 2px red border).
        let _ = session.SetIsBorderRequired(false);
        let _ = session.SetIsCursorCaptureEnabled(true);

        let started = Arc::new(AtomicBool::new(false));
        let stopped = Arc::new(AtomicBool::new(false));
        let bytes = Arc::new(AtomicU64::new(0));
        let inner = Arc::new(Mutex::new(Inner { gpu, encoder: Some(encoder), kind, crop, src_w, src_h, last_size: item_size }));

        // FrameArrived callback (runs on WinRT free-threaded pool).
        {
            let inner_cb = Arc::clone(&inner);
            let started_cb = Arc::clone(&started);
            let stopped_cb = Arc::clone(&stopped);
            let bytes_cb = Arc::clone(&bytes);
            let handler = TypedEventHandler::<Direct3D11CaptureFramePool, windows_core::IInspectable>::new(move |pool_ref, _| {
                if !started_cb.load(Ordering::Acquire) || stopped_cb.load(Ordering::Acquire) {
                    return Ok(());
                }
                let Some(pool_ref) = pool_ref.as_ref() else {
                    return Ok(());
                };
                let Ok(frame) = pool_ref.TryGetNextFrame() else {
                    return Ok(());
                };
                let Ok(content_size) = frame.ContentSize() else {
                    return Ok(());
                };
                let Ok(pts) = frame.SystemRelativeTime() else {
                    return Ok(());
                };
                let Ok(surface) = frame.Surface() else {
                    return Ok(());
                };
                let Ok(access) = surface.cast::<IDirect3DDxgiInterfaceAccess>() else {
                    return Ok(());
                };
                let Ok(tex) = (unsafe { access.GetInterface::<ID3D11Texture2D>() }) else {
                    return Ok(());
                };

                let mut guard = inner_cb.lock();
                if stopped_cb.load(Ordering::Acquire) {
                    return Ok(());
                }
                // Resize frame pool if the captured window or display changed size.
                if content_size.Width != guard.last_size.Width || content_size.Height != guard.last_size.Height {
                    if content_size.Width > 0 && content_size.Height > 0 {
                        guard.last_size = content_size;
                        let _ = pool_ref.Recreate(&guard.gpu.winrt_device, DirectXPixelFormat::B8G8R8A8UIntNormalized, 2, content_size);
                    }
                }
                let crop = match guard.kind {
                    RecKind::Rect => guard.crop,
                    RecKind::Window | RecKind::Full => (0, 0, content_size.Width, content_size.Height),
                };
                let Inner { gpu, encoder, .. } = &mut *guard;
                if let Some(enc) = encoder.as_mut() {
                    let _ = enc.write_frame(gpu, &tex, crop, pts.Duration);
                    // Refresh live file size every ~6 frames (~100ms at 60fps).
                    if enc.frame_count() % 6 == 1 {
                        bytes_cb.store(enc.bytes_written(), Ordering::Relaxed);
                    }
                }
                Ok(())
            });
            pool.FrameArrived(&handler).ok()?;
        }

        // Closed callback (fires if captured window is closed while armed or mid-recording).
        {
            let notify = notify_hwnd as usize;
            let closed_cb = TypedEventHandler::<GraphicsCaptureItem, windows_core::IInspectable>::new(move |_, _| {
                if notify != 0 {
                    unsafe { PostMessageW(notify as HWND, WM_APP_REC_ENDED, 0, 0) };
                }
                Ok(())
            });
            let _ = item.Closed(&closed_cb);
        }

        Some(Self { item, pool, session, inner, started, stopped, bytes, quality, fps })
    }

    pub fn quality(&self) -> Quality {
        self.quality
    }

    pub fn fps(&self) -> Fps {
        self.fps
    }

    /// Swaps the pre-warmed `VideoEncoder` to a new resolution / FPS preset while Armed, reusing
    /// the hot GPU device and WGC frame pool. Drops the old staging encoder first to free its
    /// NVENC slot and remove its 0-byte staging file.
    pub fn reconfigure(&mut self, quality: Quality, fps: Fps, gen_id: u32) -> bool {
        if self.started.load(Ordering::Acquire) || (self.quality == quality && self.fps == fps) {
            return true;
        }
        let mut guard = self.inner.lock();
        guard.encoder = None;
        let (src_w, src_h) = (guard.src_w, guard.src_h);
        let (out_w, out_h) = output_size(src_w, src_h, quality);
        let Some(enc) = VideoEncoder::new_staged(&guard.gpu, src_w, src_h, out_w, out_h, quality, fps, gen_id) else {
            return false;
        };
        guard.encoder = Some(enc);
        self.quality = quality;
        self.fps = fps;
        true
    }

    /// Starts frame delivery on the already-prepared capture session (instantaneous).
    pub fn begin(&mut self) -> bool {
        self.started.store(true, Ordering::Release);
        if self.session.StartCapture().is_err() {
            self.started.store(false, Ordering::Release);
            return false;
        }
        true
    }

    pub fn live_bytes(&self) -> u64 {
        self.bytes.load(Ordering::Relaxed)
    }

    /// Halts frame capture immediately (~0.2 ms) so no subsequent UI (such as the 150 ms white
    /// flash) can ever appear in the video while `stop()` finalizes the MP4 in the background.
    pub fn halt_capture(&self) {
        if !self.stopped.swap(true, Ordering::AcqRel) {
            let _ = self.session.Close();
            let _ = self.pool.Close();
        }
    }

    /// Stops capture, finalizes the MP4 file if recording started (else deletes the 0-byte staging
    /// file), and drops all WGC + D3D11 + Media Foundation handles so GPU memory frees immediately.
    pub fn stop(self, elapsed_100ns: i64) -> Option<(String, u64)> {
        let was_started = self.started.load(Ordering::Acquire);
        self.halt_capture();
        drop(self.session);
        drop(self.pool);
        drop(self.item);

        let result = {
            let mut guard = self.inner.lock();
            if was_started {
                guard.encoder.take().and_then(|enc| enc.finalize(elapsed_100ns))
            } else {
                guard.encoder = None;
                None
            }
        };
        // `self.inner` drops here (along with `GpuContext` -> `MFShutdown` + D3D11 release).
        drop(self.inner);
        result
    }
}
