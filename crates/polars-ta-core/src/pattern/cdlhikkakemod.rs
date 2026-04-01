//! CDLHIKKAKEMOD — Modified Hikkake Pattern
use super::helpers::*;

pub fn cdlhikkakemod(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    if n < 5 { return out; }
    let period = NEAR_PERIOD;
    if n <= period + 4 { return out; }

    let mut near = RollingAvg::new(period);
    for j in 0..period { near.push(hl_range(high[j], low[j])); }

    let mut bull_setup: Option<usize> = None;
    let mut bear_setup: Option<usize> = None;

    for i in (period + 1)..n {
        near.push(hl_range(high[i-1], low[i-1]));
        let avg_near = near.sum / period as f64;

        // Expire old setups
        if let Some(b) = bull_setup { if i > b + 3 { bull_setup = None; } }
        if let Some(b) = bear_setup { if i > b + 3 { bear_setup = None; } }

        // Confirm setups
        if let Some(b) = bull_setup {
            if i <= b + 3 && close[i] > high[b-1] {
                out[i] = 100.0;
                bull_setup = None;
                continue;
            }
        }
        if let Some(b) = bear_setup {
            if i <= b + 3 && close[i] < low[b-1] {
                out[i] = -100.0;
                bear_setup = None;
                continue;
            }
        }

        // Detect new modified hikkake setup (inside bar at i-2, i-1, then failed break at i)
        if i >= 3 {
            let p2 = i - 2; let p1 = i - 1;
            // bar[p1] must be inside bar[p2]
            if high[p1] < high[p2] && low[p1] > low[p2] {
                // Modified: bar[i] (current) is within range of bar[p1] ± near
                if high[i] < high[p1] && low[i] < low[p1] && high[i] > high[p1] - avg_near {
                    bull_setup = Some(i);
                } else if low[i] > low[p1] && high[i] > high[p1] && low[i] < low[p1] + avg_near {
                    bear_setup = Some(i);
                }
            }
        }
    }
    out
}
