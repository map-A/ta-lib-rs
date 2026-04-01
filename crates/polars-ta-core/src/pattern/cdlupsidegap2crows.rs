//! CDLUPSIDEGAP2CROWS — Upside Gap Two Crows
//! Bearish reversal: bullish long body + bearish gap-up + bearish that opens above first bearish.
use super::helpers::*;

pub fn cdlupsidegap2crows(open: &[f64], _high: &[f64], _low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD;
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;

        let is_pattern =
            // First: long bullish
            candle_color(open[i-2], close[i-2]) == 1 &&
            real_body(open[i-2], close[i-2]) > avg * BODY_LONG_FACTOR &&
            // Second: bearish with gap up (opens above first close)
            candle_color(open[i-1], close[i-1]) == -1 &&
            open[i-1] > close[i-2] &&
            // Third: bearish, opens above second open, closes below second close but above first close
            candle_color(open[i], close[i]) == -1 &&
            open[i] > open[i-1] &&
            close[i] > close[i-2] &&
            close[i] < close[i-1];

        if is_pattern { out[i] = -100.0; }

        body_sum += real_body(open[i-2], close[i-2]);
        body_sum -= real_body(open[i-2-period], close[i-2-period]);
    }
    out
}
