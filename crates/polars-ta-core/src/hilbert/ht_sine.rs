//! HT_SINE — Hilbert Transform Sine Wave
//! Returns two vectors: (sine, leadsine).
use super::core::{HtState, ht_step, HT_LOOKBACK};

/// Returns `(sine, leadsine)` starting at index `HT_LOOKBACK`.
pub fn ht_sine(close: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = close.len();
    if n <= HT_LOOKBACK { return (vec![], vec![]); }
    let out_len = n - HT_LOOKBACK;
    let mut sine_out = vec![0.0f64; out_len];
    let mut lead_out = vec![0.0f64; out_len];
    let mut state = HtState::new();
    for i in 0..HT_LOOKBACK {
        let _ = ht_step(close[i], &mut state);
    }
    for i in 0..out_len {
        let (_, phase) = ht_step(close[i + HT_LOOKBACK], &mut state);
        let phase_rad = (phase + 90.0) * std::f64::consts::PI / 180.0;
        sine_out[i] = phase_rad.sin();
        let lead_rad = (phase + 135.0) * std::f64::consts::PI / 180.0;
        lead_out[i] = lead_rad.sin();
    }
    (sine_out, lead_out)
}
