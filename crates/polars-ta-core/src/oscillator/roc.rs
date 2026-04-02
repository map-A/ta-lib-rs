//! Rate of Change (ROC)
//!
//! `ROC[i] = (data[i + period] - data[i]) / data[i] * 100`
//! Output length = `n - period` (lookback = period).

pub fn roc(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n <= period {
        return vec![];
    }
    let out_len = n - period;
    data[period..]
        .iter()
        .zip(data[..out_len].iter())
        .map(|(&a, &b)| {
            let safe = if b == 0.0 { 1.0 } else { b };
            let flag = if b == 0.0 { 0.0 } else { 1.0 };
            (a - b) / safe * flag * 100.0
        })
        .collect()
}
