//! CDLSHORTLINE — Short Line Candle
//! Candle with a short body and short shadows.
use super::helpers::*;

pub fn cdlshortline(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD.max(SHADOW_SHORT_PERIOD);
    let lookback = period;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut shadow_sum: f64 = (0..period)
        .map(|j| upper_shadow(open[j], high[j], close[j]) + lower_shadow(open[j], low[j], close[j]))
        .sum();

    for i in lookback..n {
        let avg_body = body_sum / period as f64;
        let avg_shadow = shadow_sum / period as f64;
        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);

        if rb < avg_body * BODY_SHORT_FACTOR
            && us < avg_shadow * SHADOW_SHORT_FACTOR
            && ls < avg_shadow * SHADOW_SHORT_FACTOR
        {
            out[i] = if candle_color(open[i], close[i]) == 1 { 100.0 } else { -100.0 };
        }

        body_sum += real_body(open[i], close[i]);
        body_sum -= real_body(open[i-period], close[i-period]);
        shadow_sum += upper_shadow(open[i], high[i], close[i]) + lower_shadow(open[i], low[i], close[i]);
        shadow_sum -= upper_shadow(open[i-period], high[i-period], close[i-period])
            + lower_shadow(open[i-period], low[i-period], close[i-period]);
    }
    out
}
