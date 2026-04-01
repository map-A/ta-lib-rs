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
    let period = BODY_DOJI_PERIOD;
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut hl_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_hl = hl_sum / period as f64;
        let rb1 = real_body(open[i-1], close[i-1]);

        // Bullish abandoned baby: bearish[i-2], gap-down doji[i-1], gap-up bullish[i]
        let bull = candle_color(open[i-2], close[i-2]) == -1 &&
            rb1 <= avg_hl * BODY_DOJI_FACTOR &&                  // middle is doji
            high[i-1] < low[i-2] &&                             // gap down from first
            candle_color(open[i], close[i]) == 1 &&
            low[i] > high[i-1] &&                               // gap up from doji
            close[i] > close[i-2] + real_body(open[i-2], close[i-2]) * penetration;

        // Bearish abandoned baby: bullish[i-2], gap-up doji[i-1], gap-down bearish[i]
        let bear = candle_color(open[i-2], close[i-2]) == 1 &&
            rb1 <= avg_hl * BODY_DOJI_FACTOR &&
            low[i-1] > high[i-2] &&                             // gap up from first
            candle_color(open[i], close[i]) == -1 &&
            high[i] < low[i-1] &&                               // gap down from doji
            close[i] < close[i-2] - real_body(open[i-2], close[i-2]) * penetration;

        if bull { out[i] = 100.0; }
        if bear { out[i] = -100.0; }

        hl_sum += hl_range(high[i-1], low[i-1]);
        hl_sum -= hl_range(high[i-1-period], low[i-1-period]);
    }
    out
}
