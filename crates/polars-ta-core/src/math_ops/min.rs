//! MIN — rolling minimum using a monotone deque (O(n)).
//!
//! Output length = `n - period + 1` (lookback = period - 1).

use std::collections::VecDeque;

pub fn min(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let out_len = n - period + 1;
    let mut out = Vec::with_capacity(out_len);
    // 单调递增队列，队首为当前窗口最小值的下标
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        // 移除滑出窗口的下标
        while deque.front().map_or(false, |&j| i - j >= period) {
            deque.pop_front();
        }
        // 移除所有严格大于当前值的下标（维护递增性，保留最左侧的最小值）
        while deque.back().map_or(false, |&j| data[j] > data[i]) {
            deque.pop_back();
        }
        deque.push_back(i);
        if i >= period - 1 {
            out.push(data[*deque.front().unwrap()]);
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
