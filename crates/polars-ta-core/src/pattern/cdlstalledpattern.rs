//! CDLSTALLEDPATTERN — Stalled Pattern
//! Bearish reversal: 3 bullish candles with the last two showing exhaustion.
use super::helpers::*;

pub fn cdlstalledpattern(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD.max(SHADOW_VERY_SHORT_PERIOD).max(NEAR_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut shadow_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();
    let mut near_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_body = body_sum / period as f64;
        let avg_shadow = shadow_sum / period as f64;
        let avg_near = near_sum / period as f64;

        let is_pattern =
            // Three consecutive bullish candles
            candle_color(open[i-2], close[i-2]) == 1 &&
            candle_color(open[i-1], close[i-1]) == 1 &&
            candle_color(open[i], close[i]) == 1 &&
            // First two are long
            real_body(open[i-2], close[i-2]) > avg_body * BODY_LONG_FACTOR &&
            real_body(open[i-1], close[i-1]) > avg_body * BODY_LONG_FACTOR &&
            // Third is short (stalled)
            real_body(open[i], close[i]) < avg_body * BODY_SHORT_FACTOR &&
            // Second opens within/near first body
            open[i-1] >= close[i-2] - avg_near * NEAR_FACTOR &&
            open[i-1] <= close[i-2] + avg_near * NEAR_FACTOR &&
            // Third has no upper shadow (showed up near high)
            upper_shadow(open[i], high[i], close[i]) < avg_shadow * SHADOW_VERY_SHORT_FACTOR;

        if is_pattern { out[i] = -100.0; }

        body_sum += real_body(open[i-2], close[i-2]);
        body_sum -= real_body(open[i-2-period], close[i-2-period]);
        shadow_sum += hl_range(high[i-2], low[i-2]);
        shadow_sum -= hl_range(high[i-2-period], low[i-2-period]);
        near_sum += hl_range(high[i-2], low[i-2]);
        near_sum -= hl_range(high[i-2-period], low[i-2-period]);
    }
    out
}
