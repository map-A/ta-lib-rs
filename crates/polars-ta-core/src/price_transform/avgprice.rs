//! Average Price (AVGPRICE)
//!
//! Formula: `(open + high + low + close) / 4`
//!
//! # Parameters
//! - `open`, `high`, `low`, `close` — OHLC price arrays (must be same length)
//!
//! # Output
//! - Length = input length (lookback = 0)
//! - Returns empty Vec if inputs have different lengths or are empty

pub fn avgprice(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    if n != high.len() || n != low.len() || n != close.len() || n == 0 {
        return vec![];
    }
    (0..n)
        .map(|i| (open[i] + high[i] + low[i] + close[i]) * 0.25)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avgprice_constant() {
        let o = vec![100.0; 5];
        let h = vec![101.0; 5];
        let l = vec![99.0; 5];
        let c = vec![100.5; 5];
        let r = avgprice(&o, &h, &l, &c);
        assert_eq!(r.len(), 5);
        // (100+101+99+100.5)/4 = 100.125
        assert!((r[0] - 100.125).abs() < 1e-10);
    }

    #[test]
    fn avgprice_empty_on_mismatch() {
        let r = avgprice(&[1.0, 2.0], &[1.0], &[1.0, 2.0], &[1.0, 2.0]);
        assert!(r.is_empty());
    }
}
