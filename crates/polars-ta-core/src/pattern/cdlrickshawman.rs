//! CDLRICKSHAWMAN — Rickshaw Man
//! Doji with long upper/lower shadows and body near the midpoint.
use super::helpers::*;

pub fn cdlrickshawman(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    // BodyDoji: period=10, hl_range, trailing=0
    // ShadowLong: period=0 → no rolling sum, compare directly against real body
    // Near: period=5, hl_range, trailing=5
    let doji_period = BODY_DOJI_PERIOD;   // 10
    let near_period = NEAR_PERIOD;        // 5
    let lookback = doji_period.max(near_period);  // 10
    if n <= lookback { return out; }

    let mut doji_sum: f64 = (0..doji_period).map(|j| hl_range(high[j], low[j])).sum();
    // Near trailing starts at startIdx - near_period = 10 - 5 = 5
    let mut near_sum: f64 = ((lookback - near_period)..lookback)
        .map(|j| hl_range(high[j], low[j])).sum();
    let mut near_trailing = lookback - near_period;  // 5

    for i in lookback..n {
        let avg_doji = doji_sum / doji_period as f64;
        let avg_near = near_sum / near_period as f64 * NEAR_FACTOR;

        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);
        let mid = (high[i] + low[i]) / 2.0;

        // ShadowLong period=0: compare directly against real body * factor(1.0) = rb
        let is_doji = rb <= avg_doji * BODY_DOJI_FACTOR;
        let long_shadows = us > rb && ls > rb;
        let near_mid = open[i].min(close[i]) <= mid + avg_near
            && open[i].max(close[i]) >= mid - avg_near;

        if is_doji && long_shadows && near_mid { out[i] = 100.0; }

        doji_sum += hl_range(high[i], low[i]);
        doji_sum -= hl_range(high[i-doji_period], low[i-doji_period]);
        near_sum += hl_range(high[i], low[i]);
        near_sum -= hl_range(high[near_trailing], low[near_trailing]);
        near_trailing += 1;
    }
    out
}
