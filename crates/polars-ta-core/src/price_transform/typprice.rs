//! Typical Price (TYPPRICE)
//!
//! Formula: `(high + low + close) / 3`
//!
//! # Parameters
//! - `high`, `low`, `close` — price arrays (must be same length)
//!
//! # Output
//! - Length = input length (lookback = 0)
//! - Returns empty Vec if inputs have different lengths or are empty

pub fn typprice(high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = high.len();
    if n != low.len() || n != close.len() || n == 0 {
        return vec![];
    }
    (0..n)
        .map(|i| (high[i] + low[i] + close[i]) / 3.0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typprice_constant() {
        let h = vec![101.0; 5];
        let l = vec![99.0; 5];
        let c = vec![100.0; 5];
        let r = typprice(&h, &l, &c);
        assert_eq!(r.len(), 5);
        assert!((r[0] - 100.0).abs() < 1e-10);
    }
}
