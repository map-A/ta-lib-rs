//! CDLHANGINGMAN — Hanging Man
//!
//! Same shape as Hammer (small body near high, long lower shadow, very short upper shadow,
//! body near prior low) but returns -100 (bearish). ta-lib does NOT check trend direction.
//!
//! ta-lib candle settings used:
//! - BodyShort:       RealBody, period=10, factor=1.0  → body < avg_body (anchor i-1)
//! - ShadowLong:      RealBody, period=0,  factor=1.0  → lower_shadow > current_body (direct, no avg)
//! - ShadowVeryShort: HighLow,  period=10, factor=0.1  → upper_shadow < avg_HL * 0.1 (anchor i-1)
//! - Near:            HighLow,  period=5,  factor=0.2  → min(o,c) <= low[i-1] + avg_HL * 0.2 (anchor i-2)
use super::helpers::*;

pub fn cdlhangingman(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let lookback = BODY_SHORT_PERIOD
        .max(SHADOW_VERY_SHORT_PERIOD)
        .max(NEAR_PERIOD)
        + 1;
    if n <= lookback {
        return out;
    }

    let mut body_sum: f64 = (1..=BODY_SHORT_PERIOD)
        .map(|j| real_body(open[j], close[j]))
        .sum();
    let mut vshort_sum: f64 = (1..=SHADOW_VERY_SHORT_PERIOD)
        .map(|j| hl_range(high[j], low[j]))
        .sum();
    let mut near_sum: f64 = (5..10_usize).map(|j| hl_range(high[j], low[j])).sum();

    let mut body_trail = 1usize;
    let mut vshort_trail = 1usize;
    let mut near_trail = 5usize;

    for i in lookback..n {
        let avg_body = body_sum / BODY_SHORT_PERIOD as f64;
        let avg_vshort = vshort_sum / SHADOW_VERY_SHORT_PERIOD as f64;
        let avg_near = near_sum / NEAR_PERIOD as f64;
        let rb = real_body(open[i], close[i]);
        let ls = lower_shadow(open[i], low[i], close[i]);
        let us = upper_shadow(open[i], high[i], close[i]);

        if rb < avg_body
            && ls > rb * SHADOW_LONG_FACTOR
            && us < avg_vshort * SHADOW_VERY_SHORT_FACTOR
            && open[i].min(close[i]) >= high[i - 1] - avg_near * NEAR_FACTOR
        {
            out[i] = -100.0;
        }

        body_sum += real_body(open[i], close[i]) - real_body(open[body_trail], close[body_trail]);
        body_trail += 1;
        vshort_sum += hl_range(high[i], low[i]) - hl_range(high[vshort_trail], low[vshort_trail]);
        vshort_trail += 1;
        near_sum += hl_range(high[i - 1], low[i - 1]) - hl_range(high[near_trail], low[near_trail]);
        near_trail += 1;
    }
    out
}
