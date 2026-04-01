//! MINMAX — Rolling Minimum and Maximum
//!
//! 滑动窗口最小值与最大值，单次 O(n) 遍历同时计算两者。
//!
//! Uses two ring-buffer monotone deques to compute both minimum and maximum
//! in a single O(n) pass. Numerically identical to ta-lib's `TA_MINMAX`.
//!
//! Returns `(min_output, max_output)` — two `Vec<f64>` of equal length.
//!
//! Output length = `n - period + 1` (lookback = period - 1).

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

    let cap = period.next_power_of_two().max(4);
    let mask = cap - 1;
    let mut min_buf = vec![0usize; cap];
    let mut max_buf = vec![0usize; cap];
    let mut min_front = 0usize;
    let mut min_back = 0usize;
    let mut max_front = 0usize;
    let mut max_back = 0usize;

    let mut min_out = vec![0.0f64; out_len];
    let mut max_out = vec![0.0f64; out_len];

    for i in 0..n {
        if i >= period {
            let ws = i - period + 1;
            // 最小值队列过期清理
            while min_front != min_back && min_buf[min_front & mask] < ws {
                min_front = min_front.wrapping_add(1);
            }
            // 最大值队列过期清理
            while max_front != max_back && max_buf[max_front & mask] < ws {
                max_front = max_front.wrapping_add(1);
            }
        }

        let val = data[i];

        // 维护最小值单调递增队列
        while min_front != min_back
            && data[min_buf[min_back.wrapping_sub(1) & mask]] > val
        {
            min_back = min_back.wrapping_sub(1);
        }
        min_buf[min_back & mask] = i;
        min_back = min_back.wrapping_add(1);

        // 维护最大值单调递减队列
        while max_front != max_back
            && data[max_buf[max_back.wrapping_sub(1) & mask]] < val
        {
            max_back = max_back.wrapping_sub(1);
        }
        max_buf[max_back & mask] = i;
        max_back = max_back.wrapping_add(1);

        if i >= period - 1 {
            let out_i = i + 1 - period;
            min_out[out_i] = data[min_buf[min_front & mask]];
            max_out[out_i] = data[max_buf[max_front & mask]];
        }
    }
    MinMaxOutput { min: min_out, max: max_out }
}
