//! CDL3OUTSIDE — Three Outside Up/Down
//! Engulfing + confirming candle.
use super::helpers::*;

pub fn cdl3outside(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    if n < 3 { return out; }

    for i in 2..n {
        // 3-outside-up: bearish candle[i-2], bullish engulfing candle[i-1], bullish candle[i]
        let up = candle_color(open[i-2], close[i-2]) == -1 &&
            candle_color(open[i-1], close[i-1]) == 1 &&
            close[i-1] > open[i-2] && open[i-1] < close[i-2] &&  // engulfs
            close[i] > close[i-1];                                 // confirmation

        // 3-outside-down: bullish candle[i-2], bearish engulfing candle[i-1], bearish candle[i]
        let dn = candle_color(open[i-2], close[i-2]) == 1 &&
            candle_color(open[i-1], close[i-1]) == -1 &&
            close[i-1] < open[i-2] && open[i-1] > close[i-2] &&  // engulfs
            close[i] < close[i-1];                                 // confirmation

        if up { out[i] = 100.0; }
        if dn { out[i] = -100.0; }
    }
    out
}
