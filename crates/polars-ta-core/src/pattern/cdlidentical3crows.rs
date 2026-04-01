//! CDLIDENTICAL3CROWS — Identical Three Crows
//! Bearish reversal: 3 consecutive bearish candles with similar opens (equal to prev close).
use super::helpers::*;

pub fn cdlidentical3crows(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = SHADOW_VERY_SHORT_PERIOD.max(EQUAL_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut shadow_sum: f64 = (0..period).map(|j| upper_shadow(open[j], high[j], close[j])).sum();
    let mut equal_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_shadow = shadow_sum / period as f64;
        let avg_equal = equal_sum / period as f64;

        let is_pattern =
            candle_color(open[i-2], close[i-2]) == -1 &&
            upper_shadow(open[i-2], high[i-2], close[i-2]) <= avg_shadow * SHADOW_VERY_SHORT_FACTOR &&
            candle_color(open[i-1], close[i-1]) == -1 &&
            upper_shadow(open[i-1], high[i-1], close[i-1]) <= avg_shadow * SHADOW_VERY_SHORT_FACTOR &&
            candle_color(open[i], close[i]) == -1 &&
            upper_shadow(open[i], high[i], close[i]) <= avg_shadow * SHADOW_VERY_SHORT_FACTOR &&
            // Opens near previous close
            (open[i-1] - close[i-2]).abs() <= avg_equal * EQUAL_FACTOR &&
            (open[i] - close[i-1]).abs() <= avg_equal * EQUAL_FACTOR;

        if is_pattern { out[i] = -100.0; }

        shadow_sum += upper_shadow(open[i-2], high[i-2], close[i-2]);
        shadow_sum -= upper_shadow(open[i-2-period], high[i-2-period], close[i-2-period]);
        equal_sum += hl_range(high[i-2], low[i-2]);
        equal_sum -= hl_range(high[i-2-period], low[i-2-period]);
    }
    out
}
