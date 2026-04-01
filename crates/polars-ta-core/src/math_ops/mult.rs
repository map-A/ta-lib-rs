//! MULT — Element-wise Multiplication
//!
//! 逐元素乘法：`output[i] = real0[i] * real1[i]`
//!
//! Numerically identical to ta-lib's `TA_MULT`. Lookback = 0, output length = input length.
//!
//! # Parameters
//!
//! - `real0` — first factor series
//! - `real1` — second factor series (same length as `real0`)
//!
//! # Output
//!
//! - Length = `real0.len()` (lookback = 0)
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::math_ops::mult;
//!
//! let a = vec![2.0, 3.0, 4.0];
//! let b = vec![5.0, 6.0, 7.0];
//! let result = mult(&a, &b);
//! assert_eq!(result, vec![10.0, 18.0, 28.0]);
//! ```

pub fn mult(real0: &[f64], real1: &[f64]) -> Vec<f64> {
    assert_eq!(real0.len(), real1.len());
    real0.iter().zip(real1.iter()).map(|(a, b)| a * b).collect()
}
