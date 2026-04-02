//! CDLTHRUSTING — Thrusting Pattern
//! Bearish continuation: bearish candle + bullish that opens below prev low and closes below midpoint.
use super::helpers::*;

pub fn cdlthrusting(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD.max(EQUAL_PERIOD);
    let lookback = period + 1;
    if n <= lookback {
        return out;
    }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut equal_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_body = body_sum / period as f64;
        let avg_equal = equal_sum / period as f64;
        let rb0 = real_body(open[i - 1], close[i - 1]);
        let mid0 = (open[i - 1] + close[i - 1]) / 2.0;

        let is_pattern = candle_color(open[i - 1], close[i - 1]) == -1
            && rb0 > avg_body * BODY_LONG_FACTOR
            && candle_color(open[i], close[i]) == 1
            && open[i] < low[i - 1]
            && close[i] > close[i - 1]
            && close[i] < mid0 - avg_equal * EQUAL_FACTOR;

        if is_pattern {
            out[i] = -100.0;
        }

        body_sum += real_body(open[i - 1], close[i - 1]);
        body_sum -= real_body(open[i - 1 - period], close[i - 1 - period]);
        equal_sum += hl_range(high[i - 1], low[i - 1]);
        equal_sum -= hl_range(high[i - 1 - period], low[i - 1 - period]);
    }
    out
}
