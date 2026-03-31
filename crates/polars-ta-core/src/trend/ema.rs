//! Exponential Moving Average (EMA)
//!
//! Computes the exponentially weighted moving average, seeded with an SMA on
//! the first `period` values — numerically identical to ta-lib's `TA_EMA`.
//!
//! # Algorithm
//!
//! ```text
//! k   = 2 / (period + 1)
//! ema[0] = SMA(data[0..period])          // SMA seed
//! ema[i] = data[period + i] * k + ema[i-1] * (1 - k)
//! ```
//!
//! # Parameters
//!
//! - `data`   — input price series (typically `close`)
//! - `period` — smoothing window length (≥ 1)
//!
//! # Output
//!
//! - Length = `data.len() - (period - 1)` = `data.len() - lookback`
//! - Returns an empty `Vec` when `data.len() < period`
//!
//! # NaN Handling
//!
//! Once the SMA seed includes a NaN, every subsequent EMA value is NaN
//! (IEEE 754 arithmetic, identical to ta-lib C behavior).
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::ema;
//!
//! let close = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let result = ema(&close, 3);
//! // lookback = 2, output length = 3
//! assert_eq!(result.len(), 3);
//! // seed = SMA([1,2,3]) = 2.0
//! // ema[1] = 4.0 * 0.5 + 2.0 * 0.5 = 3.0
//! // ema[2] = 5.0 * 0.5 + 3.0 * 0.5 = 4.0
//! assert!((result[0] - 2.0).abs() < 1e-10);
//! assert!((result[1] - 3.0).abs() < 1e-10);
//! assert!((result[2] - 4.0).abs() < 1e-10);
//! ```

/// Exponential Moving Average.
///
/// See [module documentation](self) for full details.
pub fn ema(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }

    let lookback = period - 1;
    let out_len = n - lookback;
    let mut out = Vec::with_capacity(out_len);

    let k = 2.0 / (period as f64 + 1.0);
    let k1 = 1.0 - k;

    // SMA 种子：前 period 个值的均值
    let seed: f64 = data[..period].iter().sum::<f64>() / period as f64;
    out.push(seed);

    let mut prev = seed;
    for &v in &data[period..] {
        let cur = v * k + prev * k1;
        out.push(cur);
        prev = cur;
    }

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
    fn ema_basic_period3() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ema(&data, 3);
        // k = 2/(3+1) = 0.5
        // seed = (1+2+3)/3 = 2.0
        // ema[1] = 4*0.5 + 2*0.5 = 3.0
        // ema[2] = 5*0.5 + 3*0.5 = 4.0
        assert_eq!(result.len(), 3);
        assert_close(result[0], 2.0, 1e-10);
        assert_close(result[1], 3.0, 1e-10);
        assert_close(result[2], 4.0, 1e-10);
    }

    #[test]
    fn ema_period1() {
        let data = vec![1.0, 2.0, 3.0];
        let result = ema(&data, 1);
        // k = 2/2 = 1.0; each output equals the input
        assert_eq!(result.len(), 3);
        assert_close(result[0], 1.0, 1e-10);
        assert_close(result[1], 2.0, 1e-10);
        assert_close(result[2], 3.0, 1e-10);
    }

    #[test]
    fn ema_boundary_exact() {
        let data = vec![2.0, 4.0, 6.0];
        let result = ema(&data, 3);
        // 一个输出值：SMA seed = 4.0
        assert_eq!(result.len(), 1);
        assert_close(result[0], 4.0, 1e-10);
    }

    #[test]
    fn ema_boundary_short() {
        let data = vec![1.0, 2.0];
        let result = ema(&data, 3);
        assert!(result.is_empty());
    }

    #[test]
    fn ema_empty_input() {
        assert!(ema(&[], 3).is_empty());
    }

    #[test]
    fn ema_period_zero() {
        assert!(ema(&[1.0, 2.0], 0).is_empty());
    }

    #[test]
    fn ema_with_nan_in_seed() {
        let data = vec![1.0, f64::NAN, 3.0, 4.0, 5.0];
        let result = ema(&data, 3);
        // seed includes NaN → all outputs are NaN
        assert_eq!(result.len(), 3);
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        assert!(result[2].is_nan());
    }

    #[test]
    fn ema_lookback() {
        let period = 14;
        let data = vec![1.0f64; 100];
        let result = ema(&data, period);
        assert_eq!(result.len(), 100 - (period - 1));
    }

    #[test]
    fn ema_constant_series() {
        let data = vec![5.0f64; 50];
        let result = ema(&data, 10);
        for &v in &result {
            assert_close(v, 5.0, 1e-10);
        }
    }
}
