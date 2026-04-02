//! CDLSHOOTINGSTAR — Shooting Star
//! Bearish reversal: small body near low, long upper shadow, small lower shadow.
use super::helpers::*;

pub fn cdlshootingstar(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD
        .max(SHADOW_LONG_PERIOD)
        .max(SHADOW_VERY_SHORT_PERIOD);
    let lookback = period + 1;
    if n <= lookback {
        return out;
    }

    let mut body_sum: f64 = (1..=period).map(|j| real_body(open[j], close[j])).sum();
    let mut shadow_sum: f64 = (1..=period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_body = body_sum / period as f64;
        let avg_shadow = shadow_sum / period as f64;

        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);

        let is_pattern = rb < avg_body * BODY_SHORT_FACTOR &&
            us > rb &&
            ls < avg_shadow * SHADOW_VERY_SHORT_FACTOR &&
            // Star position: body above previous candle's body (gap up)
            open[i].min(close[i]) > open[i-1].max(close[i-1]);

        if is_pattern {
            out[i] = -100.0;
        }

        body_sum += real_body(open[i], close[i]);
        body_sum -= real_body(open[i - period], close[i - period]);
        shadow_sum += hl_range(high[i], low[i]);
        shadow_sum -= hl_range(high[i - period], low[i - period]);
    }
    out
}
