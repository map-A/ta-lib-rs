//! Plus Directional Indicator (+DI)
//!
//! `+DI = 100 * smoothed_plus_DM / smoothed_TR`
//! Lookback = `period`, output length = `n - period`.

use crate::oscillator::dm_core::compute_dm_tr_smoothed;

pub fn plus_di(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    match compute_dm_tr_smoothed(high, low, close, period) {
        Some((plus, _, tr)) => plus
            .iter()
            .zip(tr.iter())
            .map(|(&p, &t)| if t == 0.0 { 0.0 } else { 100.0 * p / t })
            .collect(),
        None => vec![],
    }
}
