//! Midpoint Over Period (MIDPOINT)
//!
//! Numerically identical to ta-lib's `TA_MIDPOINT`.
//!
//! # Algorithm
//!
//! ```text
//! midpoint[i] = (max(data[i-period+1..=i]) + min(data[i-period+1..=i])) / 2
//! ```
//!
//! Uses a monotone-deque (O(n)) sliding-window min/max.
//!
//! # Parameters
//!
//! - `data`   — input price series (typically `close`)
//! - `period` — rolling window length (≥ 1)
//!
//! # Output
//!
//! - Length = `data.len() - (period - 1)`
//! - Returns an empty `Vec` when `data.len() < period`

use std::collections::VecDeque;

/// Midpoint Over Period.
///
/// See [module documentation](self) for full details.
pub fn midpoint(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }

    let lookback = period - 1;
    let out_len = n - lookback;
    let mut out = Vec::with_capacity(out_len);

    // 单调递减队列：存储最大值候选的下标
    let mut max_dq: VecDeque<usize> = VecDeque::new();
    // 单调递增队列：存储最小值候选的下标
    let mut min_dq: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        // 移除已滑出窗口的元素
        while max_dq.front().map(|&j| j + period <= i).unwrap_or(false) {
            max_dq.pop_front();
        }
        while min_dq.front().map(|&j| j + period <= i).unwrap_or(false) {
            min_dq.pop_front();
        }

        // 维护最大值单调队列（递减）
        while max_dq.back().map(|&j| data[j] <= data[i]).unwrap_or(false) {
            max_dq.pop_back();
        }
        max_dq.push_back(i);

        // 维护最小值单调队列（递增）
        while min_dq.back().map(|&j| data[j] >= data[i]).unwrap_or(false) {
            min_dq.pop_back();
        }
        min_dq.push_back(i);

        // 窗口满后输出
        if i >= lookback {
            let max_val = data[*max_dq.front().unwrap()];
            let min_val = data[*min_dq.front().unwrap()];
            out.push((max_val + min_val) / 2.0);
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
    fn midpoint_output_length() {
        let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        let result = midpoint(&data, 5);
        // lookback = 4, output = 16
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn midpoint_basic() {
        let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];
        let result = midpoint(&data, 3);
        // [0..=2]: max=3, min=1 → 2.0
        // [1..=3]: max=5, min=2 → 3.5
        // [2..=4]: max=5, min=2 → 3.5
        assert_eq!(result.len(), 3);
        assert_close(result[0], 2.0, 1e-10);
        assert_close(result[1], 3.5, 1e-10);
        assert_close(result[2], 3.5, 1e-10);
    }

    #[test]
    fn midpoint_constant_series() {
        let data = vec![5.0f64; 50];
        let result = midpoint(&data, 10);
        for &v in &result {
            assert_close(v, 5.0, 1e-10);
        }
    }

    #[test]
    fn midpoint_period1() {
        let data = vec![1.0, 2.0, 3.0];
        let result = midpoint(&data, 1);
        assert_eq!(result.len(), 3);
        assert_close(result[0], 1.0, 1e-10);
        assert_close(result[1], 2.0, 1e-10);
        assert_close(result[2], 3.0, 1e-10);
    }

    #[test]
    fn midpoint_boundary_short() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(midpoint(&data, 5).is_empty());
    }

    #[test]
    fn midpoint_period_zero() {
        assert!(midpoint(&[1.0, 2.0, 3.0], 0).is_empty());
    }

    #[test]
    fn midpoint_empty_input() {
        assert!(midpoint(&[], 5).is_empty());
    }
}
