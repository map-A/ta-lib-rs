//! HT_DCPERIOD — Hilbert Transform Dominant Cycle Period
use super::core::{HtState, ht_step, HT_LOOKBACK};

/// Returns the dominant cycle period for each bar starting at index `HT_LOOKBACK`.
pub fn ht_dcperiod(close: &[f64]) -> Vec<f64> {
    let n = close.len();
    if n <= HT_LOOKBACK { return vec![]; }
    let out_len = n - HT_LOOKBACK;
    let mut out = vec![0.0f64; out_len];
    let mut state = HtState::new();
    state.period = 0.0;
    // Prime the state with first HT_LOOKBACK values
    for i in 0..HT_LOOKBACK {
        let _ = ht_step(close[i], &mut state);
    }
    for i in 0..out_len {
        let (dc, _) = ht_step(close[i + HT_LOOKBACK], &mut state);
        out[i] = dc;
    }
    out
}
