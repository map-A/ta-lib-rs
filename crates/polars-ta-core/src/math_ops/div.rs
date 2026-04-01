//! DIV — Element-wise Division
//!
//! 逐元素除法：`output[i] = real0[i] / real1[i]`
//!
//! Numerically identical to ta-lib's `TA_DIV`. Lookback = 0, output length = input length.
//! Division by zero follows IEEE 754: `x / 0.0 = ±Inf`, `0.0 / 0.0 = NaN`.
//!
//! # Parameters
//!
//! - `real0` — numerator series
//! - `real1` — denominator series (same length as `real0`)
//!
//! # Output
//!
//! - Length = `real0.len()` (lookback = 0)
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::math_ops::div;
//!
//! let a = vec![10.0, 20.0, 30.0];
//! let b = vec![2.0, 4.0, 5.0];
//! let result = div(&a, &b);
//! assert_eq!(result, vec![5.0, 5.0, 6.0]);
//! ```

pub fn div(real0: &[f64], real1: &[f64]) -> Vec<f64> {
    assert_eq!(real0.len(), real1.len());
    real0.iter().zip(real1.iter()).map(|(a, b)| a / b).collect()
}
