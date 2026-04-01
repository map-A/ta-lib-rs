//! CDLGAPSIDESIDEWHITE — Up/Down-Gap Side-By-Side White Lines
//!
//! ta-lib source: lookback = max(Near, Equal) + 2 = 7
//! Conditions:
//!   - BOTH bar[i-1] AND bar[i] have real-body gap up (or down) from bar[i-2]
//!   - bar[i-1] and bar[i] are both white (bullish)
//!   - body sizes are near: |body[i] - body[i-1]| < avg_HL[i-1..i-5] * NEAR_FACTOR (0.2)
//!   - opens are equal:  |open[i] - open[i-1]| < avg_HL[i-1..i-5] * EQUAL_FACTOR (0.05)
//! Both thresholds use the same sliding window anchored at i-1.
use super::helpers::*;

#[inline]
fn real_body_gap_up(o1: f64, c1: f64, o2: f64, c2: f64) -> bool {
    o1.min(c1) > o2.max(c2)
}

#[inline]
fn real_body_gap_down(o1: f64, c1: f64, o2: f64, c2: f64) -> bool {
    o1.max(c1) < o2.min(c2)
}

pub fn cdlgapsidesidewhite(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let lookback = NEAR_PERIOD + 2; // = 7
    if n <= lookback { return out; }

    // ta-lib init: NearTrailingIdx = startIdx - NEAR_PERIOD = lookback - NEAR_PERIOD = 2
    // init loop: i from 2..lookback (2,3,4,5,6), accumulates HL[i-1] = HL[1..5]
    let mut near_sum: f64 = (1..=NEAR_PERIOD).map(|j| hl_range(high[j], low[j])).sum();
    let mut near_trailing: usize = 2;

    for i in lookback..n {
        let avg = near_sum / NEAR_PERIOD as f64;
        let rb1 = real_body(open[i-1], close[i-1]);
        let rb2 = real_body(open[i], close[i]);

        // Both i-1 and i must have real body gap from i-2
        let gap_up = real_body_gap_up(open[i-1], close[i-1], open[i-2], close[i-2])
                  && real_body_gap_up(open[i], close[i], open[i-2], close[i-2]);
        let gap_dn = real_body_gap_down(open[i-1], close[i-1], open[i-2], close[i-2])
                  && real_body_gap_down(open[i], close[i], open[i-2], close[i-2]);
        let both_white = candle_color(open[i-1], close[i-1]) == 1
                      && candle_color(open[i], close[i]) == 1;
        let body_sim = (rb2 - rb1).abs() < avg * NEAR_FACTOR;
        let open_sim = (open[i] - open[i-1]).abs() < avg * EQUAL_FACTOR;

        if (gap_up || gap_dn) && both_white && body_sim && open_sim {
            out[i] = if gap_up { 100.0 } else { -100.0 };
        }

        // Update rolling sum (after condition check, matching ta-lib update order)
        near_sum += hl_range(high[i-1], low[i-1])
                  - hl_range(high[near_trailing - 1], low[near_trailing - 1]);
        near_trailing += 1;
    }
    out
}
