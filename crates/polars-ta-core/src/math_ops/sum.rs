//! SUM — Rolling Summation
//!
//! 滑动窗口求和，时间复杂度 O(n)。
//!
//! Computes the sum of values over a rolling window using a sliding-sum
//! technique. Numerically identical to ta-lib's `TA_SUM`.
//!
//! # Algorithm
//!
//! ```text
//! sum = Σ data[0..period]
//! out[0] = sum
//! for i in 1..out_len:
//!     sum += data[period + i - 1] - data[i - 1]
//!     out[i] = sum
//! ```
//!
//! # Parameters
//!
//! - `data`   — input series
//! - `period` — window length (≥ 1)
//!
//! # Output
//!
//! - Length = `data.len() - (period - 1)` (lookback = period - 1)
//! - Returns empty `Vec` when `data.len() < period`
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::math_ops::sum;
//!
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let result = sum(&data, 3);
//! assert_eq!(result.len(), 3);
//! assert!((result[0] - 6.0).abs() < 1e-10);  // 1+2+3
//! assert!((result[1] - 9.0).abs() < 1e-10);  // 2+3+4
//! assert!((result[2] - 12.0).abs() < 1e-10); // 3+4+5
//! ```

pub fn sum(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let out_len = n - period + 1;
    let mut out = vec![0.0f64; out_len];

    let mut s: f64 = data[..period].iter().sum();
    out[0] = s;
    for i in period..n {
        s += data[i] - data[i - period];
        out[i - period + 1] = s;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sum(&data, 3);
        assert_eq!(result, vec![6.0, 9.0, 12.0]);
    }

    #[test]
    fn sum_boundary_short() {
        assert!(sum(&[1.0, 2.0], 3).is_empty());
    }
}
