//! SUB — Element-wise Subtraction
//!
//! 逐元素减法：`output[i] = real0[i] - real1[i]`
//!
//! Numerically identical to ta-lib's `TA_SUB`. Lookback = 0, output length = input length.
//!
//! # Parameters
//!
//! - `real0` — minuend series
//! - `real1` — subtrahend series (same length as `real0`)
//!
//! # Output
//!
//! - Length = `real0.len()` (lookback = 0)
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::math_ops::sub;
//!
//! let a = vec![5.0, 10.0, 15.0];
//! let b = vec![1.0, 2.0, 3.0];
//! let result = sub(&a, &b);
//! assert_eq!(result, vec![4.0, 8.0, 12.0]);
//! ```

pub fn sub(real0: &[f64], real1: &[f64]) -> Vec<f64> {
    assert_eq!(real0.len(), real1.len());
    real0.iter().zip(real1.iter()).map(|(a, b)| a - b).collect()
}
