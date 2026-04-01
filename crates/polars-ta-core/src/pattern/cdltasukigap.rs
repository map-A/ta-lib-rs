//! CDLTASUKIGAP — Tasuki Gap
//! Continuation: two same-color candles with real body gap, then opposite candle partially filling the gap.
use super::helpers::*;

pub fn cdltasukigap(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let near_period = NEAR_PERIOD;   // 5
    let lookback = near_period + 2;  // 7
    if n <= lookback { return out; }

    // NearTrailingIdx = startIdx - near_period = 7 - 5 = 2
    // Init: i from 2..7 (while i < startIdx=7), accumulate hl_range[i-1]
    let near_trailing_start = lookback - near_period;  // 2
    let mut near_sum: f64 = (near_trailing_start..lookback)
        .map(|j| hl_range(high[j-1], low[j-1])).sum();
    let mut near_trailing = near_trailing_start;

    for i in lookback..n {
        let avg_near = near_sum / near_period as f64 * NEAR_FACTOR;

        let rb1 = real_body(open[i-1], close[i-1]);
        let rb2 = real_body(open[i], close[i]);
        let c1 = candle_color(open[i-1], close[i-1]);
        let c2 = candle_color(open[i], close[i]);

        // REALBODYGAPUP(i-1,i-2): min(o,c)[i-1] > max(o,c)[i-2]
        let gap_up = open[i-1].min(close[i-1]) > open[i-2].max(close[i-2]);
        // REALBODYGAPDOWN(i-1,i-2): max(o,c)[i-1] < min(o,c)[i-2]
        let gap_dn = open[i-1].max(close[i-1]) < open[i-2].min(close[i-2]);

        let bull = gap_up
            && c1 == 1
            && c2 == -1
            && open[i] < close[i-1] && open[i] > open[i-1]
            && close[i] < open[i-1]
            && close[i] > open[i-2].max(close[i-2])
            && (rb1 - rb2).abs() < avg_near;

        let bear = gap_dn
            && c1 == -1
            && c2 == 1
            && open[i] < open[i-1] && open[i] > close[i-1]
            && close[i] > open[i-1]
            && close[i] < open[i-2].min(close[i-2])
            && (rb1 - rb2).abs() < avg_near;

        if bull { out[i] = 100.0; }
        else if bear { out[i] = -100.0; }

        near_sum += hl_range(high[i-1], low[i-1]) - hl_range(high[near_trailing-1], low[near_trailing-1]);
        near_trailing += 1;
    }
    out
}
