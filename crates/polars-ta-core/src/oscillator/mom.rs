//! Momentum (MOM)
//!
//! `MOM[i] = data[i + period] - data[i]`
//! Output length = `n - period` (lookback = period).

pub fn mom(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n <= period {
        return vec![];
    }
    let out_len = n - period;
    let mut out = Vec::with_capacity(out_len);
    for i in 0..out_len {
        out.push(data[period + i] - data[i]);
    }
    out
}
