//! CDLABANDONEDBABY — Abandoned Baby
//! Bullish or bearish gap-doji-gap pattern.
use super::helpers::*;

pub fn cdlabandonedbaby(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    cdlabandonedbaby_with_penetration(open, high, low, close, 0.3)
}

pub fn cdlabandonedbaby_with_penetration(
    open: &[f64], high: &[f64], low: &[f64], close: &[f64], penetration: f64
) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD.max(BODY_DOJI_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    // Three rolling averages: body_long anchored at i-2, i, and doji hl at i-1
    let mut body_long_2: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut body_long_0: f64 = (2..period+2).map(|j| real_body(open[j], close[j])).sum();
    let mut hl_doji:     f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_long_2 = body_long_2 / period as f64;
        let avg_long_0 = body_long_0 / period as f64;
        let avg_hl     = hl_doji / period as f64;
        let rb2 = real_body(open[i-2], close[i-2]);
        let rb1 = real_body(open[i-1], close[i-1]);
        let rb0 = real_body(open[i],   close[i]);

        // Bullish: long bearish[i-2], gap-down doji[i-1], gap-up long bullish[i]
        let bull = candle_color(open[i-2], close[i-2]) == -1 &&
            rb2 > avg_long_2 * BODY_LONG_FACTOR &&
            rb1 <= avg_hl * BODY_DOJI_FACTOR &&
            high[i-1] < low[i-2] &&
            candle_color(open[i], close[i]) == 1 &&
            rb0 > avg_long_0 * BODY_LONG_FACTOR &&
            low[i] > high[i-1] &&
            close[i] > close[i-2] + rb2 * penetration;

        // Bearish: long bullish[i-2], gap-up doji[i-1], gap-down long bearish[i]
        let bear = candle_color(open[i-2], close[i-2]) == 1 &&
            rb2 > avg_long_2 * BODY_LONG_FACTOR &&
            rb1 <= avg_hl * BODY_DOJI_FACTOR &&
            low[i-1] > high[i-2] &&
            candle_color(open[i], close[i]) == -1 &&
            rb0 > avg_long_0 * BODY_LONG_FACTOR &&
            high[i] < low[i-1] &&
            close[i] < close[i-2] - rb2 * penetration;

        if bull { out[i] = 100.0; }
        if bear { out[i] = -100.0; }

        body_long_2 += real_body(open[i-2], close[i-2]);
        body_long_2 -= real_body(open[i-2-period], close[i-2-period]);
        body_long_0 += real_body(open[i], close[i]);
        body_long_0 -= real_body(open[i-period], close[i-period]);
        hl_doji += hl_range(high[i-1], low[i-1]);
        hl_doji -= hl_range(high[i-1-period], low[i-1-period]);
    }
    out
}
