//! Commodity Channel Index (CCI)
//!
//! Measures the deviation of a typical price from its moving average,
//! normalized by mean deviation. Numerically identical to ta-lib's `TA_CCI`.
//!
//! # Algorithm
//!
//! ```text
//! tp[i] = (high[i] + low[i] + close[i]) / 3
//! sma_tp = SMA(tp, period)
//! mean_dev = mean(|tp[i] - sma_tp[i]|) over window
//! CCI = (tp - sma_tp) / (0.015 * mean_dev)
//! ```
//!
//! # Parameters
//!
//! - `high`   — high price series
//! - `low`    — low price series
//! - `close`  — close price series
//! - `period` — window length
//!
//! # Output
//!
//! - Length = `n - (period - 1)`
//! - Returns empty `Vec` when input is too short
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::oscillator::cci;
//!
//! let high  = vec![10.0, 11.0, 12.0, 11.0, 10.0];
//! let low   = vec![ 8.0,  9.0, 10.0,  9.0,  8.0];
//! let close = vec![ 9.0, 10.0, 11.0, 10.0,  9.0];
//! let result = cci(&high, &low, &close, 3);
//! assert_eq!(result.len(), 3);
//! ```

/// Commodity Channel Index.
///
/// See [module documentation](self) for full details.
pub fn cci(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    let n = close.len();
    if period == 0 || n < period || high.len() != n || low.len() != n {
        return vec![];
    }

    let out_len = n - (period - 1);
    let mut out = Vec::with_capacity(out_len);

    // 循环缓冲区存储 TP 值，避免重复计算；滑动窗口维护 SMA
    let mut tp_buf = vec![0.0_f64; period];
    let mut tp_sum = 0.0_f64;

    // 预热：填充前 period-1 个值
    for i in 0..period - 1 {
        let tp = (high[i] + low[i] + close[i]) / 3.0;
        tp_buf[i % period] = tp;
        tp_sum += tp;
    }

    for i in (period - 1)..n {
        let tp = (high[i] + low[i] + close[i]) / 3.0;
        // 滑出最旧值（若旧值为 NaN 则移除后重新累加，避免 NaN 污染 sum）
        let oldest_slot = i % period;
        let oldest_val = tp_buf[oldest_slot];
        tp_buf[oldest_slot] = tp;
        if oldest_val.is_nan() || tp_sum.is_nan() {
            tp_sum = tp_buf.iter().sum::<f64>();
        } else {
            tp_sum = tp_sum - oldest_val + tp;
        }

        let sma = tp_sum / period as f64;

        // 均值偏差（inherently O(period) per step）
        let mut dev_sum = 0.0_f64;
        for k in 0..period {
            dev_sum += (tp_buf[k] - sma).abs();
        }
        let mean_dev = dev_sum / period as f64;

        out.push(if mean_dev == 0.0 {
            0.0
        } else {
            (tp - sma) / (0.015 * mean_dev)
        });
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
    fn cci_output_length() {
        let n = 20_usize;
        let h = vec![10.0_f64; n];
        let l = vec![8.0_f64; n];
        let c = vec![9.0_f64; n];
        let result = cci(&h, &l, &c, 14);
        assert_eq!(result.len(), n - 13);
    }

    #[test]
    fn cci_too_short() {
        let h = vec![10.0, 11.0];
        let l = vec![8.0, 9.0];
        let c = vec![9.0, 10.0];
        assert!(cci(&h, &l, &c, 5).is_empty());
    }

    #[test]
    fn cci_period_zero() {
        let h = vec![10.0; 10];
        let l = vec![8.0; 10];
        let c = vec![9.0; 10];
        assert!(cci(&h, &l, &c, 0).is_empty());
    }

    #[test]
    fn cci_flat_is_zero() {
        // When all values are the same, tp = sma → CCI = 0 (mean_dev = 0 guard)
        let h = vec![10.0_f64; 20];
        let l = vec![10.0_f64; 20];
        let c = vec![10.0_f64; 20];
        let result = cci(&h, &l, &c, 5);
        assert_eq!(result.len(), 16);
        for v in &result {
            assert_close(*v, 0.0, 1e-10);
        }
    }

    #[test]
    fn cci_basic() {
        let high = vec![10.0, 11.0, 12.0, 11.0, 10.0];
        let low = vec![8.0, 9.0, 10.0, 9.0, 8.0];
        let close = vec![9.0, 10.0, 11.0, 10.0, 9.0];
        let result = cci(&high, &low, &close, 3);
        assert_eq!(result.len(), 3);
        // Values should be finite
        for v in &result {
            assert!(v.is_finite());
        }
    }

    #[test]
    fn cci_mismatched_lengths() {
        let h = vec![10.0; 10];
        let l = vec![8.0; 9]; // mismatch
        let c = vec![9.0; 10];
        assert!(cci(&h, &l, &c, 5).is_empty());
    }
}
