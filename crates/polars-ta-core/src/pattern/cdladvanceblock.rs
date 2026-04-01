//! CDLADVANCEBLOCK — Advance Block
//! Three advancing bullish candles with progressively smaller bodies (bearish signal).
use super::helpers::*;

pub fn cdladvanceblock(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = SHADOW_SHORT_PERIOD.max(SHADOW_LONG_PERIOD).max(BODY_LONG_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut body_sum:   f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut shadow_sum: [f64; 3] = [
        (0..period).map(|j| real_body(open[j], close[j])).sum(),
        (0..period).map(|j| real_body(open[j], close[j])).sum(),
        (0..period).map(|j| real_body(open[j], close[j])).sum(),
    ];

    for i in lookback..n {
        let avg_body = body_sum / period as f64;
        let avg_shad0 = shadow_sum[0] / period as f64;
        let avg_shad1 = shadow_sum[1] / period as f64;
        let avg_shad2 = shadow_sum[2] / period as f64;
        let rb0 = real_body(open[i-2], close[i-2]);
        let rb1 = real_body(open[i-1], close[i-1]);
        let rb2 = real_body(open[i],   close[i]);

        let is_pattern =
            candle_color(open[i-2], close[i-2]) == 1 &&
            candle_color(open[i-1], close[i-1]) == 1 &&
            candle_color(open[i],   close[i])   == 1 &&
            close[i-1] > close[i-2] && close[i] > close[i-1] &&
            open[i-1] > open[i-2] && open[i-1] < close[i-2] &&
            open[i]   > open[i-1] && open[i]   < close[i-1] &&
            // progressively smaller bodies or increasing upper shadows
            rb0 > avg_body * BODY_LONG_FACTOR &&
            (rb1 < rb0 || upper_shadow(open[i-1], high[i-1], close[i-1]) > avg_shad1 * SHADOW_SHORT_FACTOR) &&
            (rb2 < rb1 || upper_shadow(open[i],   high[i],   close[i])   > avg_shad2 * SHADOW_SHORT_FACTOR) &&
            (upper_shadow(open[i-1], high[i-1], close[i-1]) > avg_shad1 * SHADOW_SHORT_FACTOR ||
             upper_shadow(open[i],   high[i],   close[i])   > avg_shad2 * SHADOW_SHORT_FACTOR ||
             upper_shadow(open[i-2], high[i-2], close[i-2]) > avg_shad0 * SHADOW_SHORT_FACTOR);

        if is_pattern { out[i] = -100.0; }

        body_sum += real_body(open[i-2], close[i-2]);
        body_sum -= real_body(open[i-2-period], close[i-2-period]);
        for k in 0..3usize {
            let j = i - 2 + k;
            shadow_sum[k] += real_body(open[j], close[j]);
            shadow_sum[k] -= real_body(open[j - period], close[j - period]);
        }
    }
    out
}
