//! Generic Moving Average dispatcher (MA)
//!
//! Dispatches to one of nine MA implementations based on a `matype` integer —
//! numerically identical to ta-lib's `TA_MA`.
//!
//! | matype | Algorithm |
//! |--------|-----------|
//! | 0      | SMA       |
//! | 1      | EMA       |
//! | 2      | WMA       |
//! | 3      | DEMA      |
//! | 4      | TEMA      |
//! | 5      | TRIMA     |
//! | 6      | KAMA      |
//! | 7      | MAMA (not implemented — falls back to EMA) |
//! | 8      | T3        |
//! | _      | EMA (default) |
//!
//! # Parameters
//!
//! - `data`   — input price series
//! - `period` — window length
//! - `matype` — MA type selector (0–8)
//!
//! # Output
//!
//! A `Vec<f64>` whose length depends on the MA type and period.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::ma;
//!
//! let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
//! // matype=1 → EMA, lookback = period - 1
//! let out = ma(&data, 10, 1);
//! assert_eq!(out.len(), 50 - 9);
//! ```

use crate::trend::{dema, ema, kama, sma, t3, tema, trima, wma};

/// Generic Moving Average dispatcher.
///
/// See [module documentation](self) for full details.
pub fn ma(data: &[f64], period: usize, matype: usize) -> Vec<f64> {
    apply_ma(data, period, matype)
}

/// Internal MA dispatcher shared with MACDEXT.
pub(crate) fn apply_ma(data: &[f64], period: usize, matype: usize) -> Vec<f64> {
    match matype {
        0 => sma(data, period),
        1 => ema(data, period),
        2 => wma(data, period),
        3 => dema(data, period),
        4 => tema(data, period),
        5 => trima(data, period),
        6 => kama(data, period),
        8 => t3(data, period, 0.7),
        _ => ema(data, period),
    }
}

/// Lookback (number of leading output values consumed) for a given MA type and period.
pub(crate) fn ma_lookback(period: usize, matype: usize) -> usize {
    if period == 0 {
        return 0;
    }
    match matype {
        0 => period - 1,       // SMA
        1 => period - 1,       // EMA
        2 => period - 1,       // WMA
        3 => 2 * (period - 1), // DEMA
        4 => 3 * (period - 1), // TEMA
        5 => period - 1,       // TRIMA
        6 => period,           // KAMA: outputs n - period elements
        8 => 6 * (period - 1), // T3
        _ => period - 1,       // default EMA
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ma_sma_output_length() {
        let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
        assert_eq!(ma(&data, 10, 0).len(), 41);
    }

    #[test]
    fn ma_ema_output_length() {
        let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
        assert_eq!(ma(&data, 10, 1).len(), 41);
    }

    #[test]
    fn ma_wma_output_length() {
        let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
        assert_eq!(ma(&data, 10, 2).len(), 41);
    }

    #[test]
    fn ma_unknown_type_falls_back_to_ema() {
        let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
        assert_eq!(ma(&data, 10, 99).len(), ma(&data, 10, 1).len());
    }

    #[test]
    fn ma_period_1_returns_full_length() {
        let data = vec![1.0, 2.0, 3.0];
        assert_eq!(ma(&data, 1, 0).len(), 3); // SMA(1)
        assert_eq!(ma(&data, 1, 1).len(), 3); // EMA(1)
    }
}
