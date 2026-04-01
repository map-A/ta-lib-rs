//! HT_PHASOR — Hilbert Transform Phasor Components
//! Returns two vectors: (inphase, quadrature).
use super::core::{HtState, ht_step, HT_LOOKBACK};

/// Returns `(inphase, quadrature)` starting at index `HT_LOOKBACK`.
pub fn ht_phasor(close: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = close.len();
    if n <= HT_LOOKBACK { return (vec![], vec![]); }
    let out_len = n - HT_LOOKBACK;
    let mut inphase = vec![0.0f64; out_len];
    let mut quad = vec![0.0f64; out_len];
    let mut state = HtState::new();
    for i in 0..HT_LOOKBACK {
        let _ = ht_step(close[i], &mut state);
    }
    for i in 0..out_len {
        let _ = ht_step(close[i + HT_LOOKBACK], &mut state);
        inphase[i] = state.i1;
        quad[i] = state.q1;
    }
    (inphase, quad)
}
