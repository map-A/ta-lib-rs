//! ADD — Element-wise Addition
//!
//! 逐元素加法：`output[i] = real0[i] + real1[i]`
//!
//! Numerically identical to ta-lib's `TA_ADD`. Lookback = 0, output length = input length.
//! Both input slices must have the same length (asserted at runtime).
//!
//! # Parameters
//!
//! - `real0` — first input series
//! - `real1` — second input series (same length as `real0`)
//!
//! # Output
//!
//! - Length = `real0.len()` (lookback = 0)
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::math_ops::add;
//!
//! let a = vec![1.0, 2.0, 3.0];
//! let b = vec![4.0, 5.0, 6.0];
//! let result = add(&a, &b);
//! assert_eq!(result, vec![5.0, 7.0, 9.0]);
//! ```

pub fn add(real0: &[f64], real1: &[f64]) -> Vec<f64> {
    assert_eq!(real0.len(), real1.len());
    real0.iter().zip(real1.iter()).map(|(a, b)| a + b).collect()
}
