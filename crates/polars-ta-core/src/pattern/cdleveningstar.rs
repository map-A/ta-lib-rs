//! CDLEVENINGSTAR — Evening Star (bearish reversal)
use super::helpers::*;

pub fn cdleveningstar(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    cdleveningstar_with_penetration(open, high, low, close, 0.3)
}

pub fn cdleveningstar_with_penetration(
    open: &[f64], high: &[f64], low: &[f64], close: &[f64], penetration: f64
) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD.max(BODY_LONG_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut body_sum_long:  f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut body_sum_short: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg_long  = body_sum_long  / period as f64;
        let avg_short = body_sum_short / period as f64;
        let rb0 = real_body(open[i-2], close[i-2]);
        let rb1 = real_body(open[i-1], close[i-1]);

        let is_pattern =
            candle_color(open[i-2], close[i-2]) == 1 &&
            rb0 > avg_long * BODY_LONG_FACTOR &&
            rb1 <= avg_short * BODY_SHORT_FACTOR &&              // small star
            open[i-1].min(close[i-1]) > close[i-2] &&          // star gaps above
            candle_color(open[i], close[i]) == -1 &&
            close[i] < close[i-2] - rb0 * penetration;

        if is_pattern { out[i] = -100.0; }

        body_sum_long  += real_body(open[i-2], close[i-2]);
        body_sum_long  -= real_body(open[i-2-period], close[i-2-period]);
        body_sum_short += real_body(open[i-1], close[i-1]);
        body_sum_short -= real_body(open[i-1-period], close[i-1-period]);
    }
    out
}
