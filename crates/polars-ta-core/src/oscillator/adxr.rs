//! Average Directional Movement Index Rating (ADXR)
//!
//! `ADXR[i] = (ADX[i + period - 1] + ADX[i]) / 2`
//! Lookback = `3 * period - 2`, output length = `n - (3 * period - 2)`.

use crate::trend::adx::adx;

pub fn adxr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    if period == 0 {
        return vec![];
    }
    let adx_vals = adx(high, low, close, period);
    let adx_len = adx_vals.len();
    if adx_len < period {
        return vec![];
    }
    let out_len = adx_len - (period - 1);
    (0..out_len)
        .map(|i| (adx_vals[i + period - 1] + adx_vals[i]) / 2.0)
        .collect()
}
