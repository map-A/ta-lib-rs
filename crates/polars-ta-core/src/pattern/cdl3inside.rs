//! CDL3INSIDE — Three Inside Up/Down
//! Harami + confirming candle.
use super::helpers::*;

pub fn cdl3inside(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD.max(BODY_LONG_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut body_sum_long: f64  = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut body_sum_short: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg_long  = body_sum_long  / period as f64;
        let avg_short = body_sum_short / period as f64;
        let rb0 = real_body(open[i-2], close[i-2]);
        let rb1 = real_body(open[i-1], close[i-1]);

        // 3-inside-up: bearish big candle[i-2], bullish small candle[i-1] inside, bullish close[i]
        let up = candle_color(open[i-2], close[i-2]) == -1 &&
            rb0 > avg_long * BODY_LONG_FACTOR &&
            candle_color(open[i-1], close[i-1]) == 1 &&
            rb1 <= avg_short * BODY_SHORT_FACTOR &&
            close[i-1] < open[i-2] && open[i-1] > close[i-2] &&   // inside
            close[i] > close[i-1];                                  // confirming bullish

        // 3-inside-down: bullish big candle[i-2], bearish small candle[i-1] inside, bearish close[i]
        let dn = candle_color(open[i-2], close[i-2]) == 1 &&
            rb0 > avg_long * BODY_LONG_FACTOR &&
            candle_color(open[i-1], close[i-1]) == -1 &&
            rb1 <= avg_short * BODY_SHORT_FACTOR &&
            close[i-1] > open[i-2] && open[i-1] < close[i-2] &&   // inside
            close[i] < close[i-1];                                  // confirming bearish

        if up  { out[i] = 100.0; }
        if dn  { out[i] = -100.0; }

        body_sum_long  += real_body(open[i-2], close[i-2]);
        body_sum_long  -= real_body(open[i-2-period], close[i-2-period]);
        body_sum_short += real_body(open[i-1], close[i-1]);
        body_sum_short -= real_body(open[i-1-period], close[i-1-period]);
    }
    out
}
