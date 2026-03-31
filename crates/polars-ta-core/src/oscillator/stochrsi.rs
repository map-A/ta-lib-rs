//! Stochastic RSI
//!
//! Applies the Stochastic formula to RSI values, producing a momentum oscillator
//! that measures the position of RSI within its own range.
//! Numerically identical to ta-lib's `TA_STOCHRSI`.
//!
//! # Algorithm
//!
//! ```text
//! rsi_values = RSI(data, period)
//! then treat rsi_values as H/L/C and compute:
//! fastk = (rsi - lowest_rsi(fastk_period)) / (highest_rsi(fastk_period) - lowest_rsi(fastk_period)) * 100
//! fastd = SMA(fastk, fastd_period)
//! ```
//!
//! # Parameters
//!
//! - `data`          — input price series
//! - `period`        — RSI period
//! - `fastk_period`  — Stochastic %K window applied to RSI values
//! - `fastd_period`  — smoothing period for %D
//!
//! # Output
//!
//! - Length = `n - (period + fastk_period - 1 + fastd_period - 1)`
//! - Returns empty vecs when input is too short
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::oscillator::stochrsi;
//!
//! let data: Vec<f64> = (0..50).map(|i| (i as f64).sin() * 10.0 + 50.0).collect();
//! let result = stochrsi(&data, 14, 5, 3);
//! assert_eq!(result.fastk.len(), result.fastd.len());
//! ```

use super::rsi::rsi;

/// Output of the Stochastic RSI.
pub struct StochRsiOutput {
    /// Raw Stochastic %K applied to RSI values.
    pub fastk: Vec<f64>,
    /// Smoothed %D (SMA of fastk).
    pub fastd: Vec<f64>,
}

/// Stochastic RSI.
///
/// See [module documentation](self) for full details.
pub fn stochrsi(
    data: &[f64],
    period: usize,
    fastk_period: usize,
    fastd_period: usize,
) -> StochRsiOutput {
    let empty = StochRsiOutput { fastk: vec![], fastd: vec![] };

    if period == 0 || fastk_period == 0 || fastd_period == 0 {
        return empty;
    }

    let rsi_values = rsi(data, period);
    if rsi_values.is_empty() {
        return empty;
    }

    // Treat RSI as H/L/C (all the same slice)
    let n = rsi_values.len();
    if n < fastk_period {
        return empty;
    }

    // Compute fastk from RSI values
    let fastk_raw_len = n - (fastk_period - 1);
    let mut fastk_raw = Vec::with_capacity(fastk_raw_len);
    for i in (fastk_period - 1)..n {
        let start = i + 1 - fastk_period;
        let mut hh = rsi_values[start];
        let mut ll = rsi_values[start];
        for j in (start + 1)..=i {
            if rsi_values[j] > hh { hh = rsi_values[j]; }
            if rsi_values[j] < ll { ll = rsi_values[j]; }
        }
        let fk = if (hh - ll).abs() < f64::EPSILON {
            0.0
        } else {
            (rsi_values[i] - ll) / (hh - ll) * 100.0
        };
        fastk_raw.push(fk);
    }

    // fastd = SMA(fastk_raw, fastd_period)
    let fastd = sma(&fastk_raw, fastd_period);

    // Trim fastk to match fastd length
    let trim = fastk_raw.len() - fastd.len();
    let fastk = fastk_raw[trim..].to_vec();

    StochRsiOutput { fastk, fastd }
}

/// Internal SMA helper.
fn sma(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let mut out = Vec::with_capacity(n - period + 1);
    let mut sum: f64 = data[..period].iter().sum();
    out.push(sum / period as f64);
    for i in period..n {
        sum += data[i] - data[i - period];
        out.push(sum / period as f64);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stochrsi_output_length() {
        let n = 100_usize;
        let data: Vec<f64> = (0..n).map(|i| (i as f64 * 0.1).sin() * 10.0 + 50.0).collect();
        let period = 14;
        let fastk_period = 5;
        let fastd_period = 3;
        let res = stochrsi(&data, period, fastk_period, fastd_period);
        let expected = n - period - fastk_period - fastd_period + 2;
        assert_eq!(res.fastk.len(), expected);
        assert_eq!(res.fastd.len(), expected);
    }

    #[test]
    fn stochrsi_same_len() {
        let data: Vec<f64> = (0..80).map(|i| (i as f64 * 0.2).sin() * 5.0 + 50.0).collect();
        let res = stochrsi(&data, 14, 5, 3);
        assert_eq!(res.fastk.len(), res.fastd.len());
    }

    #[test]
    fn stochrsi_too_short() {
        let data = vec![1.0; 10];
        let res = stochrsi(&data, 14, 5, 3);
        assert!(res.fastk.is_empty());
        assert!(res.fastd.is_empty());
    }

    #[test]
    fn stochrsi_period_zero() {
        let data = vec![1.0; 50];
        assert!(stochrsi(&data, 0, 5, 3).fastk.is_empty());
        assert!(stochrsi(&data, 14, 0, 3).fastk.is_empty());
        assert!(stochrsi(&data, 14, 5, 0).fastk.is_empty());
    }

    #[test]
    fn stochrsi_range() {
        let data: Vec<f64> = (0..100).map(|i| (i as f64 * 0.3).sin() * 10.0 + 50.0).collect();
        let res = stochrsi(&data, 14, 5, 3);
        for k in &res.fastk {
            assert!(*k >= 0.0 && *k <= 100.0, "fastk out of range: {k}");
        }
        for d in &res.fastd {
            assert!(*d >= 0.0 && *d <= 100.0, "fastd out of range: {d}");
        }
    }
}
