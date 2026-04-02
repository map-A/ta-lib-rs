//! CDL3INSIDE — Three Inside Up/Down
//! Harami (2-candle) + confirming third candle.
use super::helpers::*;

pub fn cdl3inside(open: &[f64], _high: &[f64], _low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD.max(BODY_SHORT_PERIOD);
    let lookback = period + 2;
    if n <= lookback {
        return out;
    }

    let mut body_sum_long: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut body_sum_short: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg_long = body_sum_long / period as f64;
        let avg_short = body_sum_short / period as f64;
        let rb0 = real_body(open[i - 2], close[i - 2]);
        let rb1 = real_body(open[i - 1], close[i - 1]);

        let o0 = open[i - 2];
        let c0 = close[i - 2];
        let o1 = open[i - 1];
        let c1 = close[i - 1];

        // i-1 body inside i-2 body (same logic as CDLHARAMI)
        let inside = o1.max(c1) < o0.max(c0) && o1.min(c1) > o0.min(c0);

        // 3-inside-up: bearish big → bullish small inside → bullish close above first open
        let up = candle_color(o0, c0) == -1
            && rb0 > avg_long * BODY_LONG_FACTOR
            && rb1 < avg_short * BODY_SHORT_FACTOR
            && inside
            && candle_color(open[i], close[i]) == 1
            && close[i] > o0;

        // 3-inside-down: bullish big → bearish small inside → bearish close below first open
        let dn = candle_color(o0, c0) == 1
            && rb0 > avg_long * BODY_LONG_FACTOR
            && rb1 < avg_short * BODY_SHORT_FACTOR
            && inside
            && candle_color(open[i], close[i]) == -1
            && close[i] < o0;

        if up {
            out[i] = 100.0;
        }
        if dn {
            out[i] = -100.0;
        }

        body_sum_long += real_body(open[i - 2], close[i - 2]);
        body_sum_long -= real_body(open[i - 2 - period], close[i - 2 - period]);
        body_sum_short += real_body(open[i - 1], close[i - 1]);
        body_sum_short -= real_body(open[i - 1 - period], close[i - 1 - period]);
    }
    out
}
