//! CDLHIGHWAVE — High-Wave Candle
//!
//! Small body with very long upper and lower shadows.
//!
//! ta-lib candle settings:
//! - BodyShort:     RealBody, period=10, factor=1.0  → body < avg_body (anchor i)
//! - ShadowVeryLong:RealBody, period=0,  factor=2.0  → shadow > 2 * current body (direct, no avg)
use super::helpers::*;

pub fn cdlhighwave(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let lookback = BODY_SHORT_PERIOD; // max(10, 0) = 10
    if n <= lookback {
        return out;
    }

    let mut body_sum: f64 = (0..BODY_SHORT_PERIOD)
        .map(|j| real_body(open[j], close[j]))
        .sum();
    let mut body_trail = 0usize;

    for i in lookback..n {
        let avg_body = body_sum / BODY_SHORT_PERIOD as f64;
        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);

        if rb < avg_body * BODY_SHORT_FACTOR &&
           us > rb * SHADOW_VERY_LONG_FACTOR &&   // period=0: vs current body
           ls > rb * SHADOW_VERY_LONG_FACTOR
        {
            out[i] = if candle_color(open[i], close[i]) == 1 {
                100.0
            } else {
                -100.0
            };
        }

        body_sum += real_body(open[i], close[i]) - real_body(open[body_trail], close[body_trail]);
        body_trail += 1;
    }
    out
}
