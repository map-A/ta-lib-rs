//! CDLDRAGONFLYDOJI — Dragonfly Doji
//! Doji with long lower shadow and no upper shadow.
use super::helpers::*;

pub fn cdldragonflydoji(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_DOJI_PERIOD.max(SHADOW_VERY_SHORT_PERIOD);
    let lookback = period;
    if n <= lookback { return out; }

    let mut hl_sum:   f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();
    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg_hl   = hl_sum   / period as f64;
        let avg_body = body_sum / period as f64;

        let is_pattern =
            real_body(open[i], close[i]) <= avg_hl * BODY_DOJI_FACTOR &&
            upper_shadow(open[i], high[i], close[i]) < avg_hl * SHADOW_VERY_SHORT_FACTOR &&
            lower_shadow(open[i], low[i], close[i]) > avg_body * SHADOW_LONG_FACTOR;

        if is_pattern { out[i] = 100.0; }

        hl_sum   += hl_range(high[i], low[i]);
        hl_sum   -= hl_range(high[i - period], low[i - period]);
        body_sum += real_body(open[i], close[i]);
        body_sum -= real_body(open[i - period], close[i - period]);
    }
    out
}
