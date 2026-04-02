//! CDLSPINNINGTOP — Spinning Top
//! Small body with shadows longer than the body on both sides.
use super::helpers::*;

pub fn cdlspinningtop(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD;
    let lookback = period;
    if n <= lookback {
        return out;
    }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg_body = body_sum / period as f64;
        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);

        if rb < avg_body * BODY_SHORT_FACTOR && us > rb && ls > rb {
            out[i] = candle_color(open[i], close[i]) as f64 * 100.0;
        }

        body_sum += real_body(open[i], close[i]);
        body_sum -= real_body(open[i - period], close[i - period]);
    }
    out
}
