//! Live PC audio loopback capture (WASAPI `AUDCLNT_STREAMFLAGS_LOOPBACK`) into 5 FFT frequency
//! buckets. Runs on its own MTA thread and pauses capture automatically when nothing is playing.
use crate::fft::{BANDS, FFT_SIZE, SpectrumAnalyzer};
use alloc::sync::Arc;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use windows::Win32::Media::Audio::{
    AUDCLNT_BUFFERFLAGS_SILENT, AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_LOOPBACK, IAudioCaptureClient, IAudioClient, IMMDeviceEnumerator,
    MMDeviceEnumerator, WAVEFORMATEXTENSIBLE, eConsole, eRender,
};
use windows::Win32::System::Com::{CLSCTX_ALL, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree};
use windows_core::GUID;
use windows_sys::Win32::System::Threading::Sleep;

const WAVE_FORMAT_IEEE_FLOAT: u16 = 3;
const WAVE_FORMAT_EXTENSIBLE: u16 = 0xFFFE;
const KSDATAFORMAT_SUBTYPE_IEEE_FLOAT: GUID = GUID::from_u128(0x00000003_0000_0010_8000_00aa00389b71);

pub struct AudioMeter {
    active: Arc<AtomicBool>,
    stop: Arc<AtomicBool>,
    bands: Arc<[AtomicU32; BANDS]>,
}

impl AudioMeter {
    pub fn start() -> Self {
        let active = Arc::new(AtomicBool::new(false));
        let stop = Arc::new(AtomicBool::new(false));
        let bands = Arc::new([
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ]);
        let (a, s, b) = (active.clone(), stop.clone(), bands.clone());
        let _ = pk::thread::spawn(move || capture_loop(a, s, b));
        Self { active, stop, bands }
    }

    pub fn set_active(&self, on: bool) {
        self.active.store(on, Ordering::Relaxed);
    }

