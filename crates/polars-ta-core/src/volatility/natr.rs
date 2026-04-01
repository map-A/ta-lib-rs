//! Normalized Average True Range (NATR)
//!
//! ATR expressed as a percentage of the closing price.
//! Numerically identical to ta-lib's `TA_NATR`.
//!
//! # Algorithm
//!
//! ```text
//! natr[i] = atr[i] / close[period + i] * 100
//! ```
//!
//! Where `close[period + i]` corresponds to the closing price at the same
//! bar as the ATR output (first ATR output aligns with `close[period]`).
//!
//! # Parameters
//!
//! - `high`   — high prices
//! - `low`    — low prices
//! - `close`  — closing prices
//! - `period` — averaging window (≥ 1)
//!
//! # Output
//!
//! - Length = `n - period` (lookback = period, same as ATR)
//! - Returns empty `Vec` when `n <= period` or `period == 0`
//!
//! # NaN Handling
//!
//! NaN in any input propagates via IEEE 754 arithmetic. Division by zero
//! (when `close == 0`) produces `±Inf` or `NaN` per IEEE 754.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::volatility::natr;
//!
//! let high  = vec![10.0, 12.0, 11.0, 13.0, 12.0];
//! let low   = vec![8.0,  9.0,  8.0,  10.0, 9.0];
//! let close = vec![9.0,  11.0, 10.0, 12.0, 11.0];
//! let result = natr(&high, &low, &close, 3);
//! assert_eq!(result.len(), 2);
//! ```

use super::atr::atr;

/// Normalized Average True Range.
///
/// See [module documentation](self) for full details.
pub fn natr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    let n = close.len();
    if period == 0 || n <= period || high.len() != n || low.len() != n {
        return vec![];
    }

    let atr_vals = atr(high, low, close, period);
    if atr_vals.is_empty() {
        return vec![];
    }

    let out_len = atr_vals.len();
    let mut out = Vec::with_capacity(out_len);
    // Safety: atr_vals has out_len elements. close has n elements and n > period (checked above),
    // so close[period..n] has n-period = out_len elements. Both src pointers advance out_len times.
    // dst advances out_len times within the allocation.
    unsafe {
        out.set_len(out_len);
        let mut atr_ptr = atr_vals.as_ptr();
        let mut close_ptr = close.as_ptr().add(period);
        let mut dst = out.as_mut_ptr();
        for _ in 0..out_len {
            *dst = *atr_ptr / *close_ptr * 100.0;
            atr_ptr = atr_ptr.add(1);
            close_ptr = close_ptr.add(1);
            dst = dst.add(1);
        }
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
    fn natr_output_length() {
        let n = 20;
        let high:  Vec<f64> = vec![10.0; n];
        let low:   Vec<f64> = vec![8.0; n];
        let close: Vec<f64> = vec![9.0; n];
        let result = natr(&high, &low, &close, 14);
        assert_eq!(result.len(), n - 14);
    }

    #[test]
    fn natr_constant_range() {
        // TR = high - low = 2, close = 10, NATR = 2/10*100 = 20%
        let n = 20;
        let high:  Vec<f64> = vec![11.0; n];
        let low:   Vec<f64> = vec![9.0; n];
        let close: Vec<f64> = vec![10.0; n];
        let result = natr(&high, &low, &close, 5);
        assert_eq!(result.len(), n - 5);
        for v in &result {
            assert_close(*v, 20.0, 1e-10);
        }
    }

    #[test]
    fn natr_positive_values() {
        let high  = vec![10.0, 12.0, 11.0, 13.0, 12.0, 14.0];
        let low   = vec![8.0,  9.0,  8.0,  10.0, 9.0,  11.0];
        let close = vec![9.0,  11.0, 10.0, 12.0, 11.0, 13.0];
        let result = natr(&high, &low, &close, 3);
        for v in &result {
            assert!(*v >= 0.0, "NATR should be non-negative, got {v}");
        }
    }

    #[test]
    fn natr_too_short() {
        let v = vec![10.0; 5];
        assert!(natr(&v, &v, &v, 5).is_empty());
        assert!(natr(&v, &v, &v, 14).is_empty());
    }

    #[test]
    fn natr_period_zero() {
        let v = vec![10.0; 20];
        assert!(natr(&v, &v, &v, 0).is_empty());
    }

    #[test]
    fn natr_is_atr_over_close_times_100() {
        use super::super::atr::atr;

        let high  = vec![10.0, 12.0, 11.0, 13.0, 12.0, 14.0];
        let low   = vec![8.0,  9.0,  8.0,  10.0, 9.0,  11.0];
        let close = vec![9.0,  11.0, 10.0, 12.0, 11.0, 13.0];
        let period = 3;

        let atr_vals = atr(&high, &low, &close, period);
        let natr_vals = natr(&high, &low, &close, period);

        assert_eq!(atr_vals.len(), natr_vals.len());
        for (i, (a, n)) in atr_vals.iter().zip(natr_vals.iter()).enumerate() {
            let expected = a / close[period + i] * 100.0;
            assert_close(*n, expected, 1e-10);
        }
    }
}
