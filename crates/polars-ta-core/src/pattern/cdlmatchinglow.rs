//! CDLMATCHINGLOW — Matching Low
//! Bullish reversal: 2 bearish candles with equal (matching) lows.
use super::helpers::*;

pub fn cdlmatchinglow(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = EQUAL_PERIOD;
    let lookback = period + 1;
    if n <= lookback {
        return out;
    }

    let mut equal_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_equal = equal_sum / period as f64;

        let is_pattern = candle_color(open[i - 1], close[i - 1]) == -1
            && candle_color(open[i], close[i]) == -1
            && (close[i] - close[i - 1]).abs() <= avg_equal * EQUAL_FACTOR;

        if is_pattern {
            out[i] = 100.0;
        }

        equal_sum += hl_range(high[i - 1], low[i - 1]);
        equal_sum -= hl_range(high[i - 1 - period], low[i - 1 - period]);
    }
    out
}
