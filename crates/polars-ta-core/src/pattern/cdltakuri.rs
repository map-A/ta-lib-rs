//! CDLTAKURI — Takuri (Dragonfly Doji with very long lower shadow)
//! Doji with no upper shadow and very long lower shadow.
use super::helpers::*;

pub fn cdltakuri(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_DOJI_PERIOD
        .max(SHADOW_VERY_SHORT_PERIOD)
        .max(SHADOW_VERY_LONG_PERIOD);
    let lookback = period;
    if n <= lookback {
        return out;
    }

    let mut body_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;
        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);

        let is_pattern = rb <= avg * BODY_DOJI_FACTOR
            && us <= avg * SHADOW_VERY_SHORT_FACTOR
            && ls > rb * SHADOW_VERY_LONG_FACTOR;

        if is_pattern {
            out[i] = 100.0;
        }

        body_sum += hl_range(high[i], low[i]);
        body_sum -= hl_range(high[i - period], low[i - period]);
    }
    out
}
