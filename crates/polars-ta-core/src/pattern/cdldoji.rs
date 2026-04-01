//! CDLDOJI — Doji
//! Candle with very small body relative to high-low range.
use super::helpers::*;

pub fn cdldoji(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_DOJI_PERIOD;
    let lookback = period;
    if n <= lookback { return out; }

    // Average of HL range (preceding period candles)
    let mut hl_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_hl = hl_sum / period as f64;
        if real_body(open[i], close[i]) <= avg_hl * BODY_DOJI_FACTOR {
            out[i] = 100.0;  // ta-lib CDLDOJI returns 100 for any doji (no direction)
        }
        hl_sum += hl_range(high[i], low[i]);
        hl_sum -= hl_range(high[i - period], low[i - period]);
    }
    out
}
