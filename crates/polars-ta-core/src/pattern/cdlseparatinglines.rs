//! CDLSEPARATINGLINES — Separating Lines
//! Trend continuation: 2 candles of opposite color with same open.
use super::helpers::*;

pub fn cdlseparatinglines(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let body_period = BODY_LONG_PERIOD;        // 10
    let shadow_period = SHADOW_VERY_SHORT_PERIOD; // 10
    let equal_period = EQUAL_PERIOD;           // 5
    let lookback = body_period.max(shadow_period).max(equal_period) + 1; // 11
    if n <= lookback { return out; }

    // ShadowVeryShortTrailingIdx = startIdx - shadow_period = 11 - 10 = 1 → init bars [1..10]
    let mut shadow_sum: f64 = (1..=shadow_period).map(|j| hl_range(high[j], low[j])).sum();
    // BodyLongTrailingIdx = startIdx - body_period = 11 - 10 = 1 → init bars [1..10]
    let mut body_sum: f64 = (1..=body_period).map(|j| real_body(open[j], close[j])).sum();
    // EqualTrailingIdx = startIdx - equal_period = 11 - 5 = 6 → init accumulates hl_range[i-1] for i=6..10
    let mut equal_sum: f64 = ((lookback - equal_period)..lookback)
        .map(|j| hl_range(high[j-1], low[j-1])).sum();

    let mut shadow_trailing = 1usize;
    let mut body_trailing = 1usize;
    let mut equal_trailing = lookback - equal_period; // 6

    for i in lookback..n {
        let avg_body = body_sum / body_period as f64;
        let avg_shadow = shadow_sum / shadow_period as f64;
        let avg_equal = equal_sum / equal_period as f64 * EQUAL_FACTOR;

        let rb = real_body(open[i], close[i]);
        let c0 = candle_color(open[i-1], close[i-1]);
        let c1 = candle_color(open[i], close[i]);

        let same_open = (open[i] - open[i-1]).abs() <= avg_equal;
        let long_body = rb > avg_body * BODY_LONG_FACTOR;
        // Belt hold: bullish → no lower shadow; bearish → no upper shadow
        let belt_hold = (c1 == 1 && lower_shadow(open[i], low[i], close[i]) < avg_shadow * SHADOW_VERY_SHORT_FACTOR)
            || (c1 == -1 && upper_shadow(open[i], high[i], close[i]) < avg_shadow * SHADOW_VERY_SHORT_FACTOR);

        if c0 == -c1 && same_open && long_body && belt_hold {
            out[i] = c1 as f64 * 100.0;
        }

        // ShadowVeryShort and BodyLong anchored at i; Equal anchored at i-1
        shadow_sum += hl_range(high[i], low[i]) - hl_range(high[shadow_trailing], low[shadow_trailing]);
        shadow_trailing += 1;
        body_sum += real_body(open[i], close[i]) - real_body(open[body_trailing], close[body_trailing]);
        body_trailing += 1;
        equal_sum += hl_range(high[i-1], low[i-1]) - hl_range(high[equal_trailing-1], low[equal_trailing-1]);
        equal_trailing += 1;
    }
    out
}
