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

use std::collections::VecDeque;

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

    // Step 1: O(n) sliding min/max via monotonic deques
    let fastk_len = n - (fastk_period - 1);
    let mut fastk = Vec::with_capacity(fastk_len);

    // max_dq: indices in decreasing order of high values (front = current max)
    // min_dq: indices in increasing order of low values (front = current min)
    let mut max_dq: VecDeque<usize> = VecDeque::with_capacity(fastk_period);
    let mut min_dq: VecDeque<usize> = VecDeque::with_capacity(fastk_period);

    for i in 0..n {
        // 移除窗口外的过期索引
        if i >= fastk_period {
            let window_start = i - fastk_period + 1;
            while max_dq.front().map_or(false, |&j| j < window_start) {
                max_dq.pop_front();
            }
            while min_dq.front().map_or(false, |&j| j < window_start) {
                min_dq.pop_front();
            }
        }
        // 维护单调递减队列（最大值）
        while max_dq.back().map_or(false, |&j| high[j] <= high[i]) {
            max_dq.pop_back();
        }
        max_dq.push_back(i);
        // 维护单调递增队列（最小值）
        while min_dq.back().map_or(false, |&j| low[j] >= low[i]) {
            min_dq.pop_back();
        }
        min_dq.push_back(i);

        if i >= fastk_period - 1 {
            let hh = high[*max_dq.front().unwrap()];
            let ll = low[*min_dq.front().unwrap()];
            let fk = if (hh - ll).abs() < f64::EPSILON {
                0.0
            } else {
                (close[i] - ll) / (hh - ll) * 100.0
            };
            fastk.push(fk);
        }
    }

    // Step 2: slowk = SMA(fastk, slowk_period)
    let slowk = sma(&fastk, slowk_period);

    // Step 3: slowd = SMA(slowk, slowd_period)
    let slowd = sma(&slowk, slowd_period);

    // Trim slowk to match slowd length (ta-lib convention: both outputs same length)
    let trim = slowd_period - 1;
    let slowk = slowk[trim..].to_vec();
    StochOutput { slowk, slowd }
}

/// Internal SMA helper: O(n) sliding sum with precomputed inverse.
fn sma(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let inv = 1.0 / period as f64;
    let mut out = Vec::with_capacity(n - period + 1);
    let mut sum: f64 = data[..period].iter().sum();
    out.push(sum * inv);
    for i in period..n {
        sum += data[i] - data[i - period];
        out.push(sum * inv);
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
