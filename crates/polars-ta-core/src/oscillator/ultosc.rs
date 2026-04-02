//! Ultimate Oscillator
//!
//! Combines three time frames to reduce volatility and false trading signals.
//! Numerically identical to ta-lib's `TA_ULTOSC`.
//!
//! # Algorithm
//!
//! ```text
//! BP[i]  = close[i] - min(low[i], close[i-1])         (Buying Pressure)
//! TR[i]  = max(high[i], close[i-1]) - min(low[i], close[i-1])  (True Range)
//! avg_p  = sum(BP, p) / sum(TR, p)   for each period p
//! UO = 100 * (4*avg1 + 2*avg2 + avg3) / 7
//! ```
//!
//! # Parameters
//!
//! - `high`    — high price series
//! - `low`     — low price series
//! - `close`   — close price series
//! - `period1` — first (shortest) period
//! - `period2` — second period
//! - `period3` — third (longest) period
//!
//! # Output
//!
//! - `lookback = max(period1, period2, period3) - 1`
//! - Length = `n - lookback - 1` (need prev_close so one extra element)
//! - Returns empty `Vec` when input is too short
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::oscillator::ultosc;
//!
//! let n = 30_usize;
//! let high:  Vec<f64> = (0..n).map(|i| i as f64 + 1.0).collect();
//! let low:   Vec<f64> = (0..n).map(|i| i as f64).collect();
//! let close: Vec<f64> = (0..n).map(|i| i as f64 + 0.5).collect();
//! let result = ultosc(&high, &low, &close, 7, 14, 28);
//! assert!(!result.is_empty());
//! ```

/// Ultimate Oscillator.
///
/// See [module documentation](self) for full details.
pub fn ultosc(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period1: usize,
    period2: usize,
    period3: usize,
) -> Vec<f64> {
    let n = close.len();
    if period1 == 0 || period2 == 0 || period3 == 0 {
        return vec![];
    }
    if high.len() != n || low.len() != n {
        return vec![];
    }

    let max_period = period1.max(period2).max(period3);
    let lookback = max_period;
    if n <= lookback {
        return vec![];
    }

    // 预计算 BP 和 TR（共 n-1 个值，bp[k] 对应原始下标 k+1）
    let raw = n - 1;
    let mut bp = Vec::with_capacity(raw);
    let mut tr = Vec::with_capacity(raw);
    for i in 1..n {
        let pc = close[i - 1];
        let c = close[i];
        let h = high[i];
        let l = low[i];
        let bp_val = if c.is_nan() || l.is_nan() || pc.is_nan() {
            f64::NAN
        } else {
            c - l.min(pc)
        };
        let tr_val = if h.is_nan() || l.is_nan() || pc.is_nan() {
            f64::NAN
        } else {
            h.max(pc) - l.min(pc)
        };
        bp.push(bp_val);
        tr.push(tr_val);
    }

    // 滑动窗口：O(n) 总复杂度，三个周期各维护一对 (bp_sum, tr_sum)
    let (mut bs1, mut ts1) = (0.0_f64, 0.0_f64);
    let (mut bs2, mut ts2) = (0.0_f64, 0.0_f64);
    let (mut bs3, mut ts3) = (0.0_f64, 0.0_f64);

    let out_len = n - lookback;
    let mut out = Vec::with_capacity(out_len);

    for j in 0..raw {
        // 滑入新值
        bs1 += bp[j];
        ts1 += tr[j];
        bs2 += bp[j];
        ts2 += tr[j];
        bs3 += bp[j];
        ts3 += tr[j];
        // 滑出过期值（窗口超出 period 大小时移除最老元素）
        if j >= period1 {
            let k = j - period1;
            bs1 -= bp[k];
            ts1 -= tr[k];
        }
        if j >= period2 {
            let k = j - period2;
            bs2 -= bp[k];
            ts2 -= tr[k];
        }
        if j >= period3 {
            let k = j - period3;
            bs3 -= bp[k];
            ts3 -= tr[k];
        }

        // 当三个窗口均已填满（j >= max_period-1）时开始输出
        if j >= max_period - 1 {
            let avg1 = if ts1 != 0.0 { bs1 / ts1 } else { 0.0 };
            let avg2 = if ts2 != 0.0 { bs2 / ts2 } else { 0.0 };
            let avg3 = if ts3 != 0.0 { bs3 / ts3 } else { 0.0 };
            out.push(100.0 * (4.0 * avg1 + 2.0 * avg2 + avg3) / 7.0);
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
    fn ultosc_output_length() {
        let n = 50_usize;
        let high: Vec<f64> = (0..n).map(|i| i as f64 + 1.0).collect();
        let low: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let close: Vec<f64> = (0..n).map(|i| i as f64 + 0.5).collect();
        let result = ultosc(&high, &low, &close, 7, 14, 28);
        // lookback = max(7,14,28) = 28, so output len = 50 - 28 = 22
        assert_eq!(result.len(), 22);
    }

    #[test]
    fn ultosc_too_short() {
        let h = vec![1.0; 10];
        let l = vec![0.0; 10];
        let c = vec![0.5; 10];
        assert!(ultosc(&h, &l, &c, 7, 14, 28).is_empty());
    }

    #[test]
    fn ultosc_period_zero() {
        let h = vec![1.0; 50];
        let l = vec![0.0; 50];
        let c = vec![0.5; 50];
        assert!(ultosc(&h, &l, &c, 0, 14, 28).is_empty());
        assert!(ultosc(&h, &l, &c, 7, 0, 28).is_empty());
        assert!(ultosc(&h, &l, &c, 7, 14, 0).is_empty());
    }

    #[test]
    fn ultosc_range() {
        let n = 60_usize;
        let high: Vec<f64> = (0..n)
            .map(|i| (i as f64 * 0.5).sin() * 10.0 + 51.0)
            .collect();
        let low: Vec<f64> = (0..n)
            .map(|i| (i as f64 * 0.5).sin() * 10.0 + 49.0)
            .collect();
        let close: Vec<f64> = (0..n)
            .map(|i| (i as f64 * 0.5).sin() * 10.0 + 50.0)
            .collect();
        let result = ultosc(&high, &low, &close, 7, 14, 28);
        for v in &result {
            assert!(v.is_finite(), "UO not finite: {v}");
        }
    }

    #[test]
    fn ultosc_symmetric_periods() {
        // When all periods are equal, avg1 == avg2 == avg3
        // → UO = 100 * (4+2+1) * avg / 7 = 100 * avg
        let n = 30_usize;
        let high: Vec<f64> = (0..n).map(|i| i as f64 + 1.0).collect();
        let low: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let close: Vec<f64> = (0..n).map(|i| i as f64 + 0.5).collect();
        let res = ultosc(&high, &low, &close, 5, 5, 5);
        // lookback = 5, out_len = 25
        assert_eq!(res.len(), 25);
        for v in &res {
            assert!(v.is_finite());
        }
    }

    #[test]
    fn ultosc_mismatched_lengths() {
        let h = vec![10.0; 50];
        let l = vec![8.0; 49]; // mismatch
        let c = vec![9.0; 50];
        assert!(ultosc(&h, &l, &c, 7, 14, 28).is_empty());
    }
}
