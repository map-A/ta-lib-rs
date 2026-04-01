//! Plus Directional Movement (+DM)
//!
//! Wilder-smoothed +DM using high/low only.
//! Lookback = `period - 1`, output length = `n - (period - 1)`.

use crate::oscillator::dm_core::compute_dm_smoothed_hl;

pub fn plus_dm(high: &[f64], low: &[f64], period: usize) -> Vec<f64> {
    match compute_dm_smoothed_hl(high, low, period) {
        Some((plus, _)) => plus,
        None => vec![],
    }
}
