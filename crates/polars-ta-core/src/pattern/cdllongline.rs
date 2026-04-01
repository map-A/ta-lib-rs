//! CDLLONGLINE — Long Line Candle
//! Candle with a long body and short upper and lower shadows.
use super::helpers::*;

pub fn cdllongline(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    // BodyLong: RealBody, period=10, anchor at i
    // ShadowShort: Shadows (upper+lower), period=10, anchor at i
    let body_period = BODY_LONG_PERIOD;
    let shadow_period = SHADOW_SHORT_PERIOD;
    let lookback = body_period.max(shadow_period);
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..body_period).map(|j| real_body(open[j], close[j])).sum();
    let mut shadow_sum: f64 = (0..shadow_period)
        .map(|j| upper_shadow(open[j], high[j], close[j]) + lower_shadow(open[j], low[j], close[j]))
        .sum();
    let mut body_trailing = 0usize;
    let mut shadow_trailing = 0usize;

    for i in lookback..n {
        let avg_body = body_sum / body_period as f64;
        let avg_shadow = shadow_sum / shadow_period as f64;
        let rb = real_body(open[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);

        if rb > avg_body * BODY_LONG_FACTOR &&
           us < avg_shadow * SHADOW_SHORT_FACTOR &&
           ls < avg_shadow * SHADOW_SHORT_FACTOR
        {
            out[i] = candle_color(open[i], close[i]) as f64 * 100.0;
        }

        body_sum += real_body(open[i], close[i]);
        body_sum -= real_body(open[body_trailing], close[body_trailing]);
        body_trailing += 1;

        shadow_sum += upper_shadow(open[i], high[i], close[i]) + lower_shadow(open[i], low[i], close[i]);
        shadow_sum -= upper_shadow(open[shadow_trailing], high[shadow_trailing], close[shadow_trailing])
            + lower_shadow(open[shadow_trailing], low[shadow_trailing], close[shadow_trailing]);
        shadow_trailing += 1;
    }
    out
}
