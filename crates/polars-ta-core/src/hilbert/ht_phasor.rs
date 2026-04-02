//! HT_PHASOR — Hilbert Transform Phasor Components.
//!
//! Lookback = 32. Returns `(inphase, quadrature)` each of length `n`,
//! with `NaN` for the first 32 bars.
use super::core::{run_ht_engine, HT_LOOKBACK_SMALL};

pub fn ht_phasor(close: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = close.len();
    let results = run_ht_engine(close, 9);
    let mut inphase = vec![f64::NAN; n];
    let mut quadrature = vec![f64::NAN; n];
    for bar in HT_LOOKBACK_SMALL..n {
        if let Some(r) = &results[bar] {
            inphase[bar] = r.i1;
            quadrature[bar] = r.q1;
        }
    }
    (inphase, quadrature)
}
