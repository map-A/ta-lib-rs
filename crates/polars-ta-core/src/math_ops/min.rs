//! MIN — Rolling Minimum
//!
//! 滑动窗口最小值，使用单调递增双端队列实现 O(n) 复杂度。
//!
//! Finds the lowest value in each rolling window using an O(n) monotone
//! deque algorithm. Numerically identical to ta-lib's `TA_MIN`.
//!
//! # Algorithm
//!
//! Maintains a monotone increasing deque of indices. For each new element:
//! 1. Pop expired indices (outside current window) from the front
//! 2. Pop indices with larger values from the back
//! 3. Push current index to the back
//! 4. Front of deque always holds the index of the window minimum
//!
//! # Parameters
//!
//! - `data`   — input series
//! - `period` — window length (≥ 1)
//!
//! # Output
//!
//! - Length = `data.len() - (period - 1)` (lookback = period - 1)
//! - Returns empty `Vec` when `data.len() < period`
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::math_ops::min;
//!
//! let data = vec![3.0, 1.0, 2.0, 5.0, 4.0];
//! let result = min(&data, 3);
//! assert_eq!(result.len(), 3);
//! assert!((result[0] - 1.0).abs() < 1e-10);  // min(3,1,2)
//! assert!((result[1] - 1.0).abs() < 1e-10);  // min(1,2,5)
//! assert!((result[2] - 2.0).abs() < 1e-10);  // min(2,5,4)
//! ```

pub fn min(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let out_len = n - period + 1;

    // 幂次方容量的环形缓冲区，用位掩码替代取模
    let cap = period.next_power_of_two().max(4);
    let mask = cap - 1;
    let mut buf = vec![0usize; cap];
    let mut front = 0usize;
    let mut back = 0usize;

    let mut out = vec![0.0f64; out_len];

    for i in 0..n {
        // 移除滑出窗口的过期下标
        if i >= period {
            let ws = i - period + 1;
            while front != back && buf[front & mask] < ws {
                front = front.wrapping_add(1);
            }
        }
        // 维护单调递增（移除所有大于当前值的尾部下标）
        while front != back
            && data[buf[back.wrapping_sub(1) & mask]] > data[i]
        {
            back = back.wrapping_sub(1);
        }
        buf[back & mask] = i;
        back = back.wrapping_add(1);

        if i >= period - 1 {
            out[i + 1 - period] = data[buf[front & mask]];
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_basic() {
        let data = vec![5.0, 3.0, 4.0, 1.0, 2.0];
        let result = min(&data, 3);
        assert_eq!(result, vec![3.0, 1.0, 1.0]);
    }

    #[test]
    fn min_boundary_short() {
        assert!(min(&[1.0, 2.0], 3).is_empty());
    }
}
