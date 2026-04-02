//! HT_SINE — Hilbert Transform Sine Wave.
//!
//! Lookback = 63. Returns `(sine, leadsine)` each of length `n`,
//! with `NaN` for the first 63 bars.
use super::core::{run_ht_engine, HT_LOOKBACK_LARGE};

pub fn ht_sine(close: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = close.len();
    let results = run_ht_engine(close, 34);
    let mut sine_out = vec![f64::NAN; n];
    let mut lead_out = vec![f64::NAN; n];
    for bar in HT_LOOKBACK_LARGE..n {
        if let Some(r) = &results[bar] {
            sine_out[bar] = r.sine;
            lead_out[bar] = r.lead_sine;
        }
    }
    (sine_out, lead_out)
}
