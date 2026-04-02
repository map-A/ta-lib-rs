//! Shared Hilbert Transform engine matching ta-lib's exact algorithm.
//!
//! Implements ta-lib's DO_HILBERT_TRANSFORM macro with even/odd bar separation,
//! WMA warmup, DFT-based DC Phase, and trendline computation.

/// Lookback for DCPeriod and Phasor (HT starts at bar 12).
pub const HT_LOOKBACK_SMALL: usize = 32;
/// Lookback for DCPhase, Sine, Trendline, Trendmode (HT starts at bar 37).
pub const HT_LOOKBACK_LARGE: usize = 63;

const A: f64 = 0.0962;
const B: f64 = 0.5769;
const SMOOTH_PRICE_SIZE: usize = 50;

/// Per-bar output from the HT engine.
pub struct HtBarResult {
    pub smooth_period: f64,
    pub i1: f64,
    pub q1: f64,
    pub dc_phase: f64,
    pub prev_dc_phase: f64,
    pub sine: f64,
    pub lead_sine: f64,
    pub trendline: f64,
    pub smooth_price_cur: f64,
}

/// One Hilbert Transform stage (detrender, Q1, jI, or jQ).
#[derive(Default, Clone)]
struct HilbertBuf {
    odd: [f64; 3],
    even: [f64; 3],
    prev_odd: f64,
    prev_even: f64,
    prev_input_odd: f64,
    prev_input_even: f64,
}

impl HilbertBuf {
    fn step(&mut self, input: f64, hilbert_idx: usize, is_even: bool, adj: f64) -> f64 {
        let ht = A * input;
        if is_even {
            let out = -self.even[hilbert_idx] + ht - self.prev_even;
            self.even[hilbert_idx] = ht;
            self.prev_even = B * self.prev_input_even;
            let out = out + self.prev_even;
            self.prev_input_even = input;
            out * adj
        } else {
            let out = -self.odd[hilbert_idx] + ht - self.prev_odd;
            self.odd[hilbert_idx] = ht;
            self.prev_odd = B * self.prev_input_odd;
            let out = out + self.prev_odd;
            self.prev_input_odd = input;
            out * adj
        }
    }
}

