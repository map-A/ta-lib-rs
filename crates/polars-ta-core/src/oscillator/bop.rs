//! Balance of Power (BOP)
//!
//! `BOP[i] = (close[i] - open[i]) / (high[i] - low[i])`
//! Output length = `n` (lookback = 0).

pub fn bop(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    assert_eq!(n, high.len());
    assert_eq!(n, low.len());
    assert_eq!(n, close.len());
    open.iter().zip(high).zip(low).zip(close)
        .map(|(((&o, &h), &l), &c)| {
            let hl = h - l;
            let safe = if hl == 0.0 { 1.0 } else { hl };
            let flag = if hl == 0.0 { 0.0 } else { 1.0 };
            (c - o) / safe * flag
        })
        .collect()
}
