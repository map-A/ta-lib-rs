//! CDLCONCEALBABYSWALL — Concealing Baby Swallow
//! 4-candle bullish reversal pattern.
use super::helpers::*;

pub fn cdlconcealbabyswall(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = SHADOW_VERY_SHORT_PERIOD;
    let lookback = period + 3;
    if n <= lookback {
        return out;
    }

    let mut shadow_sum: [f64; 4] = [
        (0..period).map(|j| hl_range(high[j], low[j])).sum(),
        (0..period).map(|j| hl_range(high[j], low[j])).sum(),
        (0..period).map(|j| hl_range(high[j], low[j])).sum(),
        (0..period).map(|j| hl_range(high[j], low[j])).sum(),
    ];

    for i in lookback..n {
        let avg0 = shadow_sum[0] / period as f64;
        let avg1 = shadow_sum[1] / period as f64;

        let is_pattern = candle_color(open[i-3], close[i-3]) == -1 &&
            candle_color(open[i-2], close[i-2]) == -1 &&
            candle_color(open[i-1], close[i-1]) == -1 &&
            candle_color(open[i],   close[i])   == -1 &&
            // First two: marubozus (no upper/lower shadow)
            upper_shadow(open[i-3], high[i-3], close[i-3]) < avg0 * SHADOW_VERY_SHORT_FACTOR &&
            lower_shadow(open[i-3], low[i-3],  close[i-3]) < avg0 * SHADOW_VERY_SHORT_FACTOR &&
            upper_shadow(open[i-2], high[i-2], close[i-2]) < avg1 * SHADOW_VERY_SHORT_FACTOR &&
            lower_shadow(open[i-2], low[i-2],  close[i-2]) < avg1 * SHADOW_VERY_SHORT_FACTOR &&
            // Third: gap down, upper shadow extends into second candle's body
            open[i-1] < close[i-2] &&
            high[i-1] > close[i-2] &&
            // Fourth: bearish marubozu engulfing third
            open[i] >= open[i-1].max(close[i-1]) &&
            close[i] <= open[i-1].min(close[i-1]);

        if is_pattern {
            out[i] = 100.0;
        }

        for k in 0..4usize {
            let j = i - 3 + k;
            shadow_sum[k] += hl_range(high[j], low[j]);
            shadow_sum[k] -= hl_range(high[j - period], low[j - period]);
        }
    }
    out
}
