//! Money Flow Index (MFI)
//!
//! A volume-weighted momentum oscillator similar to RSI.
//! Numerically identical to ta-lib's `TA_MFI`.
//!
//! # Algorithm
//!
//! ```text
//! tp[i]      = (high[i] + low[i] + close[i]) / 3
//! raw_mf[i]  = tp[i] * volume[i]
//! pos_mf[i]  = raw_mf[i] if tp[i] > tp[i-1], else 0
//! neg_mf[i]  = raw_mf[i] if tp[i] < tp[i-1], else 0
//! For each window of `period` (raw_mf values from index 1..period):
//!   mfi = 100 - 100 / (1 + pos_sum / neg_sum)
//! ```
//!
//! # Parameters
//!
//! - `high`   — high price series
//! - `low`    — low price series
//! - `close`  — close price series
//! - `volume` — volume series
//! - `period` — rolling window length
//!
//! # Output
//!
//! - Length = `n - period`
//! - Returns empty `Vec` when input is too short
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::oscillator::mfi;
//!
//! let n = 20_usize;
//! let high:   Vec<f64> = (0..n).map(|i| i as f64 + 1.0).collect();
//! let low:    Vec<f64> = (0..n).map(|i| i as f64).collect();
//! let close:  Vec<f64> = (0..n).map(|i| i as f64 + 0.5).collect();
//! let volume: Vec<f64> = vec![1000.0; n];
//! let result = mfi(&high, &low, &close, &volume, 14);
//! assert_eq!(result.len(), n - 14);
//! ```

/// Money Flow Index.
///
/// See [module documentation](self) for full details.
pub fn mfi(high: &[f64], low: &[f64], close: &[f64], volume: &[f64], period: usize) -> Vec<f64> {
    let n = close.len();
    if period == 0 || n <= period {
        return vec![];
    }
    if high.len() != n || low.len() != n || volume.len() != n {
        return vec![];
    }

    // 仅保留 typical price（用于 tp[i-1] 比较），money flow 通过 tp[i]*volume[i] 内联计算
    // 相比原实现减少 3 个中间 Vec（raw_mf, pos_mf, neg_mf），提升缓存局部性
    let mut tp = Vec::with_capacity(n);
    for i in 0..n {
        tp.push((high[i] + low[i] + close[i]) / 3.0);
    }

    let out_len = n - period;
    let mut out = Vec::with_capacity(out_len);

    // 初始化第一个窗口：原始索引 1..=period 的正负 money flow
    let mut pos_sum = 0.0_f64;
    let mut neg_sum = 0.0_f64;
    for i in 1..=period {
        let raw = tp[i] * volume[i];
        if tp[i] > tp[i - 1] {
            pos_sum += raw;
        } else if tp[i] < tp[i - 1] {
            neg_sum += raw;
        }
    }
    out.push(compute_mfi(pos_sum, neg_sum));

    // 滑动窗口
    for i in (period + 1)..n {
        let out_i = i - period;
        // 移出旧元素
        let out_raw = tp[out_i] * volume[out_i];
        if tp[out_i] > tp[out_i - 1] {
            pos_sum -= out_raw;
        } else if tp[out_i] < tp[out_i - 1] {
            neg_sum -= out_raw;
        }
        // 移入新元素
        let in_raw = tp[i] * volume[i];
        if tp[i] > tp[i - 1] {
            pos_sum += in_raw;
        } else if tp[i] < tp[i - 1] {
            neg_sum += in_raw;
        }
        // 浮点误差保护：防止累计减法产生微小负值
        pos_sum = pos_sum.max(0.0);
        neg_sum = neg_sum.max(0.0);
        out.push(compute_mfi(pos_sum, neg_sum));
    }

    out
}

#[inline]
fn compute_mfi(pos_sum: f64, neg_sum: f64) -> f64 {
    if pos_sum == 0.0 {
        return 0.0;
    }
    if neg_sum == 0.0 {
        return 100.0;
    }
    100.0 - 100.0 / (1.0 + pos_sum / neg_sum)
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
    fn mfi_output_length() {
        let n = 20_usize;
        let h = vec![10.0_f64; n];
        let l = vec![8.0_f64; n];
        let c = vec![9.0_f64; n];
        let v = vec![1000.0_f64; n];
        let result = mfi(&h, &l, &c, &v, 14);
        assert_eq!(result.len(), n - 14);
    }

    #[test]
    fn mfi_too_short() {
        let h = vec![10.0; 5];
        let l = vec![8.0; 5];
        let c = vec![9.0; 5];
        let v = vec![100.0; 5];
        assert!(mfi(&h, &l, &c, &v, 14).is_empty());
    }

    #[test]
    fn mfi_period_zero() {
        let h = vec![10.0; 20];
        let l = vec![8.0; 20];
        let c = vec![9.0; 20];
        let v = vec![100.0; 20];
        assert!(mfi(&h, &l, &c, &v, 0).is_empty());
    }

    #[test]
    fn mfi_all_up() {
        // Strictly increasing TP → all positive money flow → MFI = 100
        let n = 20_usize;
        let h: Vec<f64> = (0..n).map(|i| i as f64 + 2.0).collect();
        let l: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let c: Vec<f64> = (0..n).map(|i| i as f64 + 1.0).collect();
        let v = vec![1000.0_f64; n];
        let result = mfi(&h, &l, &c, &v, 14);
        assert_eq!(result.len(), 6);
        for val in &result {
            assert_close(*val, 100.0, 1e-10);
        }
    }

    #[test]
    fn mfi_all_down() {
        // Strictly decreasing TP → all negative money flow → MFI = 0
        let n = 20_usize;
        let h: Vec<f64> = (0..n).map(|i| (100 - i) as f64 + 2.0).collect();
        let l: Vec<f64> = (0..n).map(|i| (100 - i) as f64).collect();
        let c: Vec<f64> = (0..n).map(|i| (100 - i) as f64 + 1.0).collect();
        let v = vec![1000.0_f64; n];
        let result = mfi(&h, &l, &c, &v, 14);
        assert_eq!(result.len(), 6);
        for val in &result {
            assert_close(*val, 0.0, 1e-10);
        }
    }

    #[test]
    fn mfi_range() {
        let n = 30_usize;
        let h: Vec<f64> = (0..n).map(|i| (i as f64 * 0.4).sin() * 5.0 + 52.0).collect();
        let l: Vec<f64> = (0..n).map(|i| (i as f64 * 0.4).sin() * 5.0 + 48.0).collect();
        let c: Vec<f64> = (0..n).map(|i| (i as f64 * 0.4).sin() * 5.0 + 50.0).collect();
        let v = vec![1000.0_f64; n];
        let result = mfi(&h, &l, &c, &v, 14);
        for val in &result {
            assert!(*val >= 0.0 && *val <= 100.0, "MFI out of range: {val}");
        }
    }

    #[test]
    fn mfi_mismatched_lengths() {
        let h = vec![10.0; 20];
        let l = vec![8.0; 19]; // mismatch
        let c = vec![9.0; 20];
        let v = vec![100.0; 20];
        assert!(mfi(&h, &l, &c, &v, 14).is_empty());
    }
}
