//! MINMAXINDEX — indices of rolling minimum and maximum simultaneously.
//!
//! Returns 0-based absolute indices as `f64`.
//! Output length = `n - period + 1` (lookback = period - 1).

use std::collections::VecDeque;

pub struct MinMaxIndexOutput {
    pub minidx: Vec<f64>,
    pub maxidx: Vec<f64>,
}

pub fn minmaxindex(data: &[f64], period: usize) -> MinMaxIndexOutput {
    let n = data.len();
    if period == 0 || n < period {
        return MinMaxIndexOutput { minidx: vec![], maxidx: vec![] };
    }
    let out_len = n - period + 1;
    let mut minidx_out = Vec::with_capacity(out_len);
    let mut maxidx_out = Vec::with_capacity(out_len);
    let mut min_deque: VecDeque<usize> = VecDeque::new();
    let mut max_deque: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        while min_deque.front().map_or(false, |&j| i - j >= period) {
            min_deque.pop_front();
        }
        while min_deque.back().map_or(false, |&j| data[j] > data[i]) {
            min_deque.pop_back();
        }
        min_deque.push_back(i);

        while max_deque.front().map_or(false, |&j| i - j >= period) {
            max_deque.pop_front();
        }
        while max_deque.back().map_or(false, |&j| data[j] < data[i]) {
            max_deque.pop_back();
        }
        max_deque.push_back(i);

        if i >= period - 1 {
            minidx_out.push(*min_deque.front().unwrap() as f64);
            maxidx_out.push(*max_deque.front().unwrap() as f64);
        }
    }
    MinMaxIndexOutput { minidx: minidx_out, maxidx: maxidx_out }
}
