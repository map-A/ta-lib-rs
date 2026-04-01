//! CDLLADDERBOTTOM — Ladder Bottom
//! Bullish reversal: 3 declining bearish candles + bearish candle with very short upper shadow + bullish gap-up.
use super::helpers::*;

pub fn cdlladderbottom(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    // ShadowVeryShort: HighLow range, period=10, anchor at i-1
    let period = SHADOW_VERY_SHORT_PERIOD;
    let lookback = period + 4;
    if n <= lookback { return out; }

    // Init: anchor at i-1 means bars [startIdx-1-period .. startIdx-2] = bars [period+3 .. period+3+period-1]
    // In loop starts at i=lookback=period+4, trailing at i-1-period = 3
    // Init sum for trailing starting at index 3 through lookback-1-1 = period+2
    let mut shadow_sum: f64 = (3..=(3 + period - 1)).map(|j| hl_range(high[j], low[j])).sum();
    let mut trailing = 3usize;

    for i in lookback..n {
        let avg_shadow = if period > 0 { shadow_sum / period as f64 } else { 0.0 };

        let is_pattern =
            // First 3: declining bearish candles with lower opens and closes
            candle_color(open[i-4], close[i-4]) == -1 &&
            candle_color(open[i-3], close[i-3]) == -1 &&
            candle_color(open[i-2], close[i-2]) == -1 &&
            open[i-4] > open[i-3] && open[i-3] > open[i-2] &&
            close[i-4] > close[i-3] && close[i-3] > close[i-2] &&
            // 4th: bearish with upper shadow > ShadowVeryShort avg
            candle_color(open[i-1], close[i-1]) == -1 &&
            upper_shadow(open[i-1], high[i-1], close[i-1]) > avg_shadow * SHADOW_VERY_SHORT_FACTOR &&
            // 5th: bullish, opens above prior body, closes above prior high
            candle_color(open[i], close[i]) == 1 &&
            open[i] > open[i-1] &&
            close[i] > high[i-1];

        if is_pattern { out[i] = 100.0; }

        // Update rolling sum anchored at i-1
        shadow_sum += hl_range(high[i-1], low[i-1]);
        shadow_sum -= hl_range(high[trailing], low[trailing]);
        trailing += 1;
    }
    out
}
