//! Moving Average Convergence/Divergence (MACD)
//!
//! Classic momentum/trend indicator — numerically identical to ta-lib's
//! `TA_MACD`.
//!
//! # Algorithm
//!
//! ```text
//! fast_ema  = EMA(data, fast_period)
//! slow_ema  = EMA(data, slow_period)
//! macd_line = fast_ema (aligned to slow_ema start) - slow_ema
//! signal    = EMA(macd_line, signal_period)
//! hist      = macd_line (aligned to signal) - signal
//! ```
//!
//! Alignment: the fast EMA is longer than the slow EMA by
//! `(slow_period - fast_period)` elements.  The leading elements of the fast
//! EMA are discarded so both series start at the same input index.
//!
//! # Parameters
//!
//! - `data`          — input price series
//! - `fast_period`   — fast EMA period (typically 12)
//! - `slow_period`   — slow EMA period (typically 26); must be ≥ `fast_period`
//! - `signal_period` — signal EMA period (typically 9)
//!
//! # Output
//!
//! [`MacdOutput`] with three equally-sized `Vec<f64>`:
//! - `macd`   — MACD line
//! - `signal` — signal line
//! - `hist`   — histogram (macd − signal)
//!
//! Length = `data.len() - (slow_period + signal_period - 2)`.
//! Returns all empty `Vec`s when input is too short.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::macd;
//!
//! let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
//! let out = macd(&data, 3, 5, 3);
//! // lookback = 5 - 1 + 3 - 1 = 6
//! assert_eq!(out.macd.len(), 50 - 6);
//! assert_eq!(out.signal.len(), out.macd.len());
//! assert_eq!(out.hist.len(), out.macd.len());
//! ```

use crate::trend::ema::ema;

/// Output of [`macd`]: three aligned series.
pub struct MacdOutput {
    /// MACD line (fast EMA − slow EMA).
    pub macd: Vec<f64>,
    /// Signal line (EMA of MACD line).
    pub signal: Vec<f64>,
    /// Histogram (MACD − signal).
    pub hist: Vec<f64>,
}

/// Moving Average Convergence/Divergence.
///
/// See [module documentation](self) for full details.
pub fn macd(
    data: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> MacdOutput {
    let empty = MacdOutput {
        macd: vec![],
        signal: vec![],
        hist: vec![],
    };

    if fast_period == 0 || slow_period == 0 || signal_period == 0 {
        return empty;
    }
    if slow_period < fast_period {
        return empty;
    }

    let slow_ema = ema(data, slow_period);
    if slow_ema.is_empty() {
        return empty;
    }

    let n = data.len();

    // Fast EMA: seeded from SMA(data[slow-fast .. slow]) — matches ta-lib internal MACD
    let offset = slow_period - fast_period;
    let fast_seed: f64 = data[offset..slow_period].iter().sum::<f64>() / fast_period as f64;
    let k_fast = 2.0 / (fast_period as f64 + 1.0);
    let k1_fast = 1.0 - k_fast;

    let fast_ema_len = n - (slow_period - 1);
    let mut fast_ema_aligned = Vec::with_capacity(fast_ema_len);
    fast_ema_aligned.push(fast_seed);
    let mut prev = fast_seed;
    for &v in &data[slow_period..] {
        let cur = v * k_fast + prev * k1_fast;
        fast_ema_aligned.push(cur);
        prev = cur;
    }

    debug_assert_eq!(fast_ema_aligned.len(), slow_ema.len());

    // MACD 线
    let macd_line: Vec<f64> = fast_ema_aligned
        .iter()
        .zip(slow_ema.iter())
        .map(|(&f, &s)| f - s)
        .collect();

    // 信号线：对 MACD 线做 EMA
    let signal = ema(&macd_line, signal_period);
    if signal.is_empty() {
        return empty;
    }

    // 将 MACD 线对齐到信号线（丢弃前 signal_period-1 个值）
    let sig_lb = signal_period - 1;
    let macd_trimmed: Vec<f64> = macd_line[sig_lb..].to_vec();

    debug_assert_eq!(macd_trimmed.len(), signal.len());

    let hist: Vec<f64> = macd_trimmed
        .iter()
        .zip(signal.iter())
        .map(|(&m, &s)| m - s)
        .collect();

    MacdOutput {
        macd: macd_trimmed,
        signal,
        hist,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64, eps: f64) {
        assert!(
            (actual - expected).abs() < eps || (actual.is_nan() && expected.is_nan()),
            "actual={actual:.15}, expected={expected:.15}",
        );
    }

    #[test]
    fn macd_output_length() {
        let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
        let out = macd(&data, 3, 5, 3);
        let expected = 50 - (5 + 3 - 2);
        assert_eq!(out.macd.len(), expected);
        assert_eq!(out.signal.len(), expected);
        assert_eq!(out.hist.len(), expected);
    }

    #[test]
    fn macd_standard_params() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let out = macd(&data, 12, 26, 9);
        let expected = 100 - (26 + 9 - 2);
        assert_eq!(out.macd.len(), expected);
    }

    #[test]
    fn macd_hist_equals_macd_minus_signal() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let out = macd(&data, 12, 26, 9);
        for i in 0..out.hist.len() {
            assert_close(out.hist[i], out.macd[i] - out.signal[i], 1e-10);
        }
    }

    #[test]
    fn macd_too_short() {
        let data = vec![1.0f64; 5];
        let out = macd(&data, 3, 5, 3);
        // lookback = 5+3-2=6, but data.len()=5 < 6+1
        assert!(out.macd.is_empty());
    }

    #[test]
    fn macd_zero_period() {
        let data = vec![1.0f64; 50];
        assert!(macd(&data, 0, 26, 9).macd.is_empty());
        assert!(macd(&data, 12, 0, 9).macd.is_empty());
        assert!(macd(&data, 12, 26, 0).macd.is_empty());
    }

    #[test]
    fn macd_constant_series() {
        // 常数序列：MACD 线应为零
        let data = vec![5.0f64; 100];
        let out = macd(&data, 12, 26, 9);
        for &v in &out.macd {
            assert_close(v, 0.0, 1e-10);
        }
        for &v in &out.hist {
            assert_close(v, 0.0, 1e-10);
        }
    }
}
