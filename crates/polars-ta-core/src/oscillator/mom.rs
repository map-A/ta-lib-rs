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
    data[period..].iter()
        .zip(data[..out_len].iter())
        .map(|(&a, &b)| a - b)
        .collect()
}
