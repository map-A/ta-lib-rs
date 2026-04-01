//! CDLMARUBOZU — Marubozu
//! Candle with no upper and no lower shadow (or very small).
use super::helpers::*;

pub fn cdlmarubozu(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD.max(SHADOW_VERY_SHORT_PERIOD);
    let lookback = period;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut shadow_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_body = body_sum / period as f64;
        let avg_shadow = shadow_sum / period as f64;
        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);

        if rb > avg_body * BODY_LONG_FACTOR
            && us <= avg_shadow * SHADOW_VERY_SHORT_FACTOR
            && ls <= avg_shadow * SHADOW_VERY_SHORT_FACTOR
        {
            out[i] = if candle_color(open[i], close[i]) == 1 { 100.0 } else { -100.0 };
        }

        body_sum += real_body(open[i], close[i]);
        body_sum -= real_body(open[i-period], close[i-period]);
        shadow_sum += hl_range(high[i], low[i]);
        shadow_sum -= hl_range(high[i-period], low[i-period]);
    }
    out
}
