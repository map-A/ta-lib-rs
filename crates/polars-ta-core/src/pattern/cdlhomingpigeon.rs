//! CDLHOMINGPIGEON — Homing Pigeon
//!
//! Bullish reversal: two bearish candles where 1st is long and 2nd is short, contained within 1st.
//!
//! ta-lib candle settings:
//! - BodyLong:  RealBody, period=10, factor=1.0  → 1st candle long (anchor i-1)
//! - BodyShort: RealBody, period=10, factor=1.0  → 2nd candle short ≤ avg (anchor i)
use super::helpers::*;

pub fn cdlhomingpigeon(open: &[f64], _high: &[f64], _low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let lookback = BODY_LONG_PERIOD.max(BODY_SHORT_PERIOD) + 1; // 11
    if n <= lookback {
        return out;
    }

    // BodyLong anchor i-1: TrailingIdx=1, init rb([0..9])
    let mut body_long_sum: f64 = (0..BODY_LONG_PERIOD)
        .map(|j| real_body(open[j], close[j]))
        .sum();

    // BodyShort anchor i: TrailingIdx=1, init rb([1..10])
    let mut body_short_sum: f64 = (1..=BODY_SHORT_PERIOD)
        .map(|j| real_body(open[j], close[j]))
        .sum();
    let mut short_trail = 1usize;

    for (long_trail, i) in (lookback..n).enumerate() {
        let avg_long = body_long_sum / BODY_LONG_PERIOD as f64;
        let avg_short = body_short_sum / BODY_SHORT_PERIOD as f64;

        let is_pattern = candle_color(open[i-1], close[i-1]) == -1 &&               // 1st black
            candle_color(open[i],   close[i])   == -1 &&               // 2nd black
            real_body(open[i-1], close[i-1]) > avg_long  * BODY_LONG_FACTOR  &&  // 1st long
            real_body(open[i],   close[i])   <= avg_short * BODY_SHORT_FACTOR &&  // 2nd short
            open[i]  < open[i-1]  &&                                   // 2nd engulfed by 1st
            close[i] > close[i-1];

        if is_pattern {
            out[i] = 100.0;
        }

        // BodyLong update: add rb(i-1), remove rb(long_trail-1) where trailing started at 0
        body_long_sum +=
            real_body(open[i - 1], close[i - 1]) - real_body(open[long_trail], close[long_trail]);
        // BodyShort update: add rb(i), remove rb(short_trail)
        body_short_sum +=
            real_body(open[i], close[i]) - real_body(open[short_trail], close[short_trail]);
        short_trail += 1;
    }
    out
}
