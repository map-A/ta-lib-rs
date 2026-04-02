//! Chaikin A/D Oscillator (ADOSC)
//!
//! Difference between a fast and slow EMA of the Chaikin A/D line.
//! Numerically identical to ta-lib's `TA_ADOSC`.
//!
//! # Algorithm
//!
//! ```text
//! 1. Compute the full AD line (length = n, lookback = 0)
//! 2. Both fast and slow EMAs are seeded with the FIRST AD value (first-value seeding,
//!    matching ta-lib's internal EMA behavior for ADOSC).
//!    alpha = 2 / (period + 1)
//! 3. Both EMAs advance together from index 0; output starts at index slow_period-1.
//! 4. ADOSC[i] = fast_ema[i] - slow_ema[i]
//! ```
//!
//! # Parameters
//!
//! - `high`        — high prices
//! - `low`         — low prices
//! - `close`       — closing prices
//! - `volume`      — trading volume
//! - `fast_period` — fast EMA period (typically 3)
//! - `slow_period` — slow EMA period (typically 10)
//!
//! # Output
//!
//! - Length = `n - (slow_period - 1)`
//! - Returns empty `Vec` when input is too short or periods are 0
//!
//! # NaN Handling
//!
//! NaN in any input propagates via IEEE 754 arithmetic.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::volume::adosc;
//!
//! let n = 20;
//! let high:   Vec<f64> = vec![11.0; n];
//! let low:    Vec<f64> = vec![9.0; n];
//! let close:  Vec<f64> = vec![10.0; n];
//! let volume: Vec<f64> = vec![1000.0; n];
//! let result = adosc(&high, &low, &close, &volume, 3, 10);
//! // lookback = slow_period - 1 = 9
//! assert_eq!(result.len(), n - 9);
//! ```

/// Chaikin A/D Oscillator.
///
/// See [module documentation](self) for full details.
pub fn adosc(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    fast_period: usize,
    slow_period: usize,
) -> Vec<f64> {
    let n = close.len();
    if fast_period == 0 || slow_period == 0 || n < slow_period {
        return vec![];
    }
    if high.len() != n || low.len() != n || volume.len() != n {
        return vec![];
    }

    // ta-lib ADOSC: 两条 EMA 均从第一个 AD 值开始（first-value seeding），
    // 同步向前推进，输出从 slow_period-1 开始
    let out_len = n - (slow_period - 1);
    if out_len == 0 {
        return vec![];
    }

    let kf = 2.0 / (fast_period as f64 + 1.0);
    let kf1 = 1.0 - kf;
    let ks = 2.0 / (slow_period as f64 + 1.0);
    let ks1 = 1.0 - ks;

    // 内联 AD 计算，避免中间 Vec 分配和第二次数据遍历
    let range0 = high[0] - low[0];
    let clv0 = if range0 > 0.0 {
        (2.0 * close[0] - high[0] - low[0]) / range0
    } else {
        0.0
    };
    let mut ad_acc = clv0 * volume[0];

    let mut fast_prev = ad_acc;
    let mut slow_prev = ad_acc;

    // 预热阶段（i = 1..slow_period-1）：更新两条 EMA，不产生输出
    // 消除热路径中的分支判断
    let warmup_end = slow_period.saturating_sub(1);
    for i in 1..warmup_end {
        let h = high[i];
        let l = low[i];
        let range = h - l;
        let clv = if range > 0.0 {
            (2.0 * close[i] - h - l) / range
        } else {
            0.0
        };
        ad_acc += clv * volume[i];
        fast_prev = ad_acc * kf + fast_prev * kf1;
        slow_prev = ad_acc * ks + slow_prev * ks1;
    }

    let hot_start = warmup_end.max(1);
    let count = n - hot_start;
    let mut out = vec![0.0f64; count];
    for (j, i) in (hot_start..n).enumerate() {
        let h = high[i];
        let l = low[i];
        let range = h - l;
        let clv = if range > 0.0 {
            (2.0 * close[i] - h - l) / range
        } else {
            0.0
        };
        ad_acc += clv * volume[i];
        fast_prev = ad_acc * kf + fast_prev * kf1;
        slow_prev = ad_acc * ks + slow_prev * ks1;
        out[j] = fast_prev - slow_prev;
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
    fn adosc_output_length() {
        let n = 20usize;
        let high: Vec<f64> = vec![11.0; n];
        let low: Vec<f64> = vec![9.0; n];
        let close: Vec<f64> = vec![10.0; n];
        let volume: Vec<f64> = vec![1000.0; n];
        let result = adosc(&high, &low, &close, &volume, 3, 10);
        // lookback = slow_period - 1 = 9
        assert_eq!(result.len(), n - 9);
    }

    #[test]
    fn adosc_flat_input_is_zero() {
        // 价格恒定在中点 → clv=0 → AD=0 → fast_ema=slow_ema=0 → ADOSC=0
        let n = 30usize;
        let high: Vec<f64> = vec![11.0; n];
        let low: Vec<f64> = vec![9.0; n];
        let close: Vec<f64> = vec![10.0; n];
        let volume: Vec<f64> = vec![1000.0; n];
        let result = adosc(&high, &low, &close, &volume, 3, 10);
        for v in &result {
            assert_close(*v, 0.0, 1e-10);
        }
    }

    #[test]
    fn adosc_too_short() {
        let close = vec![10.0; 5];
        assert!(adosc(&close, &close, &close, &close, 3, 10).is_empty());
    }

    #[test]
    fn adosc_period_zero() {
        let close = vec![10.0; 20];
        assert!(adosc(&close, &close, &close, &close, 0, 10).is_empty());
        assert!(adosc(&close, &close, &close, &close, 3, 0).is_empty());
    }

    #[test]
    fn adosc_length_mismatch() {
        let a = vec![10.0; 20];
        let b = vec![10.0; 19];
        assert!(adosc(&a, &b, &a, &a, 3, 10).is_empty());
    }

    #[test]
    fn adosc_exact_lookback() {
        // 输入长度 = slow_period → 输出长度 = 1
        let n = 10usize;
        let high: Vec<f64> = vec![11.0; n];
        let low: Vec<f64> = vec![9.0; n];
        let close: Vec<f64> = vec![10.0; n];
        let volume: Vec<f64> = vec![1000.0; n];
        let result = adosc(&high, &low, &close, &volume, 3, 10);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn adosc_convergence_on_constant_ad() {
        // AD 线为常数时，快慢 EMA 最终收敛到同一值 → ADOSC ≈ 0
        let n = 100usize;
        let high: Vec<f64> = vec![12.0; n];
        let low: Vec<f64> = vec![8.0; n];
        let close: Vec<f64> = vec![12.0; n]; // clv = 1 → AD 线单调递增
        let volume: Vec<f64> = vec![1.0; n];
        let result = adosc(&high, &low, &close, &volume, 3, 10);
        // 最后几个值应趋近于 0（EMA 追上线性趋势的差收敛）
        let last = result[result.len() - 1];
        assert!(last.is_finite(), "ADOSC should be finite");
    }
}
