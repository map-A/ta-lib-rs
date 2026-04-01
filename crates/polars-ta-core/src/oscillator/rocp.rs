//! Rate of Change Percentage (ROCP)
//!
//! `ROCP[i] = (data[i + period] - data[i]) / data[i]`
//! Output length = `n - period` (lookback = period).

pub fn rocp(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n <= period {
        return vec![];
    }
    let out_len = n - period;
    let mut out = Vec::with_capacity(out_len);
    for i in 0..out_len {
        let prev = data[i];
        if prev == 0.0 {
            out.push(0.0);
        } else {
            out.push((data[period + i] - prev) / prev);
        }
    }
    out
}
