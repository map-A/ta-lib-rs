//! MACD with Configurable MA Types (MACDEXT)
//!
//! Like MACD but each stage can use a different moving average type —
//! numerically identical to ta-lib's `TA_MACDEXT`.
//!
//! When all three matypes are 1 (EMA), the result is identical to `TA_MACD`.
//!
//! # Algorithm
//!
//! ```text
//! slow_ma   = apply_ma(data, slow_period, slow_matype)
//! fast_ma   = apply_ma(data, fast_period, fast_matype)  [trimmed to align with slow_ma]
//! macd_line = fast_ma_aligned − slow_ma
//! signal    = apply_ma(macd_line, signal_period, signal_matype)
//! hist      = macd_line_aligned − signal
//! ```
//!
//! # Parameters
//!
//! - `data`           — input price series
//! - `fast_period`    — fast MA period
//! - `fast_matype`    — fast MA type (0=SMA, 1=EMA, 2=WMA, …)
//! - `slow_period`    — slow MA period (must be ≥ `fast_period`)
//! - `slow_matype`    — slow MA type
//! - `signal_period`  — signal MA period
//! - `signal_matype`  — signal MA type
//!
//! # Output
//!
//! [`MacdOutput`] with three equally-sized `Vec<f64>`.
//! Returns all empty `Vec`s on invalid input.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::macdext;
//!
//! let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
//! // All EMA → identical to standard MACD
//! let out = macdext(&data, 12, 1, 26, 1, 9, 1);
//! assert_eq!(out.macd.len(), 100 - (26 + 9 - 2));
//! ```

use crate::trend::ma::apply_ma;
use crate::trend::ma::ma_lookback;
use crate::trend::macd::{macd, MacdOutput};

/// MACD with configurable MA types.
///
/// See [module documentation](self) for full details.
pub fn macdext(
    data: &[f64],
    fast_period: usize,
    fast_matype: usize,
    slow_period: usize,
    slow_matype: usize,
    signal_period: usize,
    signal_matype: usize,
) -> MacdOutput {
    let empty = MacdOutput { macd: vec![], signal: vec![], hist: vec![] };

    if fast_period == 0 || slow_period == 0 || signal_period == 0 {
        return empty;
    }
    if slow_period < fast_period {
        return empty;
    }

    // 全部 EMA → 复用带正确种子的 macd()，与 ta-lib MACDEXT(EMA,EMA,EMA) 数值一致
    if fast_matype == 1 && slow_matype == 1 && signal_matype == 1 {
        return macd(data, fast_period, slow_period, signal_period);
    }

    let slow_lb = ma_lookback(slow_period, slow_matype);
    let fast_lb = ma_lookback(fast_period, fast_matype);

    let slow_ma = apply_ma(data, slow_period, slow_matype);
    if slow_ma.is_empty() {
        return empty;
    }

    let fast_ma_full = apply_ma(data, fast_period, fast_matype);
    if fast_ma_full.is_empty() {
        return empty;
    }

    // 将 fast_ma 对齐到 slow_ma 的起始位置
    let align_trim = slow_lb.saturating_sub(fast_lb);
    if fast_ma_full.len() <= align_trim {
        return empty;
    }
    let fast_ma_aligned = &fast_ma_full[align_trim..];
    if fast_ma_aligned.len() != slow_ma.len() {
        return empty;
    }

    // MACD 线 = fast_ma_aligned - slow_ma
    let macd_line: Vec<f64> = fast_ma_aligned
        .iter()
        .zip(slow_ma.iter())
        .map(|(&f, &s)| f - s)
        .collect();

    // 信号线
    let signal = apply_ma(&macd_line, signal_period, signal_matype);
    if signal.is_empty() {
        return empty;
    }

    let sig_lb = ma_lookback(signal_period, signal_matype);
    if macd_line.len() <= sig_lb {
        return empty;
    }
    let macd_trimmed: Vec<f64> = macd_line[sig_lb..].to_vec();

    if macd_trimmed.len() != signal.len() {
        return empty;
    }

    let hist: Vec<f64> = macd_trimmed
        .iter()
        .zip(signal.iter())
        .map(|(&m, &s)| m - s)
        .collect();

    MacdOutput { macd: macd_trimmed, signal, hist }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trend::macd::macd;

    fn assert_close(actual: f64, expected: f64, eps: f64) {
        assert!(
            (actual - expected).abs() < eps || (actual.is_nan() && expected.is_nan()),
            "actual={actual:.15}, expected={expected:.15}",
        );
    }

    #[test]
    fn macdext_ema_matches_macd() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let ext = macdext(&data, 12, 1, 26, 1, 9, 1);
        let std = macd(&data, 12, 26, 9);
        assert_eq!(ext.macd.len(), std.macd.len());
        for (a, b) in ext.macd.iter().zip(std.macd.iter()) {
            assert_close(*a, *b, 1e-12);
        }
    }

    #[test]
    fn macdext_output_length() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let out = macdext(&data, 12, 1, 26, 1, 9, 1);
        assert_eq!(out.macd.len(), 100 - (26 + 9 - 2));
        assert_eq!(out.signal.len(), out.macd.len());
        assert_eq!(out.hist.len(), out.macd.len());
    }

    #[test]
    fn macdext_sma_output_length() {
        // SMA/SMA/SMA: same lookback formula as EMA for equal matypes
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let out = macdext(&data, 12, 0, 26, 0, 9, 0);
        let expected = 100 - (26 - 1) - (9 - 1);
        assert_eq!(out.macd.len(), expected);
    }

    #[test]
    fn macdext_too_short() {
        let data = vec![1.0f64; 10];
        let out = macdext(&data, 12, 1, 26, 1, 9, 1);
        assert!(out.macd.is_empty());
    }

    #[test]
    fn macdext_zero_period() {
        let data = vec![1.0f64; 50];
        assert!(macdext(&data, 0, 1, 26, 1, 9, 1).macd.is_empty());
    }

    #[test]
    fn macdext_hist_equals_macd_minus_signal() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let out = macdext(&data, 12, 1, 26, 1, 9, 1);
        for i in 0..out.hist.len() {
            assert_close(out.hist[i], out.macd[i] - out.signal[i], 1e-10);
        }
    }
}
