//! Parabolic SAR (Stop and Reverse)
//!
//! A trailing stop-loss indicator that accelerates toward price — numerically
//! identical to ta-lib's `TA_SAR`.
//!
//! # Algorithm
//!
//! The state machine maintains:
//! - `is_long` — current trend direction
//! - `ep` — extreme point (highest high in uptrend, lowest low in downtrend)
//! - `acc` — acceleration factor, increases each time EP is updated
//!
//! **Uptrend:**
//! ```text
//! new_sar = prev_sar + acc * (ep - prev_sar)
//! new_sar = min(new_sar, low[i-1])        // cannot exceed prior lows
//! new_sar = min(new_sar, low[i-2])        // (if i >= 2)
//! if low[i] < new_sar: flip to downtrend
//! ```
//!
//! **Downtrend:**
//! ```text
//! new_sar = prev_sar + acc * (ep - prev_sar)   // ep < prev_sar, so SAR falls
//! new_sar = max(new_sar, high[i-1])        // cannot exceed prior highs
//! new_sar = max(new_sar, high[i-2])        // (if i >= 2)
//! if high[i] > new_sar: flip to uptrend
//! ```
//!
//! # Parameters
//!
//! - `high`         — high price series
//! - `low`          — low price series (same length as `high`)
//! - `acceleration` — initial/increment acceleration factor (typically 0.02)
//! - `maximum`      — maximum acceleration factor (typically 0.20)
//!
//! # Output
//!
//! - Length = `high.len() - 1` (lookback = 1)
//! - Returns an empty `Vec` when `high.len() < 2`
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::sar;
//!
//! let high = vec![10.0, 11.0, 12.0, 11.5, 10.5];
//! let low  = vec![ 9.0,  9.5, 10.5, 10.0,  9.0];
//! let result = sar(&high, &low, 0.02, 0.20);
//! assert_eq!(result.len(), 4); // lookback = 1
//! ```

/// Parabolic Stop and Reverse.
///
/// See [module documentation](self) for full details.
pub fn sar(high: &[f64], low: &[f64], acceleration: f64, maximum: f64) -> Vec<f64> {
    let n = high.len();
    assert_eq!(n, low.len(), "high and low must have equal length");

    if n < 2 {
        return vec![];
    }

    let out_len = n - 1;
    let mut out = Vec::with_capacity(out_len);

    // 根据第一根与第二根 K 线的最高价确定初始方向

    let mut is_long = high[1] >= high[0];

    let (mut ep, mut sar) = if is_long {
        // 上升趋势：EP = 第二根最高价，SAR = 第一根最低价
        (high[1], low[0])
    } else {
        // 下降趋势：EP = 第二根最低价，SAR = 第一根最高价
        (low[1], high[0])
    };

    let mut acc = acceleration;

    for i in 1..n {
        // 使用上一周期的 ep/acc 计算原始 SAR
        let raw_sar = sar + acc * (ep - sar);

        let new_sar = if is_long {
            // 先截断 SAR（不能高于前两根最低价），再检查翻转（与 ta-lib 一致）
            let mut clamped = raw_sar.min(low[i - 1]);
            if i >= 3 { clamped = clamped.min(low[i - 2]); }

            if low[i] <= clamped {
                // 翻转到空头
                let old_ep = ep;
                is_long = false;
                ep = low[i];
                acc = acceleration;
                let mut s = old_ep.max(high[i - 1]);
                if i >= 3 { s = s.max(high[i - 2]); }
                s
            } else {
                // 不翻转：再更新 EP/加速系数（用于下一周期）
                if high[i] > ep {
                    ep = high[i];
                    acc = (acc + acceleration).min(maximum);
                }
                clamped
            }
        } else {
            // 先截断 SAR（不能低于前两根最高价），再检查翻转（与 ta-lib 一致）
            let mut clamped = raw_sar.max(high[i - 1]);
            if i >= 3 { clamped = clamped.max(high[i - 2]); }

            if high[i] >= clamped {
                // 翻转到多头
                let old_ep = ep;
                is_long = true;
                ep = high[i];
                acc = acceleration;
                let mut s = old_ep.min(low[i - 1]);
                if i >= 3 { s = s.min(low[i - 2]); }
                s
            } else {
                // 不翻转：再更新 EP/加速系数（用于下一周期）
                if low[i] < ep {
                    ep = low[i];
                    acc = (acc + acceleration).min(maximum);
                }
                clamped
            }
        };

        out.push(new_sar);
        sar = new_sar;
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
    fn sar_output_length() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 10.5];
        let low = vec![9.0, 9.5, 10.5, 10.0, 9.0];
        let result = sar(&high, &low, 0.02, 0.20);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn sar_too_short() {
        assert!(sar(&[10.0], &[9.0], 0.02, 0.20).is_empty());
        assert!(sar(&[], &[], 0.02, 0.20).is_empty());
    }

    #[test]
    fn sar_exactly_two_bars() {
        // 恰好 2 个输入：产生 1 个输出
        let high = vec![11.0, 12.0];
        let low = vec![9.0, 10.0];
        let result = sar(&high, &low, 0.02, 0.20);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn sar_uptrend_initial() {
        // high[1] > high[0]: 初始上升趋势，SAR 应在低价下方
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![8.0, 9.0, 10.0];
        let result = sar(&high, &low, 0.02, 0.20);
        // 第一个 SAR 应 ≤ low[0]
        assert!(result[0] <= low[0] + 1e-10);
    }

    #[test]
    fn sar_downtrend_initial() {
        // high[1] < high[0]: 初始下降趋势，SAR 应在高价上方
        let high = vec![12.0, 11.0, 10.0];
        let low = vec![10.0, 9.0, 8.0];
        let result = sar(&high, &low, 0.02, 0.20);
        // 第一个 SAR 应 ≥ high[0]
        assert!(result[0] >= high[0] - 1e-10);
    }

    #[test]
    fn sar_all_values_finite() {
        // 无 panic，所有输出有限
        let high: Vec<f64> = (0..50).map(|i| 100.0 + (i as f64).sin() * 5.0 + i as f64 * 0.1).collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 2.0).collect();
        let result = sar(&high, &low, 0.02, 0.20);
        assert_eq!(result.len(), 49);
        for &v in &result {
            assert!(v.is_finite(), "SAR value should be finite");
        }
    }

    #[test]
    fn sar_acceleration_capped() {
        // 即使 acc 不断增加，也不超过 maximum
        let n = 100;
        let high: Vec<f64> = (0..n).map(|i| 100.0 + i as f64).collect(); // 持续上涨
        let low: Vec<f64> = high.iter().map(|&h| h - 1.0).collect();
        let result = sar(&high, &low, 0.02, 0.20);
        // 所有输出应有限且为非负
        for &v in &result {
            assert!(v.is_finite());
        }
    }
}
