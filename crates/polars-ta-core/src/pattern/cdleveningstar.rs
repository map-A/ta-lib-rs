//! CDLEVENINGSTAR — Evening Star (bearish reversal)
use super::helpers::*;

pub fn cdleveningstar(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    cdleveningstar_with_penetration(open, high, low, close, 0.3)
}

pub fn cdleveningstar_with_penetration(
    open: &[f64], _high: &[f64], _low: &[f64], close: &[f64], penetration: f64
) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD.max(BODY_LONG_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    // body_long_2 anchored at i-2 (init [0..period-1])
    // body_short_1 anchored at i-1 (init [1..period])
    // body_long_0 anchored at i    (init [2..period+1])
    let mut body_long_2:  f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut body_short_1: f64 = (1..=period).map(|j| real_body(open[j], close[j])).sum();
    let mut body_long_0:  f64 = (2..period+2).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg_long_2  = body_long_2  / period as f64;
        let avg_short_1 = body_short_1 / period as f64;
        let avg_long_0  = body_long_0  / period as f64;
        let rb0  = real_body(open[i-2], close[i-2]);
        let rb1  = real_body(open[i-1], close[i-1]);
        let rb_i = real_body(open[i],   close[i]);

        let is_pattern =
            candle_color(open[i-2], close[i-2]) == 1 &&
            rb0  > avg_long_2  * BODY_LONG_FACTOR &&
            rb1  <= avg_short_1 * BODY_SHORT_FACTOR &&
            open[i-1].min(close[i-1]) > close[i-2] &&
            candle_color(open[i], close[i]) == -1 &&
            rb_i > avg_long_0  * BODY_LONG_FACTOR &&
            close[i] < close[i-2] - rb0 * penetration;

        if is_pattern { out[i] = -100.0; }

        body_long_2  += real_body(open[i-2], close[i-2]);
        body_long_2  -= real_body(open[i-2-period], close[i-2-period]);
        body_short_1 += real_body(open[i-1], close[i-1]);
        body_short_1 -= real_body(open[i-1-period], close[i-1-period]);
        body_long_0  += real_body(open[i], close[i]);
        body_long_0  -= real_body(open[i-period], close[i-period]);
    }
    out
}
