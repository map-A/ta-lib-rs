//! SUB — element-wise subtraction of two real series.
//!
//! `output[i] = real0[i] - real1[i]`
//! Output length = `n` (lookback = 0).

pub fn sub(real0: &[f64], real1: &[f64]) -> Vec<f64> {
    assert_eq!(real0.len(), real1.len());
    real0.iter().zip(real1.iter()).map(|(a, b)| a - b).collect()
}
