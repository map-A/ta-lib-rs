//! CDLXSIDEGAP3METHODS — Upside/Downside Gap Three Methods
//! Continuation: two same-color candles with gap, then opposite candle closing within first candle.
use super::helpers::*;

pub fn cdlxsidegap3methods(open: &[f64], _high: &[f64], _low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    if n < 3 { return out; }

    for i in 2..n {
        let c0 = candle_color(open[i-2], close[i-2]);
        let c1 = candle_color(open[i-1], close[i-1]);
        let c2 = candle_color(open[i], close[i]);

        if c0 != c1 || c1 != -c2 { continue; }

        // Third opens within second real body
        let opens_in_2nd = open[i] < open[i-1].max(close[i-1]) && open[i] > open[i-1].min(close[i-1]);
        // Third closes within first real body
        let closes_in_1st = close[i] < open[i-2].max(close[i-2]) && close[i] > open[i-2].min(close[i-2]);
        // Real body gap between first and second
        let gap = if c0 == 1 {
            open[i-1].min(close[i-1]) > open[i-2].max(close[i-2])  // REALBODYGAPUP
        } else {
            open[i-1].max(close[i-1]) < open[i-2].min(close[i-2])  // REALBODYGAPDOWN
        };

        if opens_in_2nd && closes_in_1st && gap {
            out[i] = c0 as f64 * 100.0;
        }
    }
    out
}
