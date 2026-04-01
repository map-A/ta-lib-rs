//! MINMAXINDEX — Indices of Rolling Minimum and Maximum
//!
//! 滑动窗口最小值与最大值的位置索引，单次 O(n) 遍历同时计算两者。
//!
//! Returns 0-based absolute indices as `f64` values to match ta-lib's convention.
//! Numerically identical to ta-lib's `TA_MINMAXINDEX`.
//!
//! Returns `(minidx_output, maxidx_output)` — two `Vec<f64>` of equal length.
//!
//! Output length = `n - period + 1` (lookback = period - 1).

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

    let cap = period.next_power_of_two().max(4);
    let mask = cap - 1;
    let mut min_buf = vec![0usize; cap];
    let mut max_buf = vec![0usize; cap];
    let mut min_front = 0usize;
    let mut min_back = 0usize;
    let mut max_front = 0usize;
    let mut max_back = 0usize;

    let mut minidx_out = vec![0.0f64; out_len];
    let mut maxidx_out = vec![0.0f64; out_len];

    for i in 0..n {
        if i >= period {
            let ws = i - period + 1;
            while min_front != min_back && min_buf[min_front & mask] < ws {
                min_front = min_front.wrapping_add(1);
            }
            while max_front != max_back && max_buf[max_front & mask] < ws {
                max_front = max_front.wrapping_add(1);
            }
        }

        let val = data[i];

        while min_front != min_back
            && data[min_buf[min_back.wrapping_sub(1) & mask]] > val
        {
            min_back = min_back.wrapping_sub(1);
        }
        min_buf[min_back & mask] = i;
        min_back = min_back.wrapping_add(1);

        while max_front != max_back
            && data[max_buf[max_back.wrapping_sub(1) & mask]] < val
        {
            max_back = max_back.wrapping_sub(1);
        }
        max_buf[max_back & mask] = i;
        max_back = max_back.wrapping_add(1);

        if i >= period - 1 {
            let out_i = i + 1 - period;
            minidx_out[out_i] = min_buf[min_front & mask] as f64;
            maxidx_out[out_i] = max_buf[max_front & mask] as f64;
        }
    }
    MinMaxIndexOutput { minidx: minidx_out, maxidx: maxidx_out }
}
