//! HT_DCPERIOD — Hilbert Transform Dominant Cycle Period.
//!
//! Lookback = 32. Returns `Vec<f64>` of length `n` with `NaN` for the first 32 bars.
use super::core::{run_ht_engine, HT_LOOKBACK_SMALL};

pub fn ht_dcperiod(close: &[f64]) -> Vec<f64> {
    let n = close.len();
    let results = run_ht_engine(close, 9);
    let mut out = vec![f64::NAN; n];
    for bar in HT_LOOKBACK_SMALL..n {
        if let Some(r) = &results[bar] {
            out[bar] = r.smooth_period;
        }
    }
    out
}
