//! CDLHIKKAKE — Hikkake Pattern
//! Inside bar followed by a failed breakout. Returns 100/200 (not just 100).
use super::helpers::*;

pub fn cdlhikkake(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    if n < 5 { return out; }

    // Pass 1: find inside bars and mark tentative signals
    // Pass 2: confirm signals up to 3 bars later

    // State: track when we had a bullish or bearish hikkake setup
    let mut bull_hikkake: Option<usize> = None;
    let mut bear_hikkake: Option<usize> = None;

    for i in 1..n {
        // Clear expired setups (more than 3 candles ago)
        if let Some(b) = bull_hikkake {
            if i > b + 3 { bull_hikkake = None; }
        }
        if let Some(b) = bear_hikkake {
            if i > b + 3 { bear_hikkake = None; }
        }

        // Check if current bar confirms a prior hikkake setup
        if let Some(b) = bull_hikkake {
            if close[i] > high[b] && i <= b + 3 {
                // Bullish confirmation: close above the inside bar's high
                // ta-lib returns 100 for 1-bar confirmation, 200 for 2+ bars
                let dist = i - b;
                out[i] = if dist == 1 { 100.0 } else { 200.0 };
                bull_hikkake = None;
            }
        }
        if let Some(b) = bear_hikkake {
            if close[i] < low[b] && i <= b + 3 {
                let dist = i - b;
                out[i] = if dist == 1 { -100.0 } else { -200.0 };
                bear_hikkake = None;
            }
        }

        // Check if this bar is an inside bar (both high and low within prev bar)
        if i >= 2 && out[i] == 0.0 {
            let prev = i - 1;
            let pprev = i - 2;
            // Inside bar: bar[i-1] is inside bar[i-2]
            if high[prev] < high[pprev] && low[prev] > low[pprev] {
                // Now look at current bar: is it a failed breakout?
                // Bullish hikkake: bar[i] breaks below the inside bar then reverses
                if high[i] < high[prev] && low[i] < low[prev] {
                    // Bearish setup: broke lower (potential bullish hikkake signal)
                    bull_hikkake = Some(i);
                } else if low[i] > low[prev] && high[i] > high[prev] {
                    // Bullish setup: broke higher (potential bearish hikkake signal)  
                    bear_hikkake = Some(i);
                }
            }
        }
    }
    out
}
