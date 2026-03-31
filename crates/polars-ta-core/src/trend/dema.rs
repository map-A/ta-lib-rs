//! Double Exponential Moving Average (DEMA)
//!
//! Reduces lag by combining two EMAs — numerically identical to ta-lib's
//! `TA_DEMA`.
//!
//! # Algorithm
//!
//! ```text
//! ema1  = EMA(data, period)
//! ema2  = EMA(ema1, period)
//! dema  = 2 * ema1 - ema2
//! ```
//!
//! `ema2` is (period - 1) shorter than `ema1`.  Both are aligned to the same
//! input position before the subtraction.
//!
//! # Parameters
//!
//! - `data`   — input price series
//! - `period` — smoothing window length (≥ 1)
//!
//! # Output
//!
//! - Length = `data.len() - 2 * (period - 1)`
//! - Returns an empty `Vec` when `data.len() < 2 * (period - 1) + 1`
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::dema;
//!
//! let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();
//! let result = dema(&data, 3);
//! // lookback = 2*(3-1) = 4, output length = 20 - 4 = 16
//! assert_eq!(result.len(), 16);
//! ```

use crate::trend::ema::ema;

/// Double Exponential Moving Average.
///
/// See [module documentation](self) for full details.
pub fn dema(data: &[f64], period: usize) -> Vec<f64> {
    if period == 0 {
        return vec![];
    }

    let ema1 = ema(data, period);
    if ema1.is_empty() {
        return vec![];
    }

    let ema2 = ema(&ema1, period);
    if ema2.is_empty() {
        return vec![];
    }

    // ema2 比 ema1 短 (period - 1)，需要对齐
    let lookback1 = period - 1;
    let ema1_aligned = &ema1[lookback1..];

    debug_assert_eq!(ema1_aligned.len(), ema2.len());

    ema1_aligned
        .iter()
        .zip(ema2.iter())
        .map(|(&e1, &e2)| 2.0 * e1 - e2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64, eps: f64) {
        assert!(
            (actual - expected).abs() < eps || (actual.is_nan() && expected.is_nan()),
            "actual={actual:.15}, expected={expected:.15}",
        );
    }

    #[test]
    fn dema_output_length() {
        let period = 3;
        let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        let result = dema(&data, period);
        let expected_len = 20 - 2 * (period - 1);
        assert_eq!(result.len(), expected_len);
    }

    #[test]
    fn dema_lookback14() {
        let period = 14;
        let data = vec![1.0f64; 100];
        let result = dema(&data, period);
        assert_eq!(result.len(), 100 - 2 * 13);
    }

    #[test]
    fn dema_constant_series() {
        // 常数序列：DEMA = 常数
        let data = vec![5.0f64; 50];
        let result = dema(&data, 5);
        for &v in &result {
            assert_close(v, 5.0, 1e-10);
        }
    }

    #[test]
    fn dema_boundary_short() {
        let data = vec![1.0, 2.0, 3.0];
        // period=3 需要至少 2*(3-1)+1 = 5 个值
        assert!(dema(&data, 3).is_empty());
    }

    #[test]
    fn dema_period1() {
        // period=1: lookback=0, DEMA = 2*EMA - EMA = EMA = data
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = dema(&data, 1);
        assert_eq!(result.len(), 5);
        assert_close(result[0], 1.0, 1e-10);
        assert_close(result[4], 5.0, 1e-10);
    }

    #[test]
    fn dema_period_zero() {
        assert!(dema(&[1.0, 2.0, 3.0], 0).is_empty());
    }

    #[test]
    fn dema_empty_input() {
        assert!(dema(&[], 5).is_empty());
    }

    #[test]
    fn dema_single_output() {
        let period = 3;
        // 最小输入：2*(period-1)+1 = 5
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = dema(&data, period);
        assert_eq!(result.len(), 1);
    }
}
