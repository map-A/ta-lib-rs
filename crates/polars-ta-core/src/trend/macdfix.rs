//! MACD with Fixed Periods (MACDFIX)
//!
//! MACD with fixed fast=12, slow=26 periods — numerically identical to
//! ta-lib's `TA_MACDFIX`.
//!
//! **Differs from `macd(data, 12, 26, signal_period)`**: ta-lib's MACDFIX uses
//! fixed EMA multipliers k_fast=0.15 and k_slow=0.075, whereas MACD derives k
//! from periods (k = 2/(period+1)). The seeding strategy is the same as MACD.
//!
//! # Parameters
//!
//! - `data`          — input price series
//! - `signal_period` — signal EMA period (typically 9)
//!
//! # Output
//!
//! [`MacdOutput`] with three equally-sized `Vec<f64>`:
//! - `macd`   — MACD line (fixed-k fast EMA − fixed-k slow EMA)
//! - `signal` — signal line (EMA of MACD, signal_period)
//! - `hist`   — histogram (macd − signal)
//!
//! Length = `data.len() − (26 + signal_period − 2)`.
//! Returns all empty `Vec`s when input is too short.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::macdfix;
//!
//! let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
//! let out = macdfix(&data, 9);
//! assert_eq!(out.macd.len(), 100 - (26 + 9 - 2));
//! ```

use crate::trend::ema::ema;
use crate::trend::macd::MacdOutput;

// ta-lib MACDFIX uses these fixed multipliers (not period-derived k=2/(p+1))
const K_FAST: f64 = 0.15; // corresponds to "period 12" in ta-lib FIX mode
const K_SLOW: f64 = 0.075; // corresponds to "period 26" in ta-lib FIX mode
const FAST_PERIOD: usize = 12;
const SLOW_PERIOD: usize = 26;

/// MACD with fixed fast=12 / slow=26 periods.
///
/// Uses fixed EMA multipliers (k_fast=0.15, k_slow=0.075) matching ta-lib's
/// internal `TA_INT_MACD` with period=0 (fixed mode).
///
/// See [module documentation](self) for full details.
pub fn macdfix(data: &[f64], signal_period: usize) -> MacdOutput {
    let empty = MacdOutput {
        macd: vec![],
        signal: vec![],
        hist: vec![],
    };

    if signal_period == 0 {
        return empty;
    }

    let n = data.len();
    // lookback = (SLOW_PERIOD - 1) + (signal_period - 1)
    if n < SLOW_PERIOD + signal_period - 1 {
        return empty;
    }

    // 慢速 EMA: 用 SMA(data[0..26]) 作为种子
    let slow_seed: f64 = data[..SLOW_PERIOD].iter().sum::<f64>() / SLOW_PERIOD as f64;

    // 快速 EMA: 用 SMA(data[offset..slow]) 作为种子（与 MACD 相同的对齐策略）
    let offset = SLOW_PERIOD - FAST_PERIOD;
    let fast_seed: f64 = data[offset..SLOW_PERIOD].iter().sum::<f64>() / FAST_PERIOD as f64;

    // 从 index 25 开始，同步推进两条 EMA
    let macd_line_len = n - (SLOW_PERIOD - 1);
    let mut macd_line = Vec::with_capacity(macd_line_len);
    let mut slow = slow_seed;
    let mut fast = fast_seed;
    macd_line.push(fast - slow);
    for &v in &data[SLOW_PERIOD..] {
        slow += K_SLOW * (v - slow);
        fast += K_FAST * (v - fast);
        macd_line.push(fast - slow);
    }
    debug_assert_eq!(macd_line.len(), macd_line_len);

    // 信号线：对 MACD 线做 EMA（用 period-derived k = 2/(signal+1)）
    let signal = ema(&macd_line, signal_period);
    if signal.is_empty() {
        return empty;
    }

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
    fn macdfix_output_length() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let out = macdfix(&data, 9);
        assert_eq!(out.macd.len(), 100 - (26 + 9 - 2));
        assert_eq!(out.signal.len(), out.macd.len());
        assert_eq!(out.hist.len(), out.macd.len());
    }

    #[test]
    fn macdfix_hist_equals_macd_minus_signal() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let out = macdfix(&data, 9);
        for i in 0..out.hist.len() {
            assert_close(out.hist[i], out.macd[i] - out.signal[i], 1e-10);
        }
    }

    #[test]
    fn macdfix_first_value_linear() {
        // ta-lib MACDFIX first output for 1..=34 is 6.80140873
        let data: Vec<f64> = (1..=34).map(|x| x as f64).collect();
        let out = macdfix(&data, 9);
        assert_eq!(out.macd.len(), 1);
        assert_close(out.macd[0], 6.80140873, 1e-6);
    }

    #[test]
    fn macdfix_too_short() {
        let data = vec![1.0f64; 33]; // lookback = 33, need ≥ 34
        let out = macdfix(&data, 9);
        assert!(out.macd.is_empty());
    }

    #[test]
    fn macdfix_zero_signal() {
        let data = vec![1.0f64; 50];
        assert!(macdfix(&data, 0).macd.is_empty());
    }

    #[test]
    fn macdfix_constant_series() {
        // 常数序列：MACD 线应为零
        let data = vec![5.0f64; 100];
        let out = macdfix(&data, 9);
        for &v in &out.macd {
            assert_close(v, 0.0, 1e-10);
        }
        for &v in &out.hist {
            assert_close(v, 0.0, 1e-10);
        }
    }
}
