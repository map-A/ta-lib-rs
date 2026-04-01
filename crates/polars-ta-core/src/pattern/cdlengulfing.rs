//! CDLENGULFING — Engulfing Pattern
//! Two-candle pattern: second candle's body engulfs the first's.
use super::helpers::*;

pub fn cdlengulfing(open: &[f64], _high: &[f64], _low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    if n < 2 { return out; }

    for i in 1..n {
        let c0 = candle_color(open[i-1], close[i-1]);
        let c1 = candle_color(open[i],   close[i]);

        // Bullish engulfing: bearish then bullish, second engulfs first
        let bull = c0 == -1 && c1 == 1 &&
            close[i] > open[i-1] && open[i] < close[i-1];

        // Bearish engulfing: bullish then bearish, second engulfs first
        let bear = c0 == 1 && c1 == -1 &&
            close[i] < open[i-1] && open[i] > close[i-1];

        if bull { out[i] = 100.0; }
        if bear { out[i] = -100.0; }
    }
    out
}
