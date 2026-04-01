//! CDL3LINESTRIKE — Three-Line Strike
//! Three candles in one direction, 4th reverses and closes beyond first.
use super::helpers::*;

pub fn cdl3linestrike(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD;
    let lookback = period + 3;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;
        let c3 = candle_color(open[i-3], close[i-3]);
        let c2 = candle_color(open[i-2], close[i-2]);
        let c1 = candle_color(open[i-1], close[i-1]);

        // Bullish strike: 3 bearish then 1 bullish closing below first candle's open
        let bull = c3 == -1 && c2 == -1 && c1 == -1 &&
            close[i-2] < close[i-3] && close[i-1] < close[i-2] &&
            open[i-2] < open[i-3] && open[i-1] < open[i-2] &&
            candle_color(open[i], close[i]) == 1 &&
            open[i] < close[i-1] && close[i] > open[i-3];

        // Bearish strike: 3 bullish then 1 bearish closing above first candle's open
        let bear = c3 == 1 && c2 == 1 && c1 == 1 &&
            close[i-2] > close[i-3] && close[i-1] > close[i-2] &&
            open[i-2] > open[i-3] && open[i-1] > open[i-2] &&
            candle_color(open[i], close[i]) == -1 &&
            open[i] > close[i-1] && close[i] < open[i-3];

        if bull { out[i] = 100.0; }
        if bear { out[i] = -100.0; }

        body_sum += real_body(open[i-3], close[i-3]);
        body_sum -= real_body(open[i-3-period], close[i-3-period]);
    }
    out
}
