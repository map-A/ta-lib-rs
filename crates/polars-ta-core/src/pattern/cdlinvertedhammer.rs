//! CDLINVERTEDHAMMER — Inverted Hammer
//!
//! Small body at bottom, long upper shadow (> body), very short lower shadow, gap down.
//!
//! ta-lib candle settings:
//! - BodyShort:       RealBody, period=10, factor=1.0  → body < avg (anchor i)
//! - ShadowLong:      RealBody, period=0,  factor=1.0  → upper_shadow > body (direct, no avg)
//! - ShadowVeryShort: HighLow,  period=10, factor=0.1  → lower_shadow < avg_HL * 0.1 (anchor i)
use super::helpers::*;

pub fn cdlinvertedhammer(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let lookback = BODY_SHORT_PERIOD.max(SHADOW_VERY_SHORT_PERIOD) + 1; // 11
    if n <= lookback { return out; }

    // Both anchored at i: TrailingIdx = startIdx-10 = 1, init while i < startIdx → [1..10]
    let mut body_sum: f64 = (1..=BODY_SHORT_PERIOD).map(|j| real_body(open[j], close[j])).sum();
    let mut vshort_sum: f64 = (1..=SHADOW_VERY_SHORT_PERIOD).map(|j| hl_range(high[j], low[j])).sum();
    let mut body_trail = 1usize;
    let mut vshort_trail = 1usize;

    for i in lookback..n {
        let avg_body   = body_sum   / BODY_SHORT_PERIOD  as f64;
        let avg_vshort = vshort_sum / SHADOW_VERY_SHORT_PERIOD as f64;
        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);

        if rb < avg_body * BODY_SHORT_FACTOR &&
           us > rb * SHADOW_LONG_FACTOR &&                               // period=0: vs current body
           ls < avg_vshort * SHADOW_VERY_SHORT_FACTOR &&
           open[i].max(close[i]) < open[i-1].min(close[i-1])           // gap down
        {
            out[i] = 100.0;
        }

        body_sum   += real_body(open[i], close[i]) - real_body(open[body_trail], close[body_trail]);
        body_trail += 1;
        vshort_sum += hl_range(high[i], low[i]) - hl_range(high[vshort_trail], low[vshort_trail]);
        vshort_trail += 1;
    }
    out
}
