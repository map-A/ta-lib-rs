//! CDLHARAMICROSS — Harami Cross Pattern
//!
//! Large candle followed by a doji contained within it.
//!
//! ta-lib candle settings:
//! - BodyLong: RealBody, period=10, factor=1.0 — anchor at i-1
//! - BodyDoji: HighLow,  period=10, factor=0.1 — anchor at i
use super::helpers::*;

pub fn cdlharamicross(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let lookback = BODY_LONG_PERIOD.max(BODY_DOJI_PERIOD) + 1;
    if n <= lookback {
        return out;
    }

    // BodyLong anchor i-1: TrailingIdx = lookback-1-BODY_LONG_PERIOD = 0
    // init while i < lookback-1 → bars [0..lookback-2] = [0..9]
    let mut body_long_sum: f64 = (0..BODY_LONG_PERIOD)
        .map(|j| real_body(open[j], close[j]))
        .sum();
    let mut hl_doji_sum: f64 = (1..=BODY_DOJI_PERIOD)
        .map(|j| hl_range(high[j], low[j]))
        .sum();
    let mut doji_trail = 1usize;

    for (long_trail, i) in (lookback..n).enumerate() {
        let avg_long = body_long_sum / BODY_LONG_PERIOD as f64;
        let avg_doji = hl_doji_sum / BODY_DOJI_PERIOD as f64;
        let rb_prev = real_body(open[i - 1], close[i - 1]);
        let rb_curr = real_body(open[i], close[i]);

        let hi_prev = open[i - 1].max(close[i - 1]);
        let lo_prev = open[i - 1].min(close[i - 1]);
        let hi_curr = open[i].max(close[i]);
        let lo_curr = open[i].min(close[i]);

        if rb_prev > avg_long * BODY_LONG_FACTOR && rb_curr <= avg_doji * BODY_DOJI_FACTOR {
            if hi_curr < hi_prev && lo_curr > lo_prev {
                // strict containment → ±100
                out[i] = -(candle_color(open[i - 1], close[i - 1]) as f64) * 100.0;
            } else if hi_curr <= hi_prev && lo_curr >= lo_prev {
                // partial containment (one end may touch) → ±80
                out[i] = -(candle_color(open[i - 1], close[i - 1]) as f64) * 80.0;
            }
        }

        // Update: BodyLong anchor i-1 → add rb(i-1), remove trailing
        body_long_sum +=
            real_body(open[i - 1], close[i - 1]) - real_body(open[long_trail], close[long_trail]);
        // Update: BodyDoji anchor i → add hl(i), remove trailing
        hl_doji_sum += hl_range(high[i], low[i]) - hl_range(high[doji_trail], low[doji_trail]);
        doji_trail += 1;
    }
    out
}
