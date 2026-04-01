//! CDLMORNINGDOJISTAR — Morning Doji Star
//! Bearish candle + gapped doji star + bullish candle closing into first body.
use super::helpers::*;

pub fn cdlmorningdojistar(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD.max(BODY_DOJI_PERIOD).max(BODY_SHORT_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    // Three rolling averages for three separate candle conditions
    let mut body_long_2: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut hl_doji:     f64 = (1..=period).map(|j| hl_range(high[j], low[j])).sum();
    let mut body_short_0:f64 = (2..period+2).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg_long  = body_long_2  / period as f64;
        let avg_hl    = hl_doji      / period as f64;
        let avg_short = body_short_0 / period as f64;

        let rb0 = real_body(open[i-2], close[i-2]);
        let rb1 = real_body(open[i-1], close[i-1]);
        let rb2 = real_body(open[i],   close[i]);
        let mid0 = (open[i-2] + close[i-2]) / 2.0;

        let is_pattern =
            candle_color(open[i-2], close[i-2]) == -1 &&
            rb0 > avg_long  * BODY_LONG_FACTOR &&
            rb1 <= avg_hl   * BODY_DOJI_FACTOR &&
            open[i-1].max(close[i-1]) < close[i-2] &&
            candle_color(open[i], close[i]) == 1 &&
            rb2 > avg_short * BODY_SHORT_FACTOR &&
            close[i] > mid0;

        if is_pattern { out[i] = 100.0; }

        body_long_2  += real_body(open[i-2], close[i-2]);
        body_long_2  -= real_body(open[i-2-period], close[i-2-period]);
        hl_doji      += hl_range(high[i-1], low[i-1]);
        hl_doji      -= hl_range(high[i-1-period], low[i-1-period]);
        body_short_0 += real_body(open[i], close[i]);
        body_short_0 -= real_body(open[i-period], close[i-period]);
    }
    out
}
