//! Williams %R
//!
//! A momentum indicator measuring overbought/oversold levels by comparing
//! the close to the high-low range. Numerically identical to ta-lib's `TA_WILLR`.
//!
//! # Algorithm
//!
//! ```text
//! For each window of `period` bars ending at index i:
//!   highest_high = max(high[i-period+1..=i])
//!   lowest_low   = min(low[i-period+1..=i])
//!   willr = (highest_high - close[i]) / (highest_high - lowest_low) * -100
//! ```
//!
//! # Parameters
//!
//! - `high`   — high price series
//! - `low`    — low price series
//! - `close`  — close price series
//! - `period` — lookback window
//!
//! # Output
//!
//! - Length = `n - (period - 1)`
//! - Returns empty `Vec` when input is too short
//! - Range: -100 to 0 (values near 0 = overbought, near -100 = oversold)
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::oscillator::willr;
//!
//! let high  = vec![10.0, 11.0, 12.0, 11.0, 10.0];
//! let low   = vec![ 8.0,  9.0, 10.0,  9.0,  8.0];
//! let close = vec![ 9.0, 10.0, 11.0, 10.0,  9.0];
//! let result = willr(&high, &low, &close, 3);
//! assert_eq!(result.len(), 3);
//! ```

/// Williams %R.
///
/// See [module documentation](self) for full details.
pub fn willr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    let n = close.len();
    if period == 0 || n < period || high.len() != n || low.len() != n {
        return vec![];
    }

    let out_len = n - (period - 1);
    let mut out = vec![0.0f64; out_len];

    // ta-lib 条件重扫算法（NaN 初始化，与 max.rs/min.rs 完全一致）
    let mut highest = f64::NAN;
    let mut highest_idx: isize = -1;
    let mut lowest = f64::NAN;
    let mut lowest_idx: isize = -1;

    for i in 0..out_len {
        let trail = i as isize;
        let newest = i + period - 1;

        if highest_idx < trail {
            highest = high[i];
            highest_idx = trail;
            for j in (i + 1)..=newest {
                if high[j] > highest {
                    highest = high[j];
                    highest_idx = j as isize;
                }
            }
        } else if high[newest] > highest {
            highest = high[newest];
            highest_idx = newest as isize;
        }

        if lowest_idx < trail {
            lowest = low[i];
            lowest_idx = trail;
            for j in (i + 1)..=newest {
                if low[j] < lowest {
                    lowest = low[j];
                    lowest_idx = j as isize;
                }
            }
        } else if low[newest] < lowest {
            lowest = low[newest];
            lowest_idx = newest as isize;
        }

        let ci = close[newest];
        let diff = highest - lowest;
        out[i] = if ci.is_nan() || highest.is_nan() || lowest.is_nan() {
            f64::NAN
        } else if diff.abs() < f64::EPSILON {
            0.0
        } else {
            (highest - ci) / diff * -100.0
        };
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64, eps: f64) {
        assert!(
            (actual - expected).abs() < eps || (actual.is_nan() && expected.is_nan()),
            "actual={actual:.10}, expected={expected:.10}",
        );
    }

    #[test]
    fn willr_output_length() {
        let n = 20_usize;
        let h = vec![10.0_f64; n];
        let l = vec![8.0_f64; n];
        let c = vec![9.0_f64; n];
        let result = willr(&h, &l, &c, 14);
        assert_eq!(result.len(), n - 13);
    }

    #[test]
    fn willr_too_short() {
        let h = vec![10.0, 11.0];
        let l = vec![8.0, 9.0];
        let c = vec![9.0, 10.0];
        assert!(willr(&h, &l, &c, 5).is_empty());
    }

    #[test]
    fn willr_period_zero() {
        let h = vec![10.0; 10];
        let l = vec![8.0; 10];
        let c = vec![9.0; 10];
        assert!(willr(&h, &l, &c, 0).is_empty());
    }

    #[test]
    fn willr_range() {
        let high = vec![10.0, 11.0, 12.0, 11.0, 10.0, 12.0, 13.0, 11.0];
        let low = vec![8.0, 9.0, 10.0, 9.0, 8.0, 10.0, 11.0, 9.0];
        let close = vec![9.0, 10.0, 11.0, 10.0, 9.0, 11.0, 12.0, 10.0];
        let result = willr(&high, &low, &close, 3);
        for v in &result {
            assert!(*v >= -100.0 && *v <= 0.0, "Williams %R out of range: {v}");
        }
    }

    #[test]
    fn willr_close_at_high() {
        // close == highest_high → %R = 0
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![8.0, 9.0, 10.0];
        let close = vec![10.0, 11.0, 12.0]; // close = high
        let result = willr(&high, &low, &close, 3);
        assert_eq!(result.len(), 1);
        assert_close(result[0], 0.0, 1e-10);
    }

    #[test]
    fn willr_close_at_low() {
        // close[i] == lowest_low of window → %R = -100
        // With uniform high/low, close == ll = 8 gives (12-8)/(12-8)*-100 = -100
        let high = vec![12.0, 12.0, 12.0];
        let low = vec![8.0, 8.0, 8.0];
        let close = vec![12.0, 12.0, 8.0]; // last close = lowest_low
        let result = willr(&high, &low, &close, 3);
        assert_eq!(result.len(), 1);
        assert_close(result[0], -100.0, 1e-10);
    }

    #[test]
    fn willr_equal_high_low() {
        // hh == ll → %R = 0
        let h = vec![5.0_f64; 10];
        let l = vec![5.0_f64; 10];
        let c = vec![5.0_f64; 10];
        let result = willr(&h, &l, &c, 5);
        for v in &result {
            assert_close(*v, 0.0, 1e-10);
        }
    }

    #[test]
    fn willr_mismatched_lengths() {
        let h = vec![10.0; 10];
        let l = vec![8.0; 9];
        let c = vec![9.0; 10];
        assert!(willr(&h, &l, &c, 5).is_empty());
    }
}
