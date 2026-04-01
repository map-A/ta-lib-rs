//! CDLINVERTEDHAMMER — Inverted Hammer
//! Small body at bottom, long upper shadow, tiny/no lower shadow.
use super::helpers::*;

pub fn cdlinvertedhammer(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD.max(SHADOW_LONG_PERIOD).max(SHADOW_VERY_SHORT_PERIOD);
    let lookback = period + 1;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut shadow_long_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut shadow_vshort_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_body = body_sum / period as f64;
        let avg_long = shadow_long_sum / period as f64;
        let avg_vshort = shadow_vshort_sum / period as f64;

        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);

        let is_pattern =
            rb < avg_body * BODY_SHORT_FACTOR &&
            us > avg_long * SHADOW_LONG_FACTOR &&
            ls < avg_vshort * SHADOW_VERY_SHORT_FACTOR &&
            // Star position: body below the previous candle's body
            open[i].max(close[i]) < open[i-1].min(close[i-1]);

        if is_pattern { out[i] = 100.0; }

        body_sum += real_body(open[i], close[i]);
        body_sum -= real_body(open[i-period], close[i-period]);
        shadow_long_sum += real_body(open[i], close[i]);
        shadow_long_sum -= real_body(open[i-period], close[i-period]);
        shadow_vshort_sum += hl_range(high[i], low[i]);
        shadow_vshort_sum -= hl_range(high[i-period], low[i-period]);
    }
    out
}
