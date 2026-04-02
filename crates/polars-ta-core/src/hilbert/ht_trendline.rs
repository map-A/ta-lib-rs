//! HT_TRENDLINE — Hilbert Transform Instantaneous Trendline.
//!
//! Lookback = 63. Returns `Vec<f64>` of length `n` with `NaN` for the first 63 bars.
use super::core::{run_ht_engine, HT_LOOKBACK_LARGE};

pub fn ht_trendline(close: &[f64]) -> Vec<f64> {
    let n = close.len();
    let results = run_ht_engine(close, 34);
    let mut out = vec![f64::NAN; n];
    for bar in HT_LOOKBACK_LARGE..n {
        if let Some(r) = &results[bar] {
            out[bar] = r.trendline;
        }
    }
    out
}
