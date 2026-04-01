//! CDLDOJISTAR — Doji Star
//! Trending candle followed by a doji (potential reversal).
use super::helpers::*;

pub fn cdldojistar(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_DOJI_PERIOD.max(BODY_LONG_PERIOD);
    let lookback = period + 1;
    if n <= lookback { return out; }

    let mut hl_sum:   f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();
    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg_hl   = hl_sum   / period as f64;
        let avg_body = body_sum / period as f64;
        let rb1 = real_body(open[i], close[i]);

        // Bullish doji star: bearish long candle[i-1], doji[i] gaps below
        let bull = candle_color(open[i-1], close[i-1]) == -1 &&
            real_body(open[i-1], close[i-1]) > avg_body * BODY_LONG_FACTOR &&
            rb1 <= avg_hl * BODY_DOJI_FACTOR &&
            open[i].max(close[i]) < close[i-1];

        // Bearish doji star: bullish long candle[i-1], doji[i] gaps above
        let bear = candle_color(open[i-1], close[i-1]) == 1 &&
            real_body(open[i-1], close[i-1]) > avg_body * BODY_LONG_FACTOR &&
            rb1 <= avg_hl * BODY_DOJI_FACTOR &&
            open[i].min(close[i]) > close[i-1];

        if bull { out[i] = 100.0; }
        if bear { out[i] = -100.0; }

        hl_sum   += hl_range(high[i], low[i]);
        hl_sum   -= hl_range(high[i - period], low[i - period]);
        body_sum += real_body(open[i-1], close[i-1]);
        body_sum -= real_body(open[i-1-period], close[i-1-period]);
    }
    out
}
