//! MININDEX — index of rolling minimum using a monotone deque (O(n)).
//!
//! Returns the 0-based absolute index in the original array of the minimum
//! value within each window, as `f64`.
//! Output length = `n - period + 1` (lookback = period - 1).

use std::collections::VecDeque;

pub fn minindex(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let out_len = n - period + 1;
    let mut out = Vec::with_capacity(out_len);
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        while deque.front().map_or(false, |&j| i - j >= period) {
            deque.pop_front();
        }
        while deque.back().map_or(false, |&j| data[j] > data[i]) {
            deque.pop_back();
        }
        deque.push_back(i);
        if i >= period - 1 {
            out.push(*deque.front().unwrap() as f64);
        }
    }
    out
}
