//! CDLRISEFALL3METHODS — Rising/Falling Three Methods
//! Bullish/bearish continuation: strong candle + 3 small opposite candles + confirming candle.
use super::helpers::*;

pub fn cdlrisefall3methods(open: &[f64], _high: &[f64], _low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD.max(BODY_LONG_PERIOD);
    let lookback = period + 4;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;
        let rb0 = real_body(open[i-4], close[i-4]);

        // Rising Three Methods (bullish)
        let bull = candle_color(open[i-4], close[i-4]) == 1 && rb0 > avg * BODY_LONG_FACTOR
            && candle_color(open[i-3], close[i-3]) == -1
            && candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == -1
            && real_body(open[i-3], close[i-3]) < avg * BODY_SHORT_FACTOR
            && real_body(open[i-2], close[i-2]) < avg * BODY_SHORT_FACTOR
            && real_body(open[i-1], close[i-1]) < avg * BODY_SHORT_FACTOR
            && close[i-3] > open[i-4] && close[i-1] < close[i-4]
            && open[i-3] < close[i-4]
            && candle_color(open[i], close[i]) == 1
            && open[i] > close[i-1]
            && close[i] > close[i-4];

        // Falling Three Methods (bearish)
        let bear = candle_color(open[i-4], close[i-4]) == -1 && rb0 > avg * BODY_LONG_FACTOR
            && candle_color(open[i-3], close[i-3]) == 1
            && candle_color(open[i-2], close[i-2]) == 1
            && candle_color(open[i-1], close[i-1]) == 1
            && real_body(open[i-3], close[i-3]) < avg * BODY_SHORT_FACTOR
            && real_body(open[i-2], close[i-2]) < avg * BODY_SHORT_FACTOR
            && real_body(open[i-1], close[i-1]) < avg * BODY_SHORT_FACTOR
            && close[i-3] < open[i-4] && close[i-1] > close[i-4]
            && open[i-3] > close[i-4]
            && candle_color(open[i], close[i]) == -1
            && open[i] < close[i-1]
            && close[i] < close[i-4];

        if bull { out[i] = 100.0; }
        else if bear { out[i] = -100.0; }

        body_sum += real_body(open[i-4], close[i-4]);
        body_sum -= real_body(open[i-4-period], close[i-4-period]);
    }
    out
}
