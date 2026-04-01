//! CDLHOMINGPIGEON — Homing Pigeon
//! Bullish reversal: 2 bearish candles where second is contained within first.
use super::helpers::*;

pub fn cdlhomingpigeon(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD;
    let lookback = period + 1;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;
        let rb0 = real_body(open[i-1], close[i-1]);
        let rb1 = real_body(open[i], close[i]);
        let is_pattern =
            candle_color(open[i-1], close[i-1]) == -1 &&
            rb0 > avg * BODY_SHORT_FACTOR &&
            candle_color(open[i], close[i]) == -1 &&
            rb1 > avg * BODY_SHORT_FACTOR &&
            open[i] < open[i-1] &&
            close[i] > close[i-1];

        if is_pattern { out[i] = 100.0; }

        body_sum += real_body(open[i-1], close[i-1]);
        body_sum -= real_body(open[i-1-period], close[i-1-period]);
    }
    out
}
