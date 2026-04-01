//! Weighted Moving Average (WMA)
//!
//! Computes a linearly weighted moving average where the most recent value has
//! the highest weight — numerically identical to ta-lib's `TA_WMA`.
//!
//! # Algorithm
//!
//! ```text
//! denom = period * (period + 1) / 2
//! wma[i] = Σ_{j=0}^{period-1}  data[i - j] * (period - j)  /  denom
//! ```
//!
//! The most recent data point receives weight `period`; the oldest receives `1`.
//!
//! Sliding update (O(n) total):
//! ```text
//! wma_sum_new = wma_sum - running_sum + period * data[i]
//! running_sum_new = running_sum + data[i] - data[i - period]
//! ```
//!
//! # Parameters
//!
//! - `data`   — input price series
//! - `period` — window length (≥ 1)
//!
//! # Output
//!
//! - Length = `data.len() - (period - 1)`
//! - Returns an empty `Vec` when `data.len() < period`
//!
//! # NaN Handling
//!
//! IEEE 754 arithmetic: any NaN in the sliding sums permanently propagates
//! to all subsequent outputs (identical to ta-lib C behavior).
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::wma;
//!
//! let data = vec![1.0, 2.0, 3.0, 4.0];
//! let result = wma(&data, 3);
//! // denom = 3*4/2 = 6
//! // wma[0] = (3*3 + 2*2 + 1*1) / 6 = 14/6 ≈ 2.333
//! // wma[1] = (4*3 + 3*2 + 2*1) / 6 = 20/6 ≈ 3.333
//! assert_eq!(result.len(), 2);
//! assert!((result[0] - 14.0/6.0).abs() < 1e-10);
//! assert!((result[1] - 20.0/6.0).abs() < 1e-10);
//! ```

/// Weighted Moving Average.
///
/// See [module documentation](self) for full details.
pub fn wma(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }

    let lookback = period - 1;
    let out_len = n - lookback;
    // 乘法替代除法，避免每次循环的除法开销
    let inv_denom = 1.0 / (period * (period + 1) / 2) as f64;

    // 初始化第一个窗口的加权和与普通和
    let mut wma_sum = 0.0f64;
    let mut running_sum = 0.0f64;
    for (j, &v) in data[..period].iter().enumerate() {
        let weight = (j + 1) as f64; // weight 1, 2, ..., period
        wma_sum += v * weight;
        running_sum += v;
    }

    let mut out = vec![0.0f64; out_len];

    out[0] = wma_sum * inv_denom;

    for i in 1..out_len {
        let new_val = data[lookback + i];
        let old_val = data[i - 1];
        wma_sum = wma_sum - running_sum + new_val * period as f64;
        running_sum += new_val - old_val;
        out[i] = wma_sum * inv_denom;
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
    fn wma_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let result = wma(&data, 3);
        // denom = 6
        // wma[0] = (1*1 + 2*2 + 3*3)/6 = 14/6
        // wma[1] = (2*1 + 3*2 + 4*3)/6 = 20/6
        assert_eq!(result.len(), 2);
        assert_close(result[0], 14.0 / 6.0, 1e-10);
        assert_close(result[1], 20.0 / 6.0, 1e-10);
    }

    #[test]
    fn wma_period1() {
        // period=1: wma = data itself
        let data = vec![1.0, 2.0, 3.0];
        let result = wma(&data, 1);
        assert_eq!(result.len(), 3);
        assert_close(result[0], 1.0, 1e-10);
        assert_close(result[1], 2.0, 1e-10);
        assert_close(result[2], 3.0, 1e-10);
    }

    #[test]
    fn wma_boundary_exact() {
        let data = vec![1.0, 2.0, 3.0];
        let result = wma(&data, 3);
        // (1*1 + 2*2 + 3*3)/6 = 14/6
        assert_eq!(result.len(), 1);
        assert_close(result[0], 14.0 / 6.0, 1e-10);
    }

    #[test]
    fn wma_boundary_short() {
        let data = vec![1.0, 2.0];
        assert!(wma(&data, 3).is_empty());
    }

    #[test]
    fn wma_empty() {
        assert!(wma(&[], 3).is_empty());
    }

    #[test]
    fn wma_period_zero() {
        assert!(wma(&[1.0, 2.0], 0).is_empty());
    }

    #[test]
    fn wma_constant_series() {
        let data = vec![5.0f64; 50];
        let result = wma(&data, 10);
        assert_eq!(result.len(), 41);
        for &v in &result {
            assert_close(v, 5.0, 1e-10);
        }
    }

    #[test]
    fn wma_lookback() {
        let period = 14;
        let data = vec![1.0f64; 100];
        let result = wma(&data, period);
        assert_eq!(result.len(), 100 - (period - 1));
    }

    #[test]
    fn wma_with_nan() {
        let data = vec![1.0, f64::NAN, 3.0, 4.0, 5.0];
        let result = wma(&data, 3);
        assert_eq!(result.len(), 3);
        // NaN 在初始窗口内：全部污染
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        assert!(result[2].is_nan());
    }
}
