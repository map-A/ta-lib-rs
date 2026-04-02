//! CDLDARKCLOUDCOVER — Dark Cloud Cover (bearish reversal)
use super::helpers::*;

pub fn cdldarkcloudcover(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    cdldarkcloudcover_with_penetration(open, high, low, close, 0.5)
}

pub fn cdldarkcloudcover_with_penetration(
    open: &[f64],
    high: &[f64],
    _low: &[f64],
    close: &[f64],
    penetration: f64,
) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD;
    let lookback = period + 1;
    if n <= lookback {
        return out;
    }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;
        let rb0 = real_body(open[i - 1], close[i - 1]);

        let is_pattern = candle_color(open[i - 1], close[i - 1]) == 1
            && rb0 > avg * BODY_LONG_FACTOR
            && candle_color(open[i], close[i]) == -1
            && open[i] > high[i - 1]
            && close[i] < close[i - 1] - rb0 * penetration
            && close[i] > open[i - 1];

        if is_pattern {
            out[i] = -100.0;
        }

        body_sum += real_body(open[i - 1], close[i - 1]);
        body_sum -= real_body(open[i - 1 - period], close[i - 1 - period]);
    }
    out
}
