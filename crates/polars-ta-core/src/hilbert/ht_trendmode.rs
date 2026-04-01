//! HT_TRENDMODE — Hilbert Transform Trend vs Cycle Mode
//! Returns 1 (trend mode) or 0 (cycle mode) for each bar.
use super::core::{HtState, ht_step, HT_LOOKBACK};

/// Returns `1.0` for trend mode and `0.0` for cycle mode, starting at index `HT_LOOKBACK`.
pub fn ht_trendmode(close: &[f64]) -> Vec<f64> {
    let n = close.len();
    if n <= HT_LOOKBACK { return vec![]; }
    let out_len = n - HT_LOOKBACK;
    let mut out = vec![0.0f64; out_len];
    let mut state = HtState::new();
    for i in 0..HT_LOOKBACK {
        let _ = ht_step(close[i], &mut state);
    }
    let mut prev_phase = 0.0_f64;
    for i in 0..out_len {
        let (_, phase) = ht_step(close[i + HT_LOOKBACK], &mut state);
        // Trend mode when phase is advancing rapidly (Ehlers criterion)
        let delta_phase = prev_phase - phase;
        // Trend if the instantaneous period is near the maximum (slow cycle = trend)
        out[i] = if state.dc_period >= 40.0 || delta_phase.abs() < 1.0 { 1.0 } else { 0.0 };
        prev_phase = phase;
    }
    out
}
