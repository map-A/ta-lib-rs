//! Stochastic Oscillator
//!
//! Compares a closing price to its high-low range over a given period.
//! Numerically identical to ta-lib's `TA_STOCH`.
//!
//! # Algorithm
//!
//! ```text
//! fastk[i] = (close[i] - lowest_low(i, fastk_period))
//!          / (highest_high(i, fastk_period) - lowest_low(i, fastk_period)) * 100
//! slowk = SMA(fastk, slowk_period)
//! slowd = SMA(slowk, slowd_period)
//! ```
//!
//! # Parameters
//!
//! - `high`          — high price series
//! - `low`           — low price series
//! - `close`         — close price series
//! - `fastk_period`  — raw %K lookback window
//! - `slowk_period`  — %K smoothing period (SMA)
//! - `slowd_period`  — %D smoothing period (SMA)
//!
//! # Output
//!
//! - `slowk` and `slowd` vectors
//! - Length = `n - (fastk_period + slowk_period + slowd_period - 3)`
//! - Returns empty vecs when input is too short
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::oscillator::stoch;
//!
//! let high  = vec![10.0, 11.0, 12.0, 11.0, 10.0, 11.0, 12.0, 11.0, 10.0];
//! let low   = vec![ 9.0, 10.0, 11.0, 10.0,  9.0, 10.0, 11.0, 10.0,  9.0];
//! let close = vec![ 9.5, 10.5, 11.5, 10.5,  9.5, 10.5, 11.5, 10.5,  9.5];
//! let result = stoch(&high, &low, &close, 5, 3, 3);
//! assert_eq!(result.slowk.len(), result.slowd.len());
//! ```

/// Output of the Stochastic Oscillator.
pub struct StochOutput {
    /// Smoothed %K (SMA of raw fastk).
    pub slowk: Vec<f64>,
    /// Smoothed %D (SMA of slowk).
    pub slowd: Vec<f64>,
}

