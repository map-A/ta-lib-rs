//! CDL2CROWS — Two Crows
//! Bearish reversal: bullish long-body candle followed by 2 bearish candles
//! closing inside the first.
use super::helpers::*;

pub fn cdl2crows(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD;
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;
        let rb0 = real_body(open[i-2], close[i-2]);
        let is_pattern =
            candle_color(open[i-2], close[i-2]) == 1 &&          // candle0 bullish
            rb0 > avg * BODY_LONG_FACTOR &&                       // candle0 long body
            candle_color(open[i-1], close[i-1]) == -1 &&         // candle1 bearish
            open[i-1] > close[i-2] &&                            // candle1 gaps up
            candle_color(open[i], close[i]) == -1 &&             // candle2 bearish
            open[i] < open[i-1] && open[i] > close[i-1] &&      // candle2 opens in candle1 body
            close[i] < close[i-2] && close[i] > open[i-2];      // candle2 closes in candle0 body

        if is_pattern { out[i] = -100.0; }

        body_sum += real_body(open[i-2], close[i-2]);
        body_sum -= real_body(open[i-2-period], close[i-2-period]);
    }
    out
}
