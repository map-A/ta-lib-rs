//! CDLLADDERBOTTOM — Ladder Bottom
//! Bullish reversal: 3 declining bearish candles + bearish candle with long upper shadow + bullish gap-up.
use super::helpers::*;

pub fn cdlladderbottom(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = SHADOW_VERY_SHORT_PERIOD.max(SHADOW_LONG_PERIOD);
    let lookback = period + 4;
    if n <= lookback { return out; }

    let mut shadow_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_shadow = shadow_sum / period as f64;

        let is_pattern =
            // First 3: declining bearish candles
            candle_color(open[i-4], close[i-4]) == -1 &&
            candle_color(open[i-3], close[i-3]) == -1 &&
            candle_color(open[i-2], close[i-2]) == -1 &&
            open[i-3] < open[i-4] && open[i-2] < open[i-3] &&
            close[i-3] < close[i-4] && close[i-2] < close[i-3] &&
            // 4th: bearish with long upper shadow
            candle_color(open[i-1], close[i-1]) == -1 &&
            upper_shadow(open[i-1], high[i-1], close[i-1]) > avg_shadow * SHADOW_LONG_FACTOR &&
            // 5th: bullish gap up
            candle_color(open[i], close[i]) == 1 &&
            open[i] > open[i-1];

        if is_pattern { out[i] = 100.0; }

        shadow_sum += hl_range(high[i-4], low[i-4]);
        shadow_sum -= hl_range(high[i-4-period], low[i-4-period]);
    }
    out
}
