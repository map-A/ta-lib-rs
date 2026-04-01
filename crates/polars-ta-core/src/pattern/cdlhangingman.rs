//! CDLHANGINGMAN — Hanging Man
//! Small body near top, long lower shadow, short upper shadow. Bearish reversal.
//! Same shape as Hammer but appears after an uptrend.
use super::helpers::*;

pub fn cdlhangingman(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD.max(SHADOW_LONG_PERIOD)
        .max(SHADOW_VERY_SHORT_PERIOD).max(NEAR_PERIOD);
    let lookback = period + 1;
    if n <= lookback { return out; }

    let mut body_sum:   f64 = (0..BODY_SHORT_PERIOD).map(|j| real_body(open[j], close[j])).sum();
    let mut shadow_sum: f64 = (0..SHADOW_LONG_PERIOD).map(|j| real_body(open[j], close[j])).sum();
    let mut near_sum:   f64 = (0..NEAR_PERIOD).map(|j| hl_range(high[j], low[j])).sum();
    let mut hl_sum:     f64 = (0..SHADOW_VERY_SHORT_PERIOD).map(|j| hl_range(high[j], low[j])).sum();

    for i in (period + 1)..n {
        let avg_body   = body_sum   / BODY_SHORT_PERIOD as f64;
        let avg_shadow = shadow_sum / SHADOW_LONG_PERIOD as f64;
        let avg_near   = near_sum   / NEAR_PERIOD as f64;
        let avg_hl     = hl_sum     / SHADOW_VERY_SHORT_PERIOD as f64;
        let rb = real_body(open[i], close[i]);

        let is_pattern =
            rb < avg_body * BODY_SHORT_FACTOR &&
            lower_shadow(open[i], low[i], close[i]) > avg_shadow * SHADOW_LONG_FACTOR &&
            upper_shadow(open[i], high[i], close[i]) < avg_hl * SHADOW_VERY_SHORT_FACTOR &&
            open[i].min(close[i]) >= high[i] - avg_near * NEAR_FACTOR &&
            // Prior candle is bullish (uptrend confirmation)
            candle_color(open[i-1], close[i-1]) == 1;

        if is_pattern { out[i] = -100.0; }

        body_sum   += real_body(open[i], close[i]);
        body_sum   -= real_body(open[i - BODY_SHORT_PERIOD], close[i - BODY_SHORT_PERIOD]);
        shadow_sum += real_body(open[i], close[i]);
        shadow_sum -= real_body(open[i - SHADOW_LONG_PERIOD], close[i - SHADOW_LONG_PERIOD]);
        near_sum   += hl_range(high[i], low[i]);
        near_sum   -= hl_range(high[i - NEAR_PERIOD], low[i - NEAR_PERIOD]);
        hl_sum     += hl_range(high[i], low[i]);
        hl_sum     -= hl_range(high[i - SHADOW_VERY_SHORT_PERIOD], low[i - SHADOW_VERY_SHORT_PERIOD]);
    }
    out
}
