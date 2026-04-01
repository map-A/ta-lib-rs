//! Minus Directional Indicator (-DI)
//!
//! `-DI = 100 * smoothed_minus_DM / smoothed_TR`
//! Lookback = `period`, output length = `n - period`.

use crate::oscillator::dm_core::compute_dm_tr_smoothed;

pub fn minus_di(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    match compute_dm_tr_smoothed(high, low, close, period) {
        Some((_, minus, tr)) => minus
            .iter()
            .zip(tr.iter())
            .map(|(&m, &t)| if t == 0.0 { 0.0 } else { 100.0 * m / t })
            .collect(),
        None => vec![],
    }
}
