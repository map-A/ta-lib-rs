//! HT_DCPHASE — Hilbert Transform Dominant Cycle Phase
use super::core::{HtState, ht_step, HT_LOOKBACK};

/// Returns the dominant cycle phase for each bar starting at index `HT_LOOKBACK`.
pub fn ht_dcphase(close: &[f64]) -> Vec<f64> {
    let n = close.len();
    if n <= HT_LOOKBACK { return vec![]; }
    let out_len = n - HT_LOOKBACK;
    let mut out = vec![0.0f64; out_len];
    let mut state = HtState::new();
    for i in 0..HT_LOOKBACK {
        let _ = ht_step(close[i], &mut state);
    }
    for i in 0..out_len {
        let (_, phase) = ht_step(close[i + HT_LOOKBACK], &mut state);
        out[i] = phase;
    }
    out
}
