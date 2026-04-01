//! CDLLONGLEGGEDDOJI — Long Legged Doji
//! Doji (open ≈ close) with long upper and lower shadows.
use super::helpers::*;

pub fn cdllongleggeddoji(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_DOJI_PERIOD.max(SHADOW_LONG_PERIOD);
    let lookback = period;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;
        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);

        let is_doji = rb <= avg * BODY_DOJI_FACTOR;
        let long_shadows = us > avg * SHADOW_LONG_FACTOR && ls > avg * SHADOW_LONG_FACTOR;

        if is_doji && long_shadows {
            out[i] = if candle_color(open[i], close[i]) == 1 { 100.0 } else { -100.0 };
        }

        body_sum += hl_range(high[i], low[i]);
        body_sum -= hl_range(high[i-period], low[i-period]);
    }
    out
}
