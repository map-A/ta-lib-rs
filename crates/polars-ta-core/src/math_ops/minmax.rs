//! MINMAX — rolling minimum and maximum simultaneously.
//!
//! Uses two monotone deques to compute both in a single O(n) pass.
//! Output length = `n - period + 1` (lookback = period - 1).

use std::collections::VecDeque;

pub struct MinMaxOutput {
    pub min: Vec<f64>,
    pub max: Vec<f64>,
}

pub fn minmax(data: &[f64], period: usize) -> MinMaxOutput {
    let n = data.len();
    if period == 0 || n < period {
        return MinMaxOutput { min: vec![], max: vec![] };
    }
    let out_len = n - period + 1;
    let mut min_out = Vec::with_capacity(out_len);
    let mut max_out = Vec::with_capacity(out_len);
    let mut min_deque: VecDeque<usize> = VecDeque::new();
    let mut max_deque: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        // 最小值队列（单调递增）
        while min_deque.front().map_or(false, |&j| i - j >= period) {
            min_deque.pop_front();
        }
        while min_deque.back().map_or(false, |&j| data[j] > data[i]) {
            min_deque.pop_back();
        }
        min_deque.push_back(i);

        // 最大值队列（单调递减）
        while max_deque.front().map_or(false, |&j| i - j >= period) {
            max_deque.pop_front();
        }
        while max_deque.back().map_or(false, |&j| data[j] < data[i]) {
            max_deque.pop_back();
        }
        max_deque.push_back(i);

        if i >= period - 1 {
            min_out.push(data[*min_deque.front().unwrap()]);
            max_out.push(data[*max_deque.front().unwrap()]);
        }
    }
    MinMaxOutput { min: min_out, max: max_out }
}
