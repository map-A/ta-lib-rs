//! Average True Range (ATR)
//!
//! Smoothed average of True Range using Wilder smoothing.
//! Numerically identical to ta-lib's `TA_ATR`.
//!
//! # Algorithm
//!
//! ```text
//! 1. tr[i] = TrueRange(high[i], low[i], close[i-1])  for i in 1..n
//! 2. Initial ATR = SMA(tr[0..period])   (mean of first `period` TR values)
//! 3. For i > period (i.e., tr index >= period):
//!      atr[j] = (atr[j-1] * (period - 1) + tr[period + j - 1]) / period
//! ```
//!
//! Wilder smoothing uses alpha = 1/period (equivalent to EMA with period `2p-1`).
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
//! - Length = `n - period` (lookback = period)
//! - Returns empty `Vec` when `n <= period` or `period == 0`
//!
//! # NaN Handling
//!
//! NaN in any input propagates via IEEE 754 arithmetic.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::volatility::atr;
//!
//! let high  = vec![10.0, 12.0, 11.0, 13.0, 12.0];
//! let low   = vec![8.0,  9.0,  8.0,  10.0, 9.0];
//! let close = vec![9.0,  11.0, 10.0, 12.0, 11.0];
//! let result = atr(&high, &low, &close, 3);
//! // lookback = 3, output length = 5 - 3 = 2
//! assert_eq!(result.len(), 2);
//! ```

use super::trange::trange;

/// Average True Range.
///
/// See [module documentation](self) for full details.
pub fn atr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    let n = close.len();
    if period == 0 || n <= period || high.len() != n || low.len() != n {
        return vec![];
    }

    // TR 序列长度 = n - 1（从 index 1 开始）
    let tr = trange(high, low, close);
    if tr.len() < period {
        return vec![];
    }

    let out_len = n - period;
    let mut out = Vec::with_capacity(out_len);

    // 种子：前 period 个 TR 的算术平均
    let seed: f64 = tr[..period].iter().sum::<f64>() / period as f64;

    // 预计算 Wilder 平滑系数，避免循环内重复运算
    let p = period as f64;
    let k_wilder = (p - 1.0) / p; // = 1 - 1/period
    let inv_p = 1.0 / p;

    // Safety: tr has tr.len() >= period elements (checked above).
    // tr[period..] has tr.len()-period = out_len-1 elements; src advances exactly that many times.
    // dst starts at out[0] and advances out_len times within the allocation.
    unsafe {
        out.set_len(out_len);
        let dst_base = out.as_mut_ptr();
        *dst_base = seed;

        let mut prev = seed;
        let mut src = tr.as_ptr().add(period);
        let mut dst = dst_base.add(1);
        let end = tr.as_ptr().add(tr.len());
        while src < end {
            let cur = prev * k_wilder + *src * inv_p;
            *dst = cur;
            prev = cur;
            src = src.add(1);
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
    fn atr_output_length() {
        let n = 20;
        let high:  Vec<f64> = vec![10.0; n];
        let low:   Vec<f64> = vec![8.0; n];
        let close: Vec<f64> = vec![9.0; n];
        let result = atr(&high, &low, &close, 14);
        assert_eq!(result.len(), n - 14);
    }

    #[test]
    fn atr_constant_range() {
        // 相邻 bar 高低差恒定 = 2，无跳空 → ATR = 2
        let n = 20;
        let high:  Vec<f64> = (0..n).map(|i| 10.0 + i as f64).collect();
        let low:   Vec<f64> = (0..n).map(|i| 8.0 + i as f64).collect();
        let close: Vec<f64> = (0..n).map(|i| 9.0 + i as f64).collect();
        let result = atr(&high, &low, &close, 5);
        assert_eq!(result.len(), n - 5);
        for v in &result {
            assert_close(*v, 2.0, 1e-10);
        }
    }

    #[test]
    fn atr_positive_values() {
        // ATR 必须为非负数
        let high  = vec![10.0, 12.0, 11.0, 13.0, 12.0, 14.0];
        let low   = vec![8.0,  9.0,  8.0,  10.0, 9.0,  11.0];
        let close = vec![9.0,  11.0, 10.0, 12.0, 11.0, 13.0];
        let result = atr(&high, &low, &close, 3);
        assert_eq!(result.len(), 3);
        for v in &result {
            assert!(*v >= 0.0, "ATR should be non-negative, got {v}");
        }
    }

    #[test]
    fn atr_too_short() {
        let high  = vec![10.0, 11.0, 12.0];
        let low   = vec![8.0,  9.0,  10.0];
        let close = vec![9.0,  10.0, 11.0];
        // n=3, period=3 → output length = n - period = 0
        assert!(atr(&high, &low, &close, 3).is_empty());
        assert!(atr(&high, &low, &close, 14).is_empty());
    }

    #[test]
    fn atr_period_zero() {
        let v = vec![10.0; 20];
        assert!(atr(&v, &v, &v, 0).is_empty());
    }

    #[test]
    fn atr_period_one() {
        // period=1 → seed = TR[0], Wilder 退化为直通
        // ATR = TR 本身
        let high  = vec![10.0, 12.0, 11.0, 13.0];
        let low   = vec![8.0,  9.0,  8.0,  10.0];
        let close = vec![9.0,  11.0, 10.0, 12.0];
        let result = atr(&high, &low, &close, 1);
        // period=1, lookback=1, output len = 4-1 = 3
        // TR: [max(3,3,0)=3, max(3,1,2)=3, max(3,1,2)=3]
        // ATR with period=1: (prev*0 + tr) / 1 = tr itself
        assert_eq!(result.len(), 3);
        for v in &result {
            assert!(*v >= 0.0);
        }
    }

    #[test]
    fn atr_wilder_smoothing_formula() {
        // 手动验证 Wilder 平滑公式
        // TR 为 [2, 2, 2, 4] with period=3
        // seed = (2+2+2)/3 = 2.0
        // next: (2*(3-1) + 4) / 3 = 8/3 ≈ 2.6667
        let high  = vec![10.0, 12.0, 12.0, 12.0, 14.0];
        let low   = vec![8.0,  10.0, 10.0, 10.0, 10.0];
        let close = vec![9.0,  11.0, 11.0, 11.0, 13.0];
        // TR[0]=max(2,|12-9|,|10-9|)=3, TR[1]=max(2,1,1)=2, TR[2]=max(2,1,1)=2, TR[3]=max(4,3,1)=4
        let result = atr(&high, &low, &close, 3);
        assert_eq!(result.len(), 2);
        // seed = (3+2+2)/3 = 7/3
        let seed = 7.0 / 3.0;
        assert_close(result[0], seed, 1e-10);
        // next = (seed * 2 + 4) / 3
        let next = (seed * 2.0 + 4.0) / 3.0;
        assert_close(result[1], next, 1e-10);
    }
}
