//! CDLHARAMI — Harami Pattern
//! Large candle followed by small one contained within it.
use super::helpers::*;

pub fn cdlharami(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD.max(BODY_SHORT_PERIOD);
    let lookback = period + 1;
    if n <= lookback { return out; }

    let mut body_sum_long:  f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut body_sum_short: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg_long  = body_sum_long  / period as f64;
        let avg_short = body_sum_short / period as f64;
        let rb0 = real_body(open[i-1], close[i-1]);
        let rb1 = real_body(open[i], close[i]);

        let o0 = open[i-1]; let c0 = close[i-1];
        let o1 = open[i];   let c1 = close[i];

        let bull = candle_color(o0, c0) == -1 &&
            rb0 > avg_long * BODY_LONG_FACTOR &&
            rb1 < avg_short * BODY_SHORT_FACTOR &&
            o1.max(c1) < o0.max(c0) &&  // contained within prev body
            o1.min(c1) > o0.min(c0);

        let bear = candle_color(o0, c0) == 1 &&
            rb0 > avg_long * BODY_LONG_FACTOR &&
            rb1 < avg_short * BODY_SHORT_FACTOR &&
            o1.max(c1) < o0.max(c0) &&
            o1.min(c1) > o0.min(c0);

        if bull { out[i] = 100.0; }
        if bear { out[i] = -100.0; }

        body_sum_long  += real_body(open[i-1], close[i-1]);
        body_sum_long  -= real_body(open[i-1-period], close[i-1-period]);
        body_sum_short += real_body(open[i], close[i]);
        body_sum_short -= real_body(open[i-period], close[i-period]);
    }
    out
}
