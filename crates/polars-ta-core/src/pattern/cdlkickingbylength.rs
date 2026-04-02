//! CDLKICKINGBYLENGTH — Kicking - bull/bear determined by the longer marubozu
//!
//! Same conditions as CDLKICKING but signal direction is determined by the longer-bodied candle.
//!
//! ta-lib candle settings:
//! - BodyLong:       RealBody, period=10, factor=1.0
//! - ShadowVeryShort:HighLow,  period=10, factor=0.1
use super::helpers::*;

pub fn cdlkickingbylength(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let lookback = BODY_LONG_PERIOD.max(SHADOW_VERY_SHORT_PERIOD) + 1; // 11
    if n <= lookback {
        return out;
    }

    // Prev (anchor i-1): init rb/hl([0..9])
    let mut body_prev: f64 = (0..BODY_LONG_PERIOD)
        .map(|j| real_body(open[j], close[j]))
        .sum();
    let mut vshort_prev: f64 = (0..SHADOW_VERY_SHORT_PERIOD)
        .map(|j| hl_range(high[j], low[j]))
        .sum();
    let mut prev_trail = 0usize;
    // Curr (anchor i): init rb/hl([1..10])
    let mut body_curr: f64 = (1..=BODY_LONG_PERIOD)
        .map(|j| real_body(open[j], close[j]))
        .sum();
    let mut vshort_curr: f64 = (1..=SHADOW_VERY_SHORT_PERIOD)
        .map(|j| hl_range(high[j], low[j]))
        .sum();
    let mut curr_trail = 1usize;

    for i in lookback..n {
        let avg_body_p = body_prev / BODY_LONG_PERIOD as f64;
        let avg_vshort_p = vshort_prev / SHADOW_VERY_SHORT_PERIOD as f64;
        let avg_body_c = body_curr / BODY_LONG_PERIOD as f64;
        let avg_vshort_c = vshort_curr / SHADOW_VERY_SHORT_PERIOD as f64;

        let c0 = candle_color(open[i - 1], close[i - 1]);
        let c1 = candle_color(open[i], close[i]);
        let rb0 = real_body(open[i - 1], close[i - 1]);
        let rb1 = real_body(open[i], close[i]);

        let maru_prev = rb0 > avg_body_p * BODY_LONG_FACTOR
            && upper_shadow(open[i - 1], high[i - 1], close[i - 1])
                < avg_vshort_p * SHADOW_VERY_SHORT_FACTOR
            && lower_shadow(open[i - 1], low[i - 1], close[i - 1])
                < avg_vshort_p * SHADOW_VERY_SHORT_FACTOR;
        let maru_curr = rb1 > avg_body_c * BODY_LONG_FACTOR
            && upper_shadow(open[i], high[i], close[i]) < avg_vshort_c * SHADOW_VERY_SHORT_FACTOR
            && lower_shadow(open[i], low[i], close[i]) < avg_vshort_c * SHADOW_VERY_SHORT_FACTOR;

        let gap_up = open[i].min(close[i]) > open[i - 1].max(close[i - 1]);
        let gap_down = open[i].max(close[i]) < open[i - 1].min(close[i - 1]);

        if c0 != c1 && maru_prev && maru_curr
            && (c0 == -1 && gap_up || c0 == 1 && gap_down) {
                // Dominant candle (longer body) determines direction
                let dominant_color = if rb1 > rb0 { c1 } else { c0 };
                out[i] = dominant_color as f64 * 100.0;
            }

        body_prev +=
            real_body(open[i - 1], close[i - 1]) - real_body(open[prev_trail], close[prev_trail]);
        vshort_prev +=
            hl_range(high[i - 1], low[i - 1]) - hl_range(high[prev_trail], low[prev_trail]);
        prev_trail += 1;
        body_curr += real_body(open[i], close[i]) - real_body(open[curr_trail], close[curr_trail]);
        vshort_curr += hl_range(high[i], low[i]) - hl_range(high[curr_trail], low[curr_trail]);
        curr_trail += 1;
    }
    out
}
