//! CDLHIKKAKE — Hikkake Pattern
//!
//! Inside bar (bar[i-1] within bar[i-2]) followed by a failed breakout bar[i].
//! Returns ±100 for the setup bar, ±200 for confirmation within 3 bars.
//! New hikkake at same bar overwrites pending confirmation.

pub fn cdlhikkake(_open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = high.len();
    let mut out = vec![0.0f64; n];
    if n < 3 {
        return out;
    }

    // ta-lib lookback = 1+1 = 2 (needs i-2, i-1, i)
    // patternResult and patternIdx track the most recent hikkake setup
    let mut pattern_result: i32 = 0; // ±100
    let mut pattern_idx: usize = 0; // breakout bar index

    for i in 2..n {
        // Check for NEW hikkake first (overwrites pending confirmation)
        if high[i-1] < high[i-2] && low[i-1] > low[i-2] // inside bar: bar[i-1] within bar[i-2]
            && (
                (high[i] < high[i-1] && low[i] < low[i-1]) ||   // bullish breakout
                (high[i] > high[i-1] && low[i] > low[i-1])       // bearish breakout
            )
        {
            pattern_result = if high[i] < high[i - 1] { 100 } else { -100 };
            pattern_idx = i;
            out[i] = pattern_result as f64;
        } else if i <= pattern_idx + 3 && pattern_result != 0 {
            // Confirmation within 3 bars: compare close against inside bar (pattern_idx - 1)
            let inside_high = high[pattern_idx - 1];
            let inside_low = low[pattern_idx - 1];
            if (pattern_result > 0 && close[i] > inside_high)
                || (pattern_result < 0 && close[i] < inside_low)
            {
                let sign = if pattern_result > 0 { 1 } else { -1 };
                out[i] = (pattern_result + 100 * sign) as f64; // ±200
                pattern_idx = 0;
                pattern_result = 0;
            }
        }
    }
    out
}
