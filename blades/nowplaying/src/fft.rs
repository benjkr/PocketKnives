//! Pure `no_std` 512-point radix-2 FFT and 5 log-spaced perceptual frequency buckets.
use core::f32::consts::PI;

pub const FFT_SIZE: usize = 512;
pub const BANDS: usize = 5;

/// Log-spaced frequency bucket boundaries in Hz:
/// Sub-bass/Bass, Low-mid, Vocal mid, Upper-mid, Treble.
const BAND_EDGES_HZ: [f32; BANDS + 1] = [40.0, 150.0, 420.0, 1200.0, 3600.0, 14000.0];

pub struct SpectrumAnalyzer {
    window: [f32; FFT_SIZE],
    re: [f32; FFT_SIZE],
    im: [f32; FFT_SIZE],
    smoothed: [f32; BANDS],
    auto_peak: f32,
}

impl SpectrumAnalyzer {
    pub fn new() -> Self {
        let mut window = [0.0f32; FFT_SIZE];
        let denom = (FFT_SIZE - 1) as f32;
        for (i, w) in window.iter_mut().enumerate() {
            *w = 0.5 * (1.0 - libm::cosf(2.0 * PI * (i as f32) / denom));
        }
        Self {
            window,
            re: [0.0; FFT_SIZE],
            im: [0.0; FFT_SIZE],
            smoothed: [0.0; BANDS],
            auto_peak: 0.15,
        }
    }

    pub fn decay_to_zero(&mut self) -> [f32; BANDS] {
        for v in &mut self.smoothed {
            *v *= 0.80;
            if *v < 0.005 {
                *v = 0.0;
            }
        }
        self.smoothed
    }

    /// Processes `FFT_SIZE` mono PCM samples at `sample_rate` Hz and returns normalized `[0..1]`
    /// energy across the 5 perceptual frequency buckets.
    pub fn process(&mut self, samples: &[f32; FFT_SIZE], sample_rate: u32) -> [f32; BANDS] {
        for i in 0..FFT_SIZE {
            self.re[i] = samples[i] * self.window[i];
            self.im[i] = 0.0;
        }
        fft_in_place(&mut self.re, &mut self.im);

        let sr = (sample_rate.max(8000)) as f32;
        let bin_hz = sr / (FFT_SIZE as f32);
        let max_bin = FFT_SIZE / 2 - 1;

        let mut raw = [0.0f32; BANDS];
        let mut frame_max = 0.0f32;

        for b in 0..BANDS {
            let lo = ((BAND_EDGES_HZ[b] / bin_hz) as usize).clamp(1, max_bin);
            let hi = ((BAND_EDGES_HZ[b + 1] / bin_hz) as usize).clamp(lo, max_bin);
            let mut sum_sq = 0.0f32;
            let mut count = 0u32;
            for k in lo..=hi {
                let mag2 = self.re[k] * self.re[k] + self.im[k] * self.im[k];
                sum_sq += mag2;
                count += 1;
            }
            let rms = if count > 0 { libm::sqrtf(sum_sq / (count as f32)) / (FFT_SIZE as f32 * 0.25) } else { 0.0 };
            // Perceptual tilt: treble has less raw RMS than bass, boost higher buckets gently
            let tilt = 1.0 + (b as f32) * 0.42;
            let weighted = rms * tilt;
            raw[b] = weighted;
            if weighted > frame_max {
                frame_max = weighted;
            }
        }

        // Smooth AGC peak tracker so quiet & loud songs both dance nicely without pinning
        if frame_max > self.auto_peak {
            self.auto_peak = self.auto_peak * 0.65 + frame_max * 0.35;
        } else {
            self.auto_peak = (self.auto_peak * 0.992).max(0.035);
        }

        for b in 0..BANDS {
            // Silence gate so background noise floor stays flat
            let norm = if raw[b] < 0.0025 {
                0.0
            } else {
                let ratio = (raw[b] / self.auto_peak).clamp(0.0, 1.4);
                // Log compression for natural musical bounce
                (libm::log10f(1.0 + ratio * 9.0) / 1.08).clamp(0.0, 1.0)
            };
            // Fast attack, smooth exponential decay
            let alpha = if norm > self.smoothed[b] { 0.48 } else { 0.18 };
            self.smoothed[b] += (norm - self.smoothed[b]) * alpha;
        }

        self.smoothed
    }
}

/// Iterative Cooley-Tukey radix-2 FFT (N must be power of two).
fn fft_in_place(re: &mut [f32; FFT_SIZE], im: &mut [f32; FFT_SIZE]) {
    let n = FFT_SIZE;
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }

    let mut len = 2usize;
    while len <= n {
        let half = len >> 1;
        let angle = -2.0 * PI / (len as f32);
        let wlen_r = libm::cosf(angle);
        let wlen_i = libm::sinf(angle);
        let mut i = 0usize;
        while i < n {
            let mut wr = 1.0f32;
            let mut wi = 0.0f32;
            for k in 0..half {
                let u_r = re[i + k];
                let u_i = im[i + k];
                let v_r = re[i + k + half] * wr - im[i + k + half] * wi;
                let v_i = re[i + k + half] * wi + im[i + k + half] * wr;
                re[i + k] = u_r + v_r;
                im[i + k] = u_i + v_i;
                re[i + k + half] = u_r - v_r;
                im[i + k + half] = u_i - v_i;
                let next_r = wr * wlen_r - wi * wlen_i;
                wi = wr * wlen_i + wi * wlen_r;
                wr = next_r;
            }
            i += len;
        }
        len <<= 1;
    }
}
