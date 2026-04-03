//! CDLLONGLEGGEDDOJI — Long Legged Doji
//! Doji body with at least one long shadow (upper OR lower).
use super::helpers::*;

pub fn cdllongleggeddoji(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    // BodyDoji: HighLow, period=10, anchor at i (trailing starts at 0)
    // ShadowLong: RealBody, period=0 → direct comparison, no rolling needed
    let body_period = BODY_DOJI_PERIOD;
    let lookback = body_period;
    if n <= lookback {
        return out;
    }

    let mut body_sum: f64 = (0..body_period).map(|j| hl_range(high[j], low[j])).sum();

    for (body_trailing, i) in (lookback..n).enumerate() {
        let avg_body = body_sum / body_period as f64;
        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);

        // ShadowLong period=0: compare against current real body * factor
        let shadow_long_thresh = rb * SHADOW_LONG_FACTOR;

        // Doji body AND (either shadow is long)
        if rb <= avg_body * BODY_DOJI_FACTOR && (us > shadow_long_thresh || ls > shadow_long_thresh)
        {
            out[i] = 100.0; // always +100
        }

        body_sum += hl_range(high[i], low[i]);
        body_sum -= hl_range(high[body_trailing], low[body_trailing]);
    }
    out
}
