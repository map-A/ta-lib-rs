//! Simple Moving Average (SMA)
//!
//! Computes the arithmetic mean of a rolling window of fixed size.
//! Numerically identical to ta-lib's `TA_SMA`.
//!
//! # Algorithm
//!
//! Uses a sliding-window sum for O(n) time complexity:
//!
//! ```text
//! sum = Σ data[0..period]
//! out[0] = sum / period
//! for i in period..n:
//!     sum += data[i] - data[i - period]
//!     out[i - period + 1] = sum / period
//! ```
//!
//! # Parameters
//!
//! - `data`   — input price series (typically `close`)
//! - `period` — averaging window length (≥ 1)
//!
//! # Output
//!
//! - Length = `data.len() - (period - 1)` = `data.len() - lookback`
//! - Returns an empty `Vec` when `data.len() < period`
//!
//! # NaN Handling
//!
//! Any NaN within the current window propagates to the output value for that
//! window. Values outside the window do not affect unrelated outputs.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::sma;
//!
//! let close = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let result = sma(&close, 3);
//! // lookback = 2, output length = 5 - 2 = 3
//! assert_eq!(result.len(), 3);
//! assert!((result[0] - 2.0).abs() < 1e-10);  // (1+2+3)/3
//! assert!((result[1] - 3.0).abs() < 1e-10);  // (2+3+4)/3
//! assert!((result[2] - 4.0).abs() < 1e-10);  // (3+4+5)/3
//! ```

/// Simple Moving Average.
///
/// See [module documentation](self) for full details.
///
/// # NaN Behavior (ta-lib compatible)
///
/// Uses a simple sliding-window sum. Once a NaN enters the running sum,
/// IEEE 754 arithmetic ensures `NaN ± x = NaN`, so **all subsequent outputs
/// are also NaN** — identical to ta-lib C behavior.
pub fn sma(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }

    let out_len = n - (period - 1);
    let mut out = vec![0.0f64; out_len];
    // 乘法替代除法，避免每次循环的除法开销
    let inv_period = 1.0 / period as f64;

    let mut sum: f64 = (0..period).map(|j| data[j]).sum();
    out[0] = sum * inv_period;

    for i in 1..out_len {
        sum += data[period + i - 1] - data[i - 1];
        out[i] = sum * inv_period;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64, eps: f64) {
        assert!(
            (actual - expected).abs() < eps || (actual.is_nan() && expected.is_nan()),
            "actual={actual:.15}, expected={expected:.15}, diff={:.2e}",
            (actual - expected).abs()
        );
    }

    #[test]
    fn sma_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sma(&data, 3);
        assert_eq!(result.len(), 3);
        assert_close(result[0], 2.0, 1e-10);
        assert_close(result[1], 3.0, 1e-10);
        assert_close(result[2], 4.0, 1e-10);
    }

    #[test]
    fn sma_period_1() {
        let data = vec![1.0, 2.0, 3.0];
        let result = sma(&data, 1);
        // period=1 → lookback=0, output length = input length
        assert_eq!(result.len(), 3);
        assert_close(result[0], 1.0, 1e-10);
        assert_close(result[2], 3.0, 1e-10);
    }

    #[test]
    fn sma_boundary_exact() {
        // 输入长度 = period → 恰好产生 1 个输出值
        let data = vec![1.0, 2.0, 3.0];
        let result = sma(&data, 3);
        assert_eq!(result.len(), 1);
        assert_close(result[0], 2.0, 1e-10);
    }

    #[test]
    fn sma_boundary_short() {
        // 输入长度 < period → 返回空 Vec
        let data = vec![1.0, 2.0];
        let result = sma(&data, 3);
        assert!(result.is_empty());
    }

    #[test]
    fn sma_empty_input() {
        let result = sma(&[], 5);
        assert!(result.is_empty());
    }

    #[test]
    fn sma_period_zero() {
        let data = vec![1.0, 2.0, 3.0];
        let result = sma(&data, 0);
        assert!(result.is_empty());
    }

    #[test]
    fn sma_with_nan() {
        // ta-lib 行为：NaN 一旦进入滑动累加和，后续全部永久污染
        let data = vec![1.0, f64::NAN, 3.0, 4.0, 5.0];
        let result = sma(&data, 3);
        assert_eq!(result.len(), 3);
        // 初始窗口 [1, NaN, 3] → sum=NaN → output NaN
        assert!(result[0].is_nan());
        // [NaN, 3, 4] → sum = NaN + 4 - 1 = NaN → NaN
        assert!(result[1].is_nan());
        // [3, 4, 5] → sum = NaN + 5 - NaN = NaN（NaN 减 NaN 仍为 NaN）
        assert!(
            result[2].is_nan(),
            "ta-lib compatible: NaN permanently contaminates the sum"
        );
    }

    #[test]
    fn sma_all_same_value() {
        let data = vec![100.0f64; 1000];
        let result = sma(&data, 20);
        assert_eq!(result.len(), 981);
        for v in &result {
            assert_close(*v, 100.0, 1e-10);
        }
    }

    #[test]
    fn sma_lookback_period() {
        // lookback = period - 1
        let period = 14;
        let lookback = period - 1;
        let data = vec![1.0f64; 100];
        let result = sma(&data, period);
        assert_eq!(result.len(), 100 - lookback);
    }
}