/// Run the HT engine over all bars of `close`.
///
/// `warmup_i = 9` for lookback-32 indicators (DCPeriod, Phasor).
/// `warmup_i = 34` for lookback-63 indicators (DCPhase, Sine, Trendline, Trendmode).
///
/// Returns a `Vec<Option<HtBarResult>>` of length `close.len()`.
/// Entries are `None` before the HT main loop starts; `Some` from the first HT bar onward.
pub fn run_ht_engine(close: &[f64], warmup_i: usize) -> Vec<Option<HtBarResult>> {
    let n = close.len();
    let mut results: Vec<Option<HtBarResult>> = (0..n).map(|_| None).collect();
    if n < 4 {
        return results;
    }

    let rad2deg = 45.0_f64 / (1.0_f64.atan());
    let deg2rad = 1.0_f64 / rad2deg;
    let c2r360 = (1.0_f64.atan()) * 8.0_f64; // 2*pi

    // --- WMA initialisation (3-bar unrolled, mirroring ta-lib) ---
    let mut trailing_wma_idx: usize = 0;
    let mut today: usize = 0;

    let c0 = close[today];
    today += 1;
    let mut pws = c0;
    let mut pw_sum = c0;
    let c1 = close[today];
    today += 1;
    pws += c1;
    pw_sum += c1 * 2.0;
    let c2 = close[today];
    today += 1;
    pws += c2;
    pw_sum += c2 * 3.0;
    let mut trailing_wma_val = 0.0_f64;

    // Inline WMA step to avoid borrow issues with close
    macro_rules! wma_step {
        ($new_price:expr) => {{
            pws += $new_price;
            pws -= trailing_wma_val;
            pw_sum += $new_price * 4.0;
            trailing_wma_val = close[trailing_wma_idx];
            trailing_wma_idx += 1;
            let sv = pw_sum * 0.1;
            pw_sum -= pws;
            sv
        }};
    }

    // WMA warmup (9 or 34 iterations)
    for _ in 0..warmup_i {
        let _ = wma_step!(close[today]);
        today += 1;
    }
    // `today` is now the start of the HT main loop

    // --- Init HT variables ---
    let mut hilbert_idx: usize = 0;
    let mut det = HilbertBuf::default();
    let mut q1_buf = HilbertBuf::default();
    let mut ji_buf = HilbertBuf::default();
    let mut jq_buf = HilbertBuf::default();

    let mut i1_odd_p2 = 0.0_f64;
    let mut i1_odd_p3 = 0.0_f64;
    let mut i1_even_p2 = 0.0_f64;
    let mut i1_even_p3 = 0.0_f64;
    let mut prev_i2 = 0.0_f64;
    let mut prev_q2 = 0.0_f64;
    let mut re = 0.0_f64;
    let mut im = 0.0_f64;
    let mut period = 0.0_f64;
    let mut smooth_period = 0.0_f64;

    let mut smooth_price = [0.0_f64; SMOOTH_PRICE_SIZE];
    let mut sp_idx: usize = 0;
    let mut dc_phase = 0.0_f64;
    let mut it1 = 0.0_f64;
    let mut it2 = 0.0_f64;
    let mut it3 = 0.0_f64;

    while today < n {
        let adj = 0.075 * period + 0.54;
        let sv = wma_step!(close[today]);
        smooth_price[sp_idx] = sv;
        let is_even = today.is_multiple_of(2);

        let detrender_val = det.step(sv, hilbert_idx, is_even, adj);
        let q1_val = q1_buf.step(detrender_val, hilbert_idx, is_even, adj);

        let ji_input = if is_even { i1_even_p3 } else { i1_odd_p3 };
        let ji_val = ji_buf.step(ji_input, hilbert_idx, is_even, adj);
        let jq_val = jq_buf.step(q1_val, hilbert_idx, is_even, adj);

        let (q2, i2, i1_out) = if is_even {
            hilbert_idx = (hilbert_idx + 1) % 3;
            let q2 = 0.2 * (q1_val + ji_val) + 0.8 * prev_q2;
            let i2 = 0.2 * (i1_even_p3 - jq_val) + 0.8 * prev_i2;
            i1_odd_p3 = i1_odd_p2;
            i1_odd_p2 = detrender_val;
            (q2, i2, i1_even_p3)
        } else {
            let q2 = 0.2 * (q1_val + ji_val) + 0.8 * prev_q2;
            let i2 = 0.2 * (i1_odd_p3 - jq_val) + 0.8 * prev_i2;
            i1_even_p3 = i1_even_p2;
            i1_even_p2 = detrender_val;
            (q2, i2, i1_odd_p3)
        };

        re = 0.2 * (i2 * prev_i2 + q2 * prev_q2) + 0.8 * re;
        im = 0.2 * (i2 * prev_q2 - q2 * prev_i2) + 0.8 * im;
        prev_q2 = q2;
        prev_i2 = i2;

        let prev_period = period;
        if im != 0.0 && re != 0.0 {
            period = 360.0 / ((im / re).atan() * rad2deg);
        }
        // C-style clamp: comparisons are false for NaN, so NaN propagates correctly
        if period > 1.5 * prev_period {
            period = 1.5 * prev_period;
        }
        if period < 0.67 * prev_period {
            period = 0.67 * prev_period;
        }
        if period < 6.0 {
            period = 6.0;
        } else if period > 50.0 {
            period = 50.0;
        }
        period = 0.2 * period + 0.8 * prev_period;
        smooth_period = 0.33 * period + 0.67 * smooth_period;

        // DCPhase via DFT over smooth_price circular buffer
        let prev_dc_phase = dc_phase;
        let dcp_int = (smooth_period + 0.5) as usize;
        let mut real_p = 0.0_f64;
        let mut imag_p = 0.0_f64;
        let mut idx = sp_idx;
        for i in 0..dcp_int {
            let tr = c2r360 * (i as f64) / (dcp_int as f64);
            real_p += tr.sin() * smooth_price[idx];
            imag_p += tr.cos() * smooth_price[idx];
            idx = if idx == 0 {
                SMOOTH_PRICE_SIZE - 1
            } else {
                idx - 1
            };
        }

        let tmp = imag_p.abs();
        if tmp > 0.0 {
            dc_phase = (real_p / imag_p).atan() * rad2deg;
        } else if tmp <= 0.01 {
            if real_p < 0.0 {
                dc_phase -= 90.0;
            } else if real_p > 0.0 {
                dc_phase += 90.0;
            }
        }
        dc_phase += 90.0;
        if smooth_period != 0.0 {
            dc_phase += 360.0 / smooth_period;
        }
        if imag_p < 0.0 {
            dc_phase += 180.0;
        }
        if dc_phase > 315.0 {
            dc_phase -= 360.0;
        }

        let sine = (dc_phase * deg2rad).sin();
        let lead_sine = ((dc_phase + 45.0) * deg2rad).sin();

        // Trendline: average of last dcp_int raw close bars, smoothed with 4-tap WMA
        let dcp_int2 = (smooth_period + 0.5) as usize;
        let mut avg = 0.0_f64;
        let mut idx2 = today;
        for _ in 0..dcp_int2 {
            avg += close[idx2];
            idx2 = idx2.saturating_sub(1);
        }
        if dcp_int2 > 0 {
            avg /= dcp_int2 as f64;
        }
        let trendline = (4.0 * avg + 3.0 * it1 + 2.0 * it2 + it3) / 10.0;
        it3 = it2;
        it2 = it1;
        it1 = avg;

        results[today] = Some(HtBarResult {
            smooth_period,
            i1: i1_out,
            q1: q1_val,
            dc_phase,
            prev_dc_phase,
            sine,
            lead_sine,
            trendline,
            smooth_price_cur: sv,
        });

        // CIRCBUF_NEXT for smooth_price
        sp_idx = (sp_idx + 1) % SMOOTH_PRICE_SIZE;
        today += 1;
    }

    results
}
