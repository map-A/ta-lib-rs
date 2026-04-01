//! CDL3STARSINSOUTH — Three Stars in the South
//! Bullish reversal: three bearish candles with progressively smaller bodies and shadows.
use super::helpers::*;

pub fn cdl3starsinsouth(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = SHADOW_VERY_SHORT_PERIOD.max(BODY_LONG_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut body_sum: f64  = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut shadow_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_body   = body_sum   / period as f64;
        let avg_shadow = shadow_sum / period as f64;

        let is_pattern =
            // All 3 bearish
            candle_color(open[i-2], close[i-2]) == -1 &&
            candle_color(open[i-1], close[i-1]) == -1 &&
            candle_color(open[i],   close[i])   == -1 &&
            // 1st has long body
            real_body(open[i-2], close[i-2]) > avg_body * BODY_LONG_FACTOR &&
            // 1st has long lower shadow
            lower_shadow(open[i-2], low[i-2], close[i-2]) > avg_shadow * SHADOW_VERY_SHORT_FACTOR &&
            // 2nd: smaller body, high < 1st open, low above 1st low
            real_body(open[i-1], close[i-1]) < real_body(open[i-2], close[i-2]) &&
            high[i-1] < open[i-2] &&
            low[i-1]  > low[i-2] &&
            lower_shadow(open[i-1], low[i-1], close[i-1]) > avg_shadow * SHADOW_VERY_SHORT_FACTOR &&
            // 3rd: small body (marubozu-like), opens at high == open, closes at low == close
            real_body(open[i], close[i]) < real_body(open[i-1], close[i-1]) &&
            high[i] <= open[i] &&
            low[i]  >= close[i];

        if is_pattern { out[i] = 100.0; }

        body_sum   += real_body(open[i-2], close[i-2]);
        body_sum   -= real_body(open[i-2-period], close[i-2-period]);
        shadow_sum += hl_range(high[i-2], low[i-2]);
        shadow_sum -= hl_range(high[i-2-period], low[i-2-period]);
    }
    out
}
