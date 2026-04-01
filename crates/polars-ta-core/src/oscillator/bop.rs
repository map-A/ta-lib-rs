//! Balance of Power (BOP)
//!
//! `BOP[i] = (close[i] - open[i]) / (high[i] - low[i])`
//! Output length = `n` (lookback = 0).

pub fn bop(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    assert_eq!(n, high.len());
    assert_eq!(n, low.len());
    assert_eq!(n, close.len());
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let hl = high[i] - low[i];
        if hl == 0.0 {
            out.push(0.0);
        } else {
            out.push((close[i] - open[i]) / hl);
        }
    }
    out
}
