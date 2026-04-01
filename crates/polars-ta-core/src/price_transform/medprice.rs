//! Median Price (MEDPRICE)
//!
//! Formula: `(high + low) / 2`
//!
//! # Parameters
//! - `high`, `low` — price arrays (must be same length)
//!
//! # Output
//! - Length = input length (lookback = 0)
//! - Returns empty Vec if inputs have different lengths or are empty

pub fn medprice(high: &[f64], low: &[f64]) -> Vec<f64> {
    let n = high.len();
    if n != low.len() || n == 0 {
        return vec![];
    }
    (0..n).map(|i| (high[i] + low[i]) * 0.5).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn medprice_constant() {
        let h = vec![101.0; 5];
        let l = vec![99.0; 5];
        let r = medprice(&h, &l);
        assert_eq!(r.len(), 5);
        assert!((r[0] - 100.0).abs() < 1e-10);
    }
}
