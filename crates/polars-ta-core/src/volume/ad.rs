//! Chaikin Accumulation/Distribution Line (AD)
//!
//! A cumulative volume-weighted indicator that measures the relationship
//! between price and volume to gauge supply and demand.
//! Numerically identical to ta-lib's `TA_AD`.
//!
//! # Algorithm
//!
//! ```text
//! clv[i] = (2*close[i] - high[i] - low[i]) / (high[i] - low[i])
//! clv[i] = 0.0  when high[i] == low[i]
//!
//! ad[0] = clv[0] * volume[0]
//! for i in 1..n:
//!   ad[i] = ad[i-1] + clv[i] * volume[i]
//! ```
//!
//! # Parameters
//!
//! - `high`   — high prices
//! - `low`    — low prices
//! - `close`  — closing prices
//! - `volume` — trading volume
//!
//! # Output
//!
//! - Length = `close.len()` (lookback = 0)
//! - Returns empty `Vec` when any input slice is empty or lengths differ
//!
//! # NaN Handling
//!
//! NaN in any input propagates via IEEE 754 arithmetic.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::volume::ad;
//!
//! let high   = vec![10.0, 11.0, 12.0];
//! let low    = vec![8.0,  9.0,  10.0];
//! let close  = vec![9.0,  10.0, 11.0];
//! let volume = vec![1000.0, 2000.0, 1500.0];
//! let result = ad(&high, &low, &close, &volume);
//! assert_eq!(result.len(), 3);
//! ```

/// Chaikin Accumulation/Distribution Line.
///
/// See [module documentation](self) for full details.
pub fn ad(high: &[f64], low: &[f64], close: &[f64], volume: &[f64]) -> Vec<f64> {
    let n = close.len();
    if n == 0 || high.len() != n || low.len() != n || volume.len() != n {
        return vec![];
    }

    // 单次遍历：直接累积到输出，避免两阶段（写中间数组 + 前缀求和）的双重内存访问。
    // 原始指针消除边界检查，与 ta-lib 的 C 单循环结构对齐。
    //
    // SAFETY: set_len 后循环对每个索引恰好写入一次，读取前所有位置均已初始化。
    let mut out = Vec::with_capacity(n);
    unsafe { out.set_len(n) };

    let mut acc = 0.0_f64;
    unsafe {
        let hp = high.as_ptr();
        let lp = low.as_ptr();
        let cp = close.as_ptr();
        let vp = volume.as_ptr();
        let op = out.as_mut_ptr() as *mut f64;
        for i in 0..n {
            let h = *hp.add(i);
            let l = *lp.add(i);
            let c = *cp.add(i);
            let v = *vp.add(i);
            let hl = h - l;
            if hl != 0.0 {
                acc += (2.0 * c - h - l) * v / hl;
            }
            *op.add(i) = acc;
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
    fn ad_basic() {
        // close = midpoint → clv = 0 → ad = 0 throughout
        let high   = vec![10.0, 11.0];
        let low    = vec![8.0,  9.0];
        let close  = vec![9.0,  10.0];   // midpoints
        let volume = vec![1000.0, 2000.0];
        let result = ad(&high, &low, &close, &volume);
        assert_eq!(result.len(), 2);
        assert_close(result[0], 0.0, 1e-10);
        assert_close(result[1], 0.0, 1e-10);
    }

    #[test]
    fn ad_close_at_high() {
        // close == high → clv = 1 → ad[0] = volume[0]
        let high   = vec![10.0];
        let low    = vec![8.0];
        let close  = vec![10.0];
        let volume = vec![500.0];
        let result = ad(&high, &low, &close, &volume);
        assert_eq!(result.len(), 1);
        assert_close(result[0], 500.0, 1e-10);
    }

    #[test]
    fn ad_close_at_low() {
        // close == low → clv = -1 → ad[0] = -volume[0]
        let high   = vec![10.0];
        let low    = vec![8.0];
        let close  = vec![8.0];
        let volume = vec![500.0];
        let result = ad(&high, &low, &close, &volume);
        assert_eq!(result.len(), 1);
        assert_close(result[0], -500.0, 1e-10);
    }

    #[test]
    fn ad_high_equals_low() {
        // 高低相同 → clv = 0 → 不改变累积值
        let high   = vec![10.0, 10.0];
        let low    = vec![10.0, 10.0];
        let close  = vec![10.0, 10.0];
        let volume = vec![1000.0, 1000.0];
        let result = ad(&high, &low, &close, &volume);
        assert_eq!(result.len(), 2);
        assert_close(result[0], 0.0, 1e-10);
        assert_close(result[1], 0.0, 1e-10);
    }

    #[test]
    fn ad_empty() {
        assert!(ad(&[], &[], &[], &[]).is_empty());
    }

    #[test]
    fn ad_length_mismatch() {
        let high  = vec![10.0, 11.0];
        let low   = vec![8.0];
        let close = vec![9.0, 10.0];
        let vol   = vec![100.0, 100.0];
        assert!(ad(&high, &low, &close, &vol).is_empty());
    }

    #[test]
    fn ad_cumulative_sum() {
        // 验证累积：close 始终在高点 → clv=1, ad[i] = sum of volumes
        let n = 5;
        let high:   Vec<f64> = vec![10.0; n];
        let low:    Vec<f64> = vec![8.0; n];
        let close:  Vec<f64> = vec![10.0; n]; // clv = 1
        let volume: Vec<f64> = vec![100.0; n];
        let result = ad(&high, &low, &close, &volume);
        assert_eq!(result.len(), n);
        for (i, v) in result.iter().enumerate() {
            assert_close(*v, 100.0 * (i + 1) as f64, 1e-10);
        }
    }

    #[test]
    fn ad_output_length_equals_input() {
        let n = 30;
        let data: Vec<f64> = vec![5.0; n];
        let result = ad(&data, &data, &data, &data);
        assert_eq!(result.len(), n);
    }
}
