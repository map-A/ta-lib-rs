//! Triple Exponential Moving Average (TEMA)
//!
//! Further reduces lag by combining three EMA passes — numerically identical
//! to ta-lib's `TA_TEMA`.
//!
//! # Algorithm
//!
//! ```text
//! ema1  = EMA(data, period)
//! ema2  = EMA(ema1, period)
//! ema3  = EMA(ema2, period)
//! tema  = 3*ema1 - 3*ema2 + ema3
//! ```
//!
//! All three series are aligned to the same input index before combining.
//!
//! # Parameters
//!
//! - `data`   — input price series
//! - `period` — smoothing window length (≥ 1)
//!
//! # Output
//!
//! - Length = `data.len() - 3 * (period - 1)`
//! - Returns an empty `Vec` when `data.len() < 3 * (period - 1) + 1`
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::tema;
//!
//! let data: Vec<f64> = (1..=30).map(|x| x as f64).collect();
//! let result = tema(&data, 3);
//! // lookback = 3*(3-1) = 6, output length = 30 - 6 = 24
//! assert_eq!(result.len(), 24);
//! ```

use crate::trend::ema::ema;

/// Triple Exponential Moving Average.
///
/// See [module documentation](self) for full details.
pub fn tema(data: &[f64], period: usize) -> Vec<f64> {
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

    let ema3 = ema(&ema2, period);
    if ema3.is_empty() {
        return vec![];
    }

    // ema3 比 ema1 短 2*(period-1)，比 ema2 短 (period-1)
    let lb = period - 1;
    let ema1_aligned = &ema1[2 * lb..];
    let ema2_aligned = &ema2[lb..];

    debug_assert_eq!(ema1_aligned.len(), ema3.len());
    debug_assert_eq!(ema2_aligned.len(), ema3.len());

    let out_len = ema3.len();
    let out: Vec<f64> = ema1_aligned.iter()
        .zip(ema2_aligned.iter())
        .zip(ema3.iter())
        .map(|((&a, &b), &c)| 3.0 * a - 3.0 * b + c)
        .collect();
    out
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
    fn tema_output_length() {
        let period = 3;
        let data: Vec<f64> = (1..=30).map(|x| x as f64).collect();
        let result = tema(&data, period);
        let expected_len = 30 - 3 * (period - 1);
        assert_eq!(result.len(), expected_len);
    }

    #[test]
    fn tema_lookback14() {
        let period = 14;
        let data = vec![1.0f64; 200];
        let result = tema(&data, period);
        assert_eq!(result.len(), 200 - 3 * 13);
    }

    #[test]
    fn tema_constant_series() {
        // 常数序列：TEMA = 常数
        let data = vec![7.0f64; 100];
        let result = tema(&data, 5);
        for &v in &result {
            assert_close(v, 7.0, 1e-10);
        }
    }

    #[test]
    fn tema_boundary_short() {
        // period=3 需要至少 3*(3-1)+1 = 7 个值
        let data = vec![1.0f64; 6];
        assert!(tema(&data, 3).is_empty());
    }

    #[test]
    fn tema_period1() {
        // period=1: lookback=0, TEMA = 3*data - 3*data + data = data
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = tema(&data, 1);
        assert_eq!(result.len(), 5);
        assert_close(result[0], 1.0, 1e-10);
        assert_close(result[4], 5.0, 1e-10);
    }

    #[test]
    fn tema_period_zero() {
        assert!(tema(&[1.0, 2.0, 3.0], 0).is_empty());
    }

    #[test]
    fn tema_empty_input() {
        assert!(tema(&[], 5).is_empty());
    }

    #[test]
    fn tema_single_output() {
        let period = 3;
        // 最小输入：3*(period-1)+1 = 7
        let data: Vec<f64> = (1..=7).map(|x| x as f64).collect();
        let result = tema(&data, period);
        assert_eq!(result.len(), 1);
    }
}
