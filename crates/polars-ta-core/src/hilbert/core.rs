//! Shared Hilbert Transform state used by all HT indicators.
//!
//! Implements ta-lib's internal `TA_INT_*` Hilbert Transform kernel — a
//! causal IIR approximation of the analytic signal decomposition.

pub(crate) const HT_LOOKBACK: usize = 63;

/// Per-step Hilbert Transform state matching ta-lib's internal variables.
#[derive(Default)]
pub(crate) struct HtState {
    pub period: f64,
    pub prev_period: f64,
    pub re: f64,
    pub im: f64,
    // Hilbert variables (I1, I2, Q1, Q2, jI, jQ)
    pub i1_prev2: f64,
    pub q1_prev2: f64,
    pub smooth_prev3: f64,
    pub smooth_prev2: f64,
    pub detrender_prev2: f64,
    pub q1_prev_smooth: f64,
    pub i1_prev_q1: f64,
    pub ji_prev2: f64,
    pub jq_prev2: f64,
    pub smooth: f64,
    pub detrender: f64,
    pub q1: f64,
    pub i1: f64,
    pub ji: f64,
    pub jq: f64,
    pub q2: f64,
    pub i2: f64,
    pub re_sum: f64,
    pub im_sum: f64,
    pub smooth_period: f64,
    pub dc_period: f64,
    pub phase: f64,
    pub prev_phase: f64,
    pub prev_i2: f64,
    pub prev_q2: f64,
    pub mesa_period: f64,
    pub mesa_period_mult: f64,
    pub smooth_price_buf: [f64; 4],
    pub sp_idx: usize,
}

impl HtState {
    pub fn new() -> Self {
        let mut s = Self::default();
        s.mesa_period = 0.0;
        s.mesa_period_mult = 0.075;
        s
    }
}

/// Run one step of ta-lib's HT kernel; updates `state` in place.
/// Returns (smooth, detrender, q1, i1, ji, jq, q2, i2, re, im, period, phase).
pub(crate) fn ht_step(price: f64, state: &mut HtState) -> (f64, f64) {
    let a = 0.0962_f64;
    let b = 0.5769_f64;
    let rad_per_deg = std::f64::consts::PI / 180.0;

    // Smooth price (4-tap WMA-like)
    state.smooth_price_buf[state.sp_idx % 4] = price;
    state.sp_idx += 1;
    let sp = &state.smooth_price_buf;
    let idx = state.sp_idx;
    let smooth = (4.0 * sp[(idx-1) % 4] + 3.0 * sp[(idx-2) % 4]
        + 2.0 * sp[(idx-3) % 4] + sp[(idx-4) % 4]) / 10.0;

    let detrender = (0.0962 * smooth + 0.5769 * state.smooth_prev2
        - 0.5769 * state.detrender_prev2 - 0.0962 * state.smooth_prev3)
        * (0.075 * state.period + 0.54);

    let q1 = (0.0962 * detrender + 0.5769 * state.detrender_prev2
        - 0.5769 * state.q1_prev_smooth - 0.0962 * state.i1_prev2)
        * (0.075 * state.period + 0.54);

    let i1 = state.detrender_prev2;

    let ji = (a * i1 + b * state.ji_prev2 - b * state.q1_prev2 - a * state.i1_prev2)
        * (0.075 * state.period + 0.54);
    let jq = (a * q1 + b * state.jq_prev2 - b * state.i1_prev_q1 - a * state.q1_prev2)
        * (0.075 * state.period + 0.54);

    let mut i2 = i1 - jq;
    let mut q2 = q1 + ji;
    i2 = 0.2 * i2 + 0.8 * state.prev_i2;
    q2 = 0.2 * q2 + 0.8 * state.prev_q2;

    let re = i2 * state.prev_i2 + q2 * state.prev_q2;
    let im = i2 * state.prev_q2 - q2 * state.prev_i2;
    let re_s = 0.2 * re + 0.8 * state.re_sum;
    let im_s = 0.2 * im + 0.8 * state.im_sum;

    let period = if re_s != 0.0 && im_s != 0.0 {
        let mut p = 360.0 / (im_s.atan2(re_s) / rad_per_deg);
        p = p.min(1.5 * state.prev_period).max(0.67 * state.prev_period);
        p.clamp(6.0, 50.0)
    } else {
        state.prev_period
    };
    let sp = 0.2 * period + 0.8 * state.smooth_period;

    let phase = if i1 != 0.0 {
        (q1 / i1).atan() / rad_per_deg
    } else if q1 > 0.0 { 90.0 } else if q1 < 0.0 { -90.0 } else { 0.0 };

    // Advance state
    state.smooth_prev3 = state.smooth_prev2;
    state.smooth_prev2 = smooth;
    state.detrender_prev2 = detrender;
    state.q1_prev_smooth = state.i1_prev_q1;
    state.i1_prev_q1 = q1;
    state.i1_prev2 = i1;
    state.q1_prev2 = q1;
    state.ji_prev2 = ji;
    state.jq_prev2 = jq;
    state.prev_i2 = i2;
    state.prev_q2 = q2;
    state.re_sum = re_s;
    state.im_sum = im_s;
    state.prev_period = period;
    state.smooth_period = sp;
    state.period = period;
    state.phase = phase;
    state.dc_period = sp;

    (sp, phase)
}
