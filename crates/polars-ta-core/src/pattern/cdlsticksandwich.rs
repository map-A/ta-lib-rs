//! CDLSTICKSANDWICH — Stick Sandwich
//! Bullish reversal: two bearish candles with same close sandwiching a bullish candle.
use super::helpers::*;

pub fn cdlsticksandwich(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = EQUAL_PERIOD;
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut equal_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_equal = equal_sum / period as f64;

        let is_pattern =
            candle_color(open[i-2], close[i-2]) == -1 &&
            candle_color(open[i-1], close[i-1]) == 1 &&
            candle_color(open[i], close[i]) == -1 &&
            low[i-1] > close[i-2] &&
            (close[i] - close[i-2]).abs() <= avg_equal * EQUAL_FACTOR;

        if is_pattern { out[i] = 100.0; }

        equal_sum += hl_range(high[i-2], low[i-2]);
        equal_sum -= hl_range(high[i-2-period], low[i-2-period]);
    }
    out
}