    pub fn read_bands(&self) -> [f32; BANDS] {
        let mut out = [0.0f32; BANDS];
        for (i, slot) in self.bands.iter().enumerate() {
            out[i] = f32::from_bits(slot.load(Ordering::Relaxed));
        }
        out
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

fn publish(slots: &[AtomicU32; BANDS], vals: [f32; BANDS]) {
    for (i, v) in vals.iter().enumerate() {
        slots[i].store(v.to_bits(), Ordering::Relaxed);
    }
}

fn capture_loop(active: Arc<AtomicBool>, stop: Arc<AtomicBool>, slots: Arc<[AtomicU32; BANDS]>) {
    let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
    let mut analyzer = SpectrumAnalyzer::new();
    let mut ring = [0.0f32; FFT_SIZE];
    let mut write_pos = 0usize;

    while !stop.load(Ordering::Relaxed) {
        if !active.load(Ordering::Relaxed) {
            let vals = analyzer.decay_to_zero();
            publish(&slots, vals);
            unsafe { Sleep(80) };
            continue;
        }

        let Some(mut cap) = open_loopback() else {
            let vals = analyzer.decay_to_zero();
            publish(&slots, vals);
            unsafe { Sleep(500) };
            continue;
        };

        while !stop.load(Ordering::Relaxed) && active.load(Ordering::Relaxed) {
            match cap.drain_into(&mut ring, &mut write_pos) {
                Drain::Samples => {
                    let mut ordered = [0.0f32; FFT_SIZE];
                    let tail = FFT_SIZE - write_pos;
                    ordered[..tail].copy_from_slice(&ring[write_pos..]);
                    ordered[tail..].copy_from_slice(&ring[..write_pos]);
                    let vals = analyzer.process(&ordered, cap.sample_rate);
                    publish(&slots, vals);
                }
                Drain::Idle => {
                    let vals = analyzer.decay_to_zero();
                    publish(&slots, vals);
                }
                Drain::DeviceLost => break,
            }
            unsafe { Sleep(16) };
        }

        let _ = unsafe { cap.client.Stop() };
    }
}

enum Drain {
    Samples,
    Idle,
    DeviceLost,
}

struct Loopback {
    client: IAudioClient,
    capture: IAudioCaptureClient,
    sample_rate: u32,
    channels: usize,
    is_float: bool,
    bits_per_sample: u16,
}

fn open_loopback() -> Option<Loopback> {
    let enumerator: IMMDeviceEnumerator = unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }.ok()?;
    let device = unsafe { enumerator.GetDefaultAudioEndpoint(eRender, eConsole) }.ok()?;
    let client: IAudioClient = unsafe { device.Activate(CLSCTX_ALL, None) }.ok()?;

    let mix_ptr = unsafe { client.GetMixFormat() }.ok()?;
    if mix_ptr.is_null() {
        return None;
    }

    let (sample_rate, channels, is_float, bits_per_sample) = unsafe {
        let wf = &*mix_ptr;
        let is_float = if wf.wFormatTag == WAVE_FORMAT_IEEE_FLOAT {
            true
        } else if wf.wFormatTag == WAVE_FORMAT_EXTENSIBLE as u16 && wf.cbSize >= 22 {
            let ext = &*(mix_ptr as *const WAVEFORMATEXTENSIBLE);
            let sub = core::ptr::addr_of!(ext.SubFormat).read_unaligned();
            sub == KSDATAFORMAT_SUBTYPE_IEEE_FLOAT
        } else {
            false
        };
        (wf.nSamplesPerSec, wf.nChannels.max(1) as usize, is_float, wf.wBitsPerSample)
    };

    // 40 ms shared-mode loopback buffer
    let init = unsafe { client.Initialize(AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_LOOPBACK, 400_000, 0, mix_ptr, None) };
    unsafe { CoTaskMemFree(Some(mix_ptr.cast())) };
    if init.is_err() {
        return None;
    }

    let capture: IAudioCaptureClient = unsafe { client.GetService() }.ok()?;
    if unsafe { client.Start() }.is_err() {
        return None;
    }

    Some(Loopback { client, capture, sample_rate, channels, is_float, bits_per_sample })
}

impl Loopback {
    fn drain_into(&mut self, ring: &mut [f32; FFT_SIZE], pos: &mut usize) -> Drain {
        let mut got_audio = false;
        loop {
            let packet_frames = match unsafe { self.capture.GetNextPacketSize() } {
                Ok(n) => n,
                Err(_) => return Drain::DeviceLost,
            };
            if packet_frames == 0 {
                break;
            }
            let mut data_ptr = null_mut();
            let mut frames = 0u32;
            let mut flags = 0u32;
            if unsafe { self.capture.GetBuffer(&mut data_ptr, &mut frames, &mut flags, None, None) }.is_err() {
                return Drain::DeviceLost;
            }

            let silent = (flags & (AUDCLNT_BUFFERFLAGS_SILENT.0 as u32)) != 0 || data_ptr.is_null();
            if silent {
                for _ in 0..frames {
                    ring[*pos] = 0.0;
                    *pos = (*pos + 1) % FFT_SIZE;
                }
            } else if self.is_float && self.bits_per_sample == 32 {
                let floats = unsafe { core::slice::from_raw_parts(data_ptr as *const f32, (frames as usize) * self.channels) };
                for frame in floats.chunks_exact(self.channels) {
                    let mut sum = 0.0f32;
                    for &s in frame {
                        sum += s;
                    }
                    ring[*pos] = sum / (self.channels as f32);
                    *pos = (*pos + 1) % FFT_SIZE;
                }
                got_audio = true;
            } else if !self.is_float && self.bits_per_sample == 16 {
                let ints = unsafe { core::slice::from_raw_parts(data_ptr as *const i16, (frames as usize) * self.channels) };
                for frame in ints.chunks_exact(self.channels) {
                    let mut sum = 0.0f32;
                    for &s in frame {
                        sum += (s as f32) / 32768.0;
                    }
                    ring[*pos] = sum / (self.channels as f32);
                    *pos = (*pos + 1) % FFT_SIZE;
                }
                got_audio = true;
            }

            let _ = unsafe { self.capture.ReleaseBuffer(frames) };
        }
        if got_audio { Drain::Samples } else { Drain::Idle }
    }
}
