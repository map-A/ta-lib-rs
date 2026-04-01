//! CDLPIERCING — Piercing Pattern
//! Bullish reversal: bearish candle + bullish that opens below prev low, closes above midpoint.
use super::helpers::*;

pub fn cdlpiercing(open: &[f64], _high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD;
    let lookback = period + 1;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;
        let rb0 = real_body(open[i-1], close[i-1]);
        let rb1 = real_body(open[i], close[i]);
        let mid0 = (open[i-1] + close[i-1]) / 2.0;

        let is_pattern =
            candle_color(open[i-1], close[i-1]) == -1 && rb0 > avg * BODY_LONG_FACTOR &&
            candle_color(open[i], close[i]) == 1 && rb1 > avg * BODY_LONG_FACTOR &&
            open[i] < low[i-1] &&
            close[i] > mid0 &&
            close[i] < open[i-1];

        if is_pattern { out[i] = 100.0; }

        body_sum += real_body(open[i-1], close[i-1]);
        body_sum -= real_body(open[i-1-period], close[i-1-period]);
    }
    out
}
