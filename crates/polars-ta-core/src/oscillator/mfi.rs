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

    let inv3 = 1.0 / 3.0;
    let out_len = n - period;
    let mut out = vec![0.0f64; out_len];

    // 环形缓冲区大小恰好为 period，使用 if 重置（分支预测器
    // 每 period 次才触发一次，成本可忽略；避免了 next_power_of_two
    // 导致 cap > period 时槽位未初始化被错误读为 0 的 bug）
    let mut pos_ring = vec![0.0f64; period];
    let mut neg_ring = vec![0.0f64; period];
    let mut pos_sum = 0.0f64;
    let mut neg_sum = 0.0f64;
    let mut ring_head = 0usize;

    // tp[0] 没有前驱，不产生 money flow
    let mut tp_prev = (high[0] + low[0] + close[0]) * inv3;

    // 预热：填充窗口 [1..=period]
    for i in 1..=period {
        let t = (high[i] + low[i] + close[i]) * inv3;
        let raw = t * volume[i];
        let sign = (t > tp_prev) as i64 - (t < tp_prev) as i64;
        let signed = raw * sign as f64;
        let pm = signed.max(0.0);
        let nm = (-signed).max(0.0);
        pos_ring[i - 1] = pm;
        neg_ring[i - 1] = nm;
        pos_sum += pm;
        neg_sum += nm;
        tp_prev = t;
    }

    out[0] = compute_mfi(pos_sum, neg_sum);

    // 滑动循环：替换最旧元素，纯加减
    for i in 0..out_len - 1 {
        let j = i + period + 1;
        let t = (high[j] + low[j] + close[j]) * inv3;
        let raw = t * volume[j];
        let sign = (t > tp_prev) as i64 - (t < tp_prev) as i64;
        let signed = raw * sign as f64;
        let pm = signed.max(0.0);
        let nm = (-signed).max(0.0);

        let old_pos = pos_ring[ring_head];
        let old_neg = neg_ring[ring_head];
        pos_ring[ring_head] = pm;
        neg_ring[ring_head] = nm;
        ring_head += 1;
        if ring_head >= period {
            ring_head = 0;
        }

        pos_sum += pm - old_pos;
        neg_sum += nm - old_neg;
        out[i + 1] = compute_mfi(pos_sum, neg_sum);
        tp_prev = t;
    }

    out
}

#[inline]
fn compute_mfi(pos_sum: f64, neg_sum: f64) -> f64 {
    // 代数等价：100 - 100/(1+pos/neg) = 100*pos/(pos+neg)
    // 单次除法；total=0 时返回 0（覆盖 pos=0 和 neg=0 双零情况）
    let total = pos_sum + neg_sum;
    if total == 0.0 {
        0.0
    } else {
        100.0 * pos_sum / total
    }
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
        let h: Vec<f64> = (0..n)
            .map(|i| (i as f64 * 0.4).sin() * 5.0 + 52.0)
            .collect();
        let l: Vec<f64> = (0..n)
            .map(|i| (i as f64 * 0.4).sin() * 5.0 + 48.0)
            .collect();
        let c: Vec<f64> = (0..n)
            .map(|i| (i as f64 * 0.4).sin() * 5.0 + 50.0)
            .collect();
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
