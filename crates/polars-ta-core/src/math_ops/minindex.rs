//! MININDEX — Index of Rolling Minimum
//!
//! 滑动窗口最小值的位置索引，使用单调递增双端队列实现 O(n) 复杂度。
//!
//! Returns the 0-based absolute index in the original array of the minimum
//! value within each window, returned as `f64` to match ta-lib's convention.
//! Numerically identical to ta-lib's `TA_MININDEX`.
//!
//! Output length = `n - period + 1` (lookback = period - 1).

pub fn minindex(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let out_len = n - period + 1;

    let cap = period.next_power_of_two().max(4);
    let mask = cap - 1;
    let mut buf = vec![0usize; cap];
    let mut front = 0usize;
    let mut back = 0usize;

    let mut out = vec![0.0f64; out_len];

    for i in 0..n {
        if i >= period {
            let ws = i - period + 1;
            while front != back && buf[front & mask] < ws {
                front = front.wrapping_add(1);
            }
        }
        while front != back
            && data[buf[back.wrapping_sub(1) & mask]] > data[i]
        {
            back = back.wrapping_sub(1);
        }
        buf[back & mask] = i;
        back = back.wrapping_add(1);

        if i >= period - 1 {
            out[i + 1 - period] = buf[front & mask] as f64;
        }
    }
    out
}
