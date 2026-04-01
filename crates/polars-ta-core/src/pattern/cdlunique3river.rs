//! CDLUNIQUE3RIVER — Unique 3 River
//! Bullish reversal: large bearish + small bearish with new low + small bullish.
use super::helpers::*;

pub fn cdlunique3river(open: &[f64], _high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD.max(BODY_SHORT_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;

        let is_pattern =
            // First: long bearish
            candle_color(open[i-2], close[i-2]) == -1 &&
            real_body(open[i-2], close[i-2]) > avg * BODY_LONG_FACTOR &&
            // Second: bearish with lower low
            candle_color(open[i-1], close[i-1]) == -1 &&
            low[i-1] < low[i-2] &&
            real_body(open[i-1], close[i-1]) < avg * BODY_SHORT_FACTOR &&
            close[i-1] > close[i-2] &&  // hammer-like: closes above prev close
            // Third: small bullish, closes below second open
            candle_color(open[i], close[i]) == 1 &&
            real_body(open[i], close[i]) < avg * BODY_SHORT_FACTOR &&
            close[i] < open[i-1];

        if is_pattern { out[i] = 100.0; }

        body_sum += real_body(open[i-2], close[i-2]);
        body_sum -= real_body(open[i-2-period], close[i-2-period]);
    }
    out
}
