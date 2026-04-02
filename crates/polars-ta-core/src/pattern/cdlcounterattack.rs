//! CDLCOUNTERATTACK — Counterattack Lines
//! Opposite candles with same close as preceding candle.
use super::helpers::*;

pub fn cdlcounterattack(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD.max(EQUAL_PERIOD);
    let lookback = period + 1;
    if n <= lookback {
        return out;
    }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut equal_sum: f64 = (0..EQUAL_PERIOD).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_body = body_sum / period as f64;
        let avg_equal = equal_sum / EQUAL_PERIOD as f64;
        let rb0 = real_body(open[i - 1], close[i - 1]);
        let rb1 = real_body(open[i], close[i]);

        // Bullish counterattack: bearish[i-1] long, bullish[i] long, closes ~= close[i-1]
        let bull = candle_color(open[i - 1], close[i - 1]) == -1
            && rb0 > avg_body * BODY_LONG_FACTOR
            && candle_color(open[i], close[i]) == 1
            && rb1 > avg_body * BODY_LONG_FACTOR
            && (close[i] - close[i - 1]).abs() <= avg_equal * EQUAL_FACTOR;

        // Bearish counterattack: bullish[i-1] long, bearish[i] long, closes ~= close[i-1]
        let bear = candle_color(open[i - 1], close[i - 1]) == 1
            && rb0 > avg_body * BODY_LONG_FACTOR
            && candle_color(open[i], close[i]) == -1
            && rb1 > avg_body * BODY_LONG_FACTOR
            && (close[i] - close[i - 1]).abs() <= avg_equal * EQUAL_FACTOR;

        if bull {
            out[i] = 100.0;
        }
        if bear {
            out[i] = -100.0;
        }

        body_sum += real_body(open[i - 1], close[i - 1]);
        body_sum -= real_body(open[i - 1 - period], close[i - 1 - period]);
        equal_sum += hl_range(high[i - 1], low[i - 1]);
        equal_sum -= hl_range(high[i - 1 - EQUAL_PERIOD], low[i - 1 - EQUAL_PERIOD]);
    }
    out
}
