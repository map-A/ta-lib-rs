//! Midpoint Price Over Period (MIDPRICE)
//!
//! Numerically identical to ta-lib's `TA_MIDPRICE`.
//!
//! # Algorithm
//!
//! ```text
//! midprice[i] = (max(high[i-period+1..=i]) + min(low[i-period+1..=i])) / 2
//! ```
//!
//! Uses a monotone-deque (O(n)) sliding-window approach.
//!
//! # Parameters
//!
//! - `high`   — high price series
//! - `low`    — low price series
//! - `period` — rolling window length (≥ 1)
//!
//! # Output
//!
//! - Length = `high.len() - (period - 1)`
//! - Returns an empty `Vec` when inputs are empty, have unequal lengths,
//!   or `high.len() < period`

use std::collections::VecDeque;

/// Midpoint Price Over Period.
///
/// See [module documentation](self) for full details.
pub fn midprice(high: &[f64], low: &[f64], period: usize) -> Vec<f64> {
    let n = high.len();
    if period == 0 || n < period || n != low.len() {
        return vec![];
    }

    let lookback = period - 1;
    let out_len = n - lookback;
    let mut out = Vec::with_capacity(out_len);

    // 高价最大值单调递减队列
    let mut max_dq: VecDeque<usize> = VecDeque::new();
    // 低价最小值单调递增队列
    let mut min_dq: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        while max_dq.front().map(|&j| j + period <= i).unwrap_or(false) {
            max_dq.pop_front();
        }
        while min_dq.front().map(|&j| j + period <= i).unwrap_or(false) {
            min_dq.pop_front();
        }

        while max_dq.back().map(|&j| high[j] <= high[i]).unwrap_or(false) {
            max_dq.pop_back();
        }
        max_dq.push_back(i);

        while min_dq.back().map(|&j| low[j] >= low[i]).unwrap_or(false) {
            min_dq.pop_back();
        }
        min_dq.push_back(i);

        if i >= lookback {
            let max_high = high[*max_dq.front().unwrap()];
            let min_low = low[*min_dq.front().unwrap()];
            out.push((max_high + min_low) / 2.0);
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
            "actual={actual:.15}, expected={expected:.15}",
        );
    }

    #[test]
    fn midprice_output_length() {
        let high: Vec<f64> = (1..=20).map(|x| x as f64 * 1.01).collect();
        let low: Vec<f64> = (1..=20).map(|x| x as f64 * 0.99).collect();
        let result = midprice(&high, &low, 5);
        // lookback = 4, output = 16
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn midprice_basic() {
        let high = vec![3.0, 5.0, 4.0, 6.0, 5.0];
        let low  = vec![1.0, 2.0, 1.5, 3.0, 2.5];
        let result = midprice(&high, &low, 3);
        // [0..=2]: max_high=5, min_low=1 → 3.0
        // [1..=3]: max_high=6, min_low=1.5 → 3.75
        // [2..=4]: max_high=6, min_low=1.5 → 3.75
        assert_eq!(result.len(), 3);
        assert_close(result[0], 3.0, 1e-10);
        assert_close(result[1], 3.75, 1e-10);
        assert_close(result[2], 3.75, 1e-10);
    }

    #[test]
    fn midprice_constant_series() {
        let high = vec![101.0f64; 50];
        let low  = vec![99.0f64; 50];
        let result = midprice(&high, &low, 10);
        for &v in &result {
            assert_close(v, 100.0, 1e-10);
        }
    }

    #[test]
    fn midprice_period1() {
        let high = vec![3.0, 5.0, 4.0];
        let low  = vec![1.0, 2.0, 1.5];
        let result = midprice(&high, &low, 1);
        assert_eq!(result.len(), 3);
        assert_close(result[0], 2.0, 1e-10);
        assert_close(result[1], 3.5, 1e-10);
        assert_close(result[2], 2.75, 1e-10);
    }

    #[test]
    fn midprice_boundary_short() {
        let high = vec![1.0, 2.0, 3.0];
        let low  = vec![0.5, 1.0, 1.5];
        assert!(midprice(&high, &low, 5).is_empty());
    }

    #[test]
    fn midprice_mismatched_lengths() {
        let high = vec![1.0, 2.0, 3.0];
        let low  = vec![0.5, 1.0];
        assert!(midprice(&high, &low, 2).is_empty());
    }

    #[test]
    fn midprice_period_zero() {
        let high = vec![1.0, 2.0, 3.0];
        let low  = vec![0.5, 1.0, 1.5];
        assert!(midprice(&high, &low, 0).is_empty());
    }

    #[test]
    fn midprice_empty_input() {
        assert!(midprice(&[], &[], 5).is_empty());
    }
}
