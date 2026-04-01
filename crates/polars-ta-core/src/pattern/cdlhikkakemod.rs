//! CDLHIKKAKEMOD — Modified Hikkake Pattern
//!
//! Two consecutive inside bars, then a failed breakout whose second bar's close is near the bottom/top.
//! Returns ±100 for setup bar, ±200 for confirmation within 3 bars.
//!
//! ta-lib candle settings:
//! - Near: HighLow, period=5, factor=0.2  — checks close[i-2] near low/high of bar[i-2]
use super::helpers::*;

pub fn cdlhikkakemod(_open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = high.len();
    let mut out = vec![0.0f64; n];
    // lookback = NEAR_PERIOD + 3 = 5 + 3 = 8
    let lookback = NEAR_PERIOD + 3;
    if n <= lookback { return out; }

    // Near rolling sum: HL values at position i-2, 5-bar window
    // NearTrailingIdx = startIdx - 3 - NEAR_PERIOD = 0 (when startIdx=8)
    // Init: while i < startIdx-3=5, accumulate HL[i-2] (negative indices treated as 0)
    let near_trailing_start: usize = 0; // max(startIdx-3-NEAR_PERIOD, 0)
    let mut near_sum: f64 = (near_trailing_start..lookback.saturating_sub(3))
        .map(|j| {
            if j < 2 { 0.0 } else { hl_range(high[j-2], low[j-2]) }
        })
        .sum();
    let mut near_trail = near_trailing_start;

    // Pre-roll: scan i=startIdx-3..startIdx to detect patterns before output range
    // (matches ta-lib's second init while loop)
    let mut pattern_result: i32 = 0;
    let mut pattern_idx: usize = 0;

    // Process bars from first possible pattern bar
    let scan_start = 3usize; // need i-3 to be valid
    for i in scan_start..n {
        // Compute avg_near using HL at i-2
        let avg_near = near_sum / NEAR_PERIOD as f64;

        // Check for new MODIFIED hikkake setup (overwrites pending confirmation)
        let is_setup =
            high[i-2] < high[i-3] && low[i-2] > low[i-3] &&       // bar[i-2] inside bar[i-3]
            high[i-1] < high[i-2] && low[i-1] > low[i-2] &&       // bar[i-1] inside bar[i-2]
            (
                // Bullish: breakout bar goes lower AND 2nd bar (i-2) close near low
                (high[i] < high[i-1] && low[i] < low[i-1] &&
                 close[i-2] <= low[i-2] + avg_near * NEAR_FACTOR)
                ||
                // Bearish: breakout bar goes higher AND 2nd bar (i-2) close near high
                (high[i] > high[i-1] && low[i] > low[i-1] &&
                 close[i-2] >= high[i-2] - avg_near * NEAR_FACTOR)
            );

        if is_setup {
            pattern_result = if high[i] < high[i-1] { 100 } else { -100 };
            pattern_idx = i;
            if i >= lookback {
                out[i] = pattern_result as f64;
            }
        } else if i <= pattern_idx + 3 && pattern_result != 0 {
            // Confirmation: close beyond inside bar (pattern_idx-1 = second inside bar)
            let inside_high = high[pattern_idx - 1];
            let inside_low  = low[pattern_idx - 1];
            let confirmed = (pattern_result > 0 && close[i] > inside_high)
                         || (pattern_result < 0 && close[i] < inside_low);
            if confirmed {
                if i >= lookback {
                    let sign = if pattern_result > 0 { 1 } else { -1 };
                    out[i] = (pattern_result + 100 * sign) as f64; // ±200
                }
                pattern_idx = 0;
                pattern_result = 0;
            }
        }

        // Update rolling sum: add HL[i-2], remove HL[near_trail-2]
        let add = hl_range(high[i-2], low[i-2]);
        let sub = if near_trail >= 2 { hl_range(high[near_trail-2], low[near_trail-2]) } else { 0.0 };
        near_sum += add - sub;
        near_trail += 1;
    }
    out
}
