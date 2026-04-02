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
    let empty = StochOutput {
        slowk: vec![],
        slowd: vec![],
    };

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

    // ta-lib 条件重扫算法（NaN 初始化）用于计算 fastk
    let mut highest = f64::NAN;
    let mut highest_idx: isize = -1;
    let mut lowest = f64::NAN;
    let mut lowest_idx: isize = -1;

    // 两个 SMA 环形缓冲区（永久污染：NaN 进入后 sum 永久为 NaN）
    let mut sk_ring = vec![0.0f64; slowk_period];
    let mut sd_ring = vec![0.0f64; slowd_period];
    let mut sk_sum = 0.0f64;
    let mut sd_sum = 0.0f64;
    let mut sk_head = 0usize;
    let mut sd_head = 0usize;
    let sk_inv = 1.0 / slowk_period as f64;
    let sd_inv = 1.0 / slowd_period as f64;
    let mut fk_count = 0usize;
    let mut sk_count = 0usize;
    let mut out_idx = 0usize;

    for i in 0..n {
        if i + 1 >= fastk_period {
            let trail = (i + 1 - fastk_period) as isize;

            // 条件重扫：滑动最大高价
            if highest_idx < trail {
                let t = trail as usize;
                highest = high[t];
                highest_idx = trail;
                for j in (t + 1)..=i {
                    if high[j] > highest {
                        highest = high[j];
                        highest_idx = j as isize;
                    }
                }
            } else if high[i] > highest {
                highest = high[i];
                highest_idx = i as isize;
            }

            // 条件重扫：滑动最小低价
            if lowest_idx < trail {
                let t = trail as usize;
                lowest = low[t];
                lowest_idx = trail;
                for j in (t + 1)..=i {
                    if low[j] < lowest {
                        lowest = low[j];
                        lowest_idx = j as isize;
                    }
                }
            } else if low[i] < lowest {
                lowest = low[i];
                lowest_idx = i as isize;
            }

            let fk = if close[i].is_nan() || highest.is_nan() || lowest.is_nan() {
                f64::NAN
            } else {
                let diff = highest - lowest;
                if diff.abs() < f64::EPSILON {
                    0.0
                } else {
                    (close[i] - lowest) / diff * 100.0
                }
            };

            // slowk SMA（永久污染的增量运行总和）
            if fk_count < slowk_period {
                sk_ring[fk_count] = fk;
                sk_sum += fk; // NaN 进入 sum 后永久为 NaN
            } else {
                sk_sum += fk - sk_ring[sk_head];
                sk_ring[sk_head] = fk;
                sk_head += 1;
                if sk_head == slowk_period {
                    sk_head = 0;
                }
            }
            fk_count += 1;

            if fk_count >= slowk_period {
                let sk = sk_sum * sk_inv;

                // slowd SMA（永久污染的增量运行总和）
                if sk_count < slowd_period {
                    sd_ring[sk_count] = sk;
                    sd_sum += sk;
                } else {
                    sd_sum += sk - sd_ring[sd_head];
                    sd_ring[sd_head] = sk;
                    sd_head += 1;
                    if sd_head == slowd_period {
                        sd_head = 0;
                    }
                }
                sk_count += 1;

                if sk_count >= slowd_period {
                    out_slowk[out_idx] = sk;
                    out_slowd[out_idx] = sd_sum * sd_inv;
                    out_idx += 1;
                }
            }
        }
    }

    StochOutput {
        slowk: out_slowk,
        slowd: out_slowd,
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
        let high: Vec<f64> = (0..n)
            .map(|i| (i as f64).sin() * 10.0 + 50.0 + 1.0)
            .collect();
        let low: Vec<f64> = (0..n)
            .map(|i| (i as f64).sin() * 10.0 + 50.0 - 1.0)
            .collect();
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
