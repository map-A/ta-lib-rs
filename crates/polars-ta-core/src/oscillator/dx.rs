//! Directional Movement Index (DX)
//!
//! `DX = 100 * |+DI - -DI| / (+DI + -DI)`
//!    = `100 * |s_pdm - s_mdm| / (s_pdm + s_mdm)`
//! Lookback = `period`, output length = `n - period`.

use crate::oscillator::dm_core::compute_dm_tr_smoothed;

pub fn dx(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    match compute_dm_tr_smoothed(high, low, close, period) {
        Some((plus, minus, _tr)) => plus
            .iter()
            .zip(minus.iter())
            .map(|(&p, &m)| {
                let sum = p + m;
                if sum == 0.0 {
                    0.0
                } else {
                    100.0 * (p - m).abs() / sum
                }
            })
            .collect(),
        None => vec![],
    }
}