/// Stochastic Oscillator.
///
/// See [module documentation](self) for full details.
pub fn stoch(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    fastk_period: usize,
    slowk_period: usize,
    slowd_period: usize,
) -> StochOutput {
    let empty = StochOutput { slowk: vec![], slowd: vec![] };

    let n = close.len();
    if fastk_period == 0 || slowk_period == 0 || slowd_period == 0 {
        return empty;
    }
    if high.len() != n || low.len() != n {
        return empty;
    }

    let lookback = fastk_period + slowk_period + slowd_period - 3;
    if n <= lookback {
        return empty;
    }

    let out_len = n - lookback;
    let mut out_slowk = vec![0.0f64; out_len];
    let mut out_slowd = vec![0.0f64; out_len];

    // 融合两个 SMA 过程：用小型环形缓冲区替代中间大数组 fastk[] 和 slowk[]
    // sk_ring: 存储最近 slowk_period 个 fastk 值（滑动窗口）
    // sd_ring: 存储最近 slowd_period 个 slowk 值（滑动窗口）
    let mut sk_ring = vec![0.0f64; slowk_period];
    let mut sd_ring = vec![0.0f64; slowd_period];
    let mut sk_sum = 0.0f64;
    let mut sd_sum = 0.0f64;
    let mut sk_head = 0usize; // sk_ring 中最旧元素的位置
    let mut sd_head = 0usize; // sd_ring 中最旧元素的位置
    let sk_inv = 1.0 / slowk_period as f64;
    let sd_inv = 1.0 / slowd_period as f64;
    let mut fk_count = 0usize; // 已生成的 fastk 值数量
    let mut sk_count = 0usize; // 已生成的 slowk 值数量
    let mut out_idx = 0usize;

    // Step 1: O(n) 单调双端队列，维护滑动最大/最小值
    let cap = fastk_period.next_power_of_two().max(4);
    let mask = cap - 1;
    let mut max_buf = vec![0usize; cap];
    let mut min_buf = vec![0usize; cap];
    let mut max_front = 0usize;
    let mut max_back = 0usize;
    let mut min_front = 0usize;
    let mut min_back = 0usize;

    unsafe {
        let high_ptr = high.as_ptr();
        let low_ptr = low.as_ptr();
        let close_ptr = close.as_ptr();
        let out_sk_ptr = out_slowk.as_mut_ptr();
        let out_sd_ptr = out_slowd.as_mut_ptr();

        for i in 0..n {
            // 移除窗口外的过期索引
            if i >= fastk_period {
                let window_start = i - fastk_period + 1;
                while max_front != max_back
                    && *max_buf.get_unchecked(max_front & mask) < window_start
                {
                    max_front = max_front.wrapping_add(1);
                }
                while min_front != min_back
                    && *min_buf.get_unchecked(min_front & mask) < window_start
                {
                    min_front = min_front.wrapping_add(1);
                }
            }
            // 维护单调递减队列（最大 high）
            while max_front != max_back
                && *high_ptr.add(*max_buf.get_unchecked(max_back.wrapping_sub(1) & mask))
                    <= *high_ptr.add(i)
            {
                max_back = max_back.wrapping_sub(1);
            }
            *max_buf.get_unchecked_mut(max_back & mask) = i;
            max_back = max_back.wrapping_add(1);

            // 维护单调递增队列（最小 low）
            while min_front != min_back
                && *low_ptr.add(*min_buf.get_unchecked(min_back.wrapping_sub(1) & mask))
                    >= *low_ptr.add(i)
            {
                min_back = min_back.wrapping_sub(1);
            }
            *min_buf.get_unchecked_mut(min_back & mask) = i;
            min_back = min_back.wrapping_add(1);

            if i + 1 >= fastk_period {
                let hh = *high_ptr.add(*max_buf.get_unchecked(max_front & mask));
                let ll = *low_ptr.add(*min_buf.get_unchecked(min_front & mask));
                let fk = if (hh - ll).abs() < f64::EPSILON {
                    0.0
                } else {
                    (*close_ptr.add(i) - ll) / (hh - ll) * 100.0
                };

                // Step 2: 融合 SMA pass 1 — 将 fastk 滑入 sk_ring
                if fk_count < slowk_period {
                    // 填充阶段：直接写入
                    *sk_ring.get_unchecked_mut(fk_count) = fk;
                    sk_sum += fk;
                } else {
                    // 滑动阶段：替换最旧元素
                    let old = *sk_ring.get_unchecked(sk_head);
                    *sk_ring.get_unchecked_mut(sk_head) = fk;
                    sk_sum += fk - old;
                    sk_head += 1;
                    if sk_head == slowk_period { sk_head = 0; }
                }
                fk_count += 1;

                if fk_count >= slowk_period {
                    let sk = sk_sum * sk_inv;

                    // Step 3: 融合 SMA pass 2 — 将 slowk 滑入 sd_ring
                    if sk_count < slowd_period {
                        *sd_ring.get_unchecked_mut(sk_count) = sk;
                        sd_sum += sk;
                    } else {
                        let old = *sd_ring.get_unchecked(sd_head);
                        *sd_ring.get_unchecked_mut(sd_head) = sk;
                        sd_sum += sk - old;
                        sd_head += 1;
                        if sd_head == slowd_period { sd_head = 0; }
                    }
                    sk_count += 1;

                    if sk_count >= slowd_period {
                        // slowk 对应的是与 slowd 同一时间点的那个值（ta-lib 约定）
                        *out_sk_ptr.add(out_idx) = sk;
                        *out_sd_ptr.add(out_idx) = sd_sum * sd_inv;
                        out_idx += 1;
                    }
                }
            }
        }
    }

    StochOutput { slowk: out_slowk, slowd: out_slowd }
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
    fn stoch_output_length() {
        let n = 50_usize;
        let high: Vec<f64> = (0..n).map(|i| i as f64 + 1.0).collect();
        let low: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let close: Vec<f64> = (0..n).map(|i| i as f64 + 0.5).collect();
        let res = stoch(&high, &low, &close, 5, 3, 3);
        let expected_len = n - (5 + 3 + 3 - 3);
        assert_eq!(res.slowk.len(), expected_len);
        assert_eq!(res.slowd.len(), expected_len);
    }

    #[test]
    fn stoch_too_short() {
        let h = vec![1.0, 2.0];
        let l = vec![0.0, 1.0];
        let c = vec![0.5, 1.5];
        let res = stoch(&h, &l, &c, 5, 3, 3);
        assert!(res.slowk.is_empty());
        assert!(res.slowd.is_empty());
    }

    #[test]
    fn stoch_period_zero() {
        let h = vec![1.0; 20];
        let l = vec![0.0; 20];
        let c = vec![0.5; 20];
        assert!(stoch(&h, &l, &c, 0, 3, 3).slowk.is_empty());
        assert!(stoch(&h, &l, &c, 5, 0, 3).slowk.is_empty());
        assert!(stoch(&h, &l, &c, 5, 3, 0).slowk.is_empty());
    }

    #[test]
    fn stoch_range() {
        let n = 100_usize;
        let high: Vec<f64> = (0..n).map(|i| (i as f64).sin() * 10.0 + 50.0 + 1.0).collect();
        let low: Vec<f64> = (0..n).map(|i| (i as f64).sin() * 10.0 + 50.0 - 1.0).collect();
        let close: Vec<f64> = (0..n).map(|i| (i as f64).sin() * 10.0 + 50.0).collect();
        let res = stoch(&high, &low, &close, 5, 3, 3);
        for (k, d) in res.slowk.iter().zip(res.slowd.iter()) {
            assert!(*k >= 0.0 && *k <= 100.0, "slowk out of range: {k}");
            assert!(*d >= 0.0 && *d <= 100.0, "slowd out of range: {d}");
        }
    }

    #[test]
    fn stoch_equal_high_low() {
        // When hh == ll, fastk should be 0
        let h = vec![5.0_f64; 20];
        let l = vec![5.0_f64; 20];
        let c = vec![5.0_f64; 20];
        let res = stoch(&h, &l, &c, 5, 3, 3);
        assert!(!res.slowk.is_empty());
        for v in &res.slowk {
            assert_close(*v, 0.0, 1e-10);
        }
        for v in &res.slowd {
            assert_close(*v, 0.0, 1e-10);
        }
    }

    #[test]
    fn stoch_slowk_always_same_len_as_slowd() {
        let n = 30_usize;
        let h: Vec<f64> = (0..n).map(|i| i as f64 + 2.0).collect();
        let l: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let c: Vec<f64> = (0..n).map(|i| i as f64 + 1.0).collect();
        let res = stoch(&h, &l, &c, 5, 3, 3);
        assert_eq!(res.slowk.len(), res.slowd.len());
    }
}
