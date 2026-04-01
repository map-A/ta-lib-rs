//! Stochastic RSI
//!
//! Applies the Stochastic formula to RSI values, producing a momentum oscillator
//! that measures the position of RSI within its own range.
//! Numerically identical to ta-lib's `TA_STOCHRSI`.
//!
//! # Algorithm
//!
//! ```text
//! rsi_values = RSI(data, period)
//! then treat rsi_values as H/L/C and compute:
//! fastk = (rsi - lowest_rsi(fastk_period)) / (highest_rsi(fastk_period) - lowest_rsi(fastk_period)) * 100
//! fastd = SMA(fastk, fastd_period)
//! ```
//!
//! # Parameters
//!
//! - `data`          — input price series
//! - `period`        — RSI period
//! - `fastk_period`  — Stochastic %K window applied to RSI values
//! - `fastd_period`  — smoothing period for %D
//!
//! # Output
//!
//! - Length = `n - (period + fastk_period - 1 + fastd_period - 1)`
//! - Returns empty vecs when input is too short
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::oscillator::stochrsi;
//!
//! let data: Vec<f64> = (0..50).map(|i| (i as f64).sin() * 10.0 + 50.0).collect();
//! let result = stochrsi(&data, 14, 5, 3);
//! assert_eq!(result.fastk.len(), result.fastd.len());
//! ```

use super::rsi::rsi;
use std::collections::VecDeque;

/// Output of the Stochastic RSI.
pub struct StochRsiOutput {
    /// Raw Stochastic %K applied to RSI values.
    pub fastk: Vec<f64>,
    /// Smoothed %D (SMA of fastk).
    pub fastd: Vec<f64>,
}

/// Stochastic RSI.
///
/// See [module documentation](self) for full details.
pub fn stochrsi(
    data: &[f64],
    period: usize,
    fastk_period: usize,
    fastd_period: usize,
) -> StochRsiOutput {
    let empty = StochRsiOutput { fastk: vec![], fastd: vec![] };

    if period == 0 || fastk_period == 0 || fastd_period == 0 {
        return empty;
    }

    let rsi_values = rsi(data, period);
    if rsi_values.is_empty() {
        return empty;
    }

    // Treat RSI as H/L/C (all the same slice)
    let n = rsi_values.len();
    if n < fastk_period {
        return empty;
    }

    // Compute fastk from RSI values using O(n) sliding min/max deques
    let fastk_raw_len = n - (fastk_period - 1);
    let mut fastk_raw = Vec::with_capacity(fastk_raw_len);

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
        while max_dq.back().map_or(false, |&j| rsi_values[j] <= rsi_values[i]) {
            max_dq.pop_back();
        }
        max_dq.push_back(i);
        // 维护单调递增队列（最小值）
        while min_dq.back().map_or(false, |&j| rsi_values[j] >= rsi_values[i]) {
            min_dq.pop_back();
        }
        min_dq.push_back(i);

        if i >= fastk_period - 1 {
            let hh = rsi_values[*max_dq.front().unwrap()];
            let ll = rsi_values[*min_dq.front().unwrap()];
            let fk = if (hh - ll).abs() < f64::EPSILON {
                0.0
            } else {
                (rsi_values[i] - ll) / (hh - ll) * 100.0
            };
            fastk_raw.push(fk);
        }
    }

    // fastd = SMA(fastk_raw, fastd_period)
    let fastd = sma(&fastk_raw, fastd_period);

    // Trim fastk to match fastd length
    let trim = fastk_raw.len() - fastd.len();
    let fastk = fastk_raw[trim..].to_vec();

    StochRsiOutput { fastk, fastd }
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

    #[test]
    fn stochrsi_output_length() {
        let n = 100_usize;
        let data: Vec<f64> = (0..n).map(|i| (i as f64 * 0.1).sin() * 10.0 + 50.0).collect();
        let period = 14;
        let fastk_period = 5;
        let fastd_period = 3;
        let res = stochrsi(&data, period, fastk_period, fastd_period);
        let expected = n - period - fastk_period - fastd_period + 2;
        assert_eq!(res.fastk.len(), expected);
        assert_eq!(res.fastd.len(), expected);
    }

    #[test]
    fn stochrsi_same_len() {
        let data: Vec<f64> = (0..80).map(|i| (i as f64 * 0.2).sin() * 5.0 + 50.0).collect();
        let res = stochrsi(&data, 14, 5, 3);
        assert_eq!(res.fastk.len(), res.fastd.len());
    }

    #[test]
    fn stochrsi_too_short() {
        let data = vec![1.0; 10];
        let res = stochrsi(&data, 14, 5, 3);
        assert!(res.fastk.is_empty());
        assert!(res.fastd.is_empty());
    }

    #[test]
    fn stochrsi_period_zero() {
        let data = vec![1.0; 50];
        assert!(stochrsi(&data, 0, 5, 3).fastk.is_empty());
        assert!(stochrsi(&data, 14, 0, 3).fastk.is_empty());
        assert!(stochrsi(&data, 14, 5, 0).fastk.is_empty());
    }

    #[test]
    fn stochrsi_range() {
        let data: Vec<f64> = (0..100).map(|i| (i as f64 * 0.3).sin() * 10.0 + 50.0).collect();
        let res = stochrsi(&data, 14, 5, 3);
        for k in &res.fastk {
            assert!(*k >= 0.0 && *k <= 100.0, "fastk out of range: {k}");
        }
        for d in &res.fastd {
            assert!(*d >= 0.0 && *d <= 100.0, "fastd out of range: {d}");
        }
    }
}
