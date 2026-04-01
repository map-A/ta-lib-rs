//! True Range (TRange)
//!
//! Measures the greatest of three price distances to capture gap risk.
//! Numerically identical to ta-lib's `TA_TRANGE`.
//!
//! # Algorithm
//!
//! ```text
//! for i in 1..n:
//!   tr[i] = max(high[i] - low[i],
//!               |high[i] - close[i-1]|,
//!               |low[i]  - close[i-1]|)
//! ```
//!
//! # Parameters
//!
//! - `high`  — high prices
//! - `low`   — low prices
//! - `close` — closing prices
//!
//! # Output
//!
//! - Length = `n - 1` (lookback = 1)
//! - Returns empty `Vec` when `n < 2` or slice lengths differ
//!
//! # NaN Handling
//!
//! NaN in any input propagates via IEEE 754 arithmetic.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::volatility::trange;
//!
//! let high  = vec![10.0, 12.0, 11.0];
//! let low   = vec![8.0,  9.0,  7.0];
//! let close = vec![9.0,  11.0, 10.0];
//! let result = trange(&high, &low, &close);
//! assert_eq!(result.len(), 2); // lookback = 1
//! // tr[0]: max(12-9, |12-9|, |9-9|) = max(3, 3, 0) = 3
//! assert!((result[0] - 3.0).abs() < 1e-10);
//! ```

/// True Range.
///
/// See [module documentation](self) for full details.
pub fn trange(high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = close.len();
    if n < 2 || high.len() != n || low.len() != n {
        return vec![];
    }

    let out_len = n - 1;
    let mut out = vec![0.0f64; out_len];

    // 重新切片使三个数组对齐相同偏移，便于 LLVM 自动向量化
    let h = &high[1..];
    let l = &low[1..];
    let c = &close[..out_len];

    for i in 0..out_len {
        let hi = h[i];
        let li = l[i];
        let ci = c[i];
        let hl = hi - li;
        let hc = (hi - ci).abs();
        let lc = (li - ci).abs();
        out[i] = hl.max(hc).max(lc);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64, eps: f64) {
        assert!(
            (actual - expected).abs() < eps || (actual.is_nan() && expected.is_nan()),
            "actual={actual:.10}, expected={expected:.10}",
        );
    }

    #[test]
    fn trange_basic() {
        let high  = vec![10.0, 12.0, 11.0];
        let low   = vec![8.0,  9.0,  7.0];
        let close = vec![9.0,  11.0, 10.0];
        let result = trange(&high, &low, &close);
        assert_eq!(result.len(), 2);
        // i=1: max(12-9=3, |12-9|=3, |9-9|=0) = 3
        assert_close(result[0], 3.0, 1e-10);
        // i=2: max(11-7=4, |11-11|=0, |7-11|=4) = 4
        assert_close(result[1], 4.0, 1e-10);
    }

    #[test]
    fn trange_no_gap() {
        // 无跳空：TR = high - low
        let high  = vec![10.0, 12.0];
        let low   = vec![8.0,  9.0];
        let close = vec![11.0, 10.0]; // close[0] between high[1]/low[1]
        let result = trange(&high, &low, &close);
        assert_eq!(result.len(), 1);
        // max(12-9=3, |12-11|=1, |9-11|=2) = 3
        assert_close(result[0], 3.0, 1e-10);
    }

    #[test]
    fn trange_gap_up() {
        // 跳空向上：close[i-1] 低于 low[i]
        let high  = vec![5.0, 20.0];
        let low   = vec![3.0, 18.0];
        let close = vec![4.0, 19.0];
        let result = trange(&high, &low, &close);
        assert_eq!(result.len(), 1);
        // max(20-18=2, |20-4|=16, |18-4|=14) = 16
        assert_close(result[0], 16.0, 1e-10);
    }

    #[test]
    fn trange_gap_down() {
        // 跳空向下：close[i-1] 高于 high[i]
        let high  = vec![20.0, 5.0];
        let low   = vec![18.0, 3.0];
        let close = vec![19.0, 4.0];
        let result = trange(&high, &low, &close);
        assert_eq!(result.len(), 1);
        // max(5-3=2, |5-19|=14, |3-19|=16) = 16
        assert_close(result[0], 16.0, 1e-10);
    }

    #[test]
    fn trange_too_short() {
        let v = vec![10.0];
        assert!(trange(&v, &v, &v).is_empty());
        assert!(trange(&[], &[], &[]).is_empty());
    }

    #[test]
    fn trange_length_mismatch() {
        let a = vec![10.0, 11.0];
        let b = vec![9.0];
        assert!(trange(&a, &b, &a).is_empty());
    }

    #[test]
    fn trange_output_length() {
        let n = 50;
        let high:  Vec<f64> = vec![10.0; n];
        let low:   Vec<f64> = vec![8.0; n];
        let close: Vec<f64> = vec![9.0; n];
        let result = trange(&high, &low, &close);
        assert_eq!(result.len(), n - 1);
    }

    #[test]
    fn trange_all_same() {
        // 价格完全相同 → TR = 0
        let v = vec![10.0; 5];
        let result = trange(&v, &v, &v);
        assert_eq!(result.len(), 4);
        for val in &result {
            assert_close(*val, 0.0, 1e-10);
        }
    }
}
