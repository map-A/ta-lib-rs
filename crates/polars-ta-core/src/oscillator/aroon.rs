//! Aroon Oscillator
//!
//! Identifies trends and trend changes by measuring how long ago the highest high
//! and lowest low occurred within a given lookback window.
//! Numerically identical to ta-lib's `TA_AROON`.
//!
//! # Algorithm
//!
//! ```text
//! For each output position (window of period+1 bars):
//!   bars_since_hh = period - argmax(high[i-period..=i])
//!   bars_since_ll = period - argmin(low[i-period..=i])
//!   aroon_up   = bars_since_hh / period * 100
//!              = (period - bars_since_hh) ... wait, correct formula:
//!   aroon_up   = (period - (i - index_of_hh)) / period * 100
//!   aroon_down = (period - (i - index_of_ll)) / period * 100
//! ```
//!
//! # Parameters
//!
//! - `high`   — high price series
//! - `low`    — low price series
//! - `period` — lookback window
//!
//! # Output
//!
//! - Length = `n - period`
//! - Returns empty vecs when input is too short
//! - Range: 0 to 100 for both aroon_up and aroon_down
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::oscillator::aroon;
//!
//! let high = vec![10.0, 11.0, 12.0, 11.0, 10.0, 11.0];
//! let low  = vec![ 8.0,  9.0, 10.0,  9.0,  8.0,  9.0];
//! let result = aroon(&high, &low, 5);
//! assert_eq!(result.aroon_up.len(), 1);
//! assert_eq!(result.aroon_down.len(), 1);
//! ```

/// Output of the Aroon indicator.
pub struct AroonOutput {
    /// Aroon Up: measures bars since the highest high.
    pub aroon_up: Vec<f64>,
    /// Aroon Down: measures bars since the lowest low.
    pub aroon_down: Vec<f64>,
}

/// Aroon indicator.
///
/// See [module documentation](self) for full details.
pub fn aroon(high: &[f64], low: &[f64], period: usize) -> AroonOutput {
    let empty = AroonOutput { aroon_up: vec![], aroon_down: vec![] };
    let n = high.len();
    if period == 0 || n <= period || low.len() != n {
        return empty;
    }

    let out_len = n - period;
    let inv_period = 100.0 / period as f64;

    // Power-of-2 ring buffer for O(1) amortized push/pop without modulo cost
    let cap = (period + 2).next_power_of_two();
    let mask = cap - 1;
    let mut max_buf = vec![0usize; cap];
    let mut min_buf = vec![0usize; cap];
    let mut max_f = 0usize;
    let mut max_b = 0usize; // front..back (mod cap), empty when max_f == max_b
    let mut min_f = 0usize;
    let mut min_b = 0usize;

    let mut aroon_up   = vec![0.0f64; out_len];
    let mut aroon_down = vec![0.0f64; out_len];

    for i in 0..n {
        let hi = high[i];
        let lo = low[i];

        while max_f != max_b {
            let back_idx = max_buf[(max_b.wrapping_sub(1)) & mask];
            if high[back_idx] <= hi {
                max_b = max_b.wrapping_sub(1);
            } else {
                break;
            }
        }
        max_buf[max_b & mask] = i;
        max_b = max_b.wrapping_add(1);

        while min_f != min_b {
            let back_idx = min_buf[(min_b.wrapping_sub(1)) & mask];
            if low[back_idx] >= lo {
                min_b = min_b.wrapping_sub(1);
            } else {
                break;
            }
        }
        min_buf[min_b & mask] = i;
        min_b = min_b.wrapping_add(1);

        if i >= period {
            let window_start = i - period;
            while max_f != max_b && max_buf[max_f & mask] < window_start {
                max_f = max_f.wrapping_add(1);
            }
            while min_f != min_b && min_buf[min_f & mask] < window_start {
                min_f = min_f.wrapping_add(1);
            }

            let out_i = i - period;
            let hh_idx = max_buf[max_f & mask];
            let ll_idx = min_buf[min_f & mask];
            let bars_since_hh = i - hh_idx;
            let bars_since_ll = i - ll_idx;
            aroon_up[out_i] = (period - bars_since_hh) as f64 * inv_period;
            aroon_down[out_i] = (period - bars_since_ll) as f64 * inv_period;
        }
    }

    AroonOutput { aroon_up, aroon_down }
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
    fn aroon_output_length() {
        let n = 20_usize;
        let h = vec![10.0_f64; n];
        let l = vec![8.0_f64; n];
        let result = aroon(&h, &l, 14);
        assert_eq!(result.aroon_up.len(), n - 14);
        assert_eq!(result.aroon_down.len(), n - 14);
    }

    #[test]
    fn aroon_too_short() {
        let h = vec![10.0; 5];
        let l = vec![8.0; 5];
        let res = aroon(&h, &l, 14);
        assert!(res.aroon_up.is_empty());
        assert!(res.aroon_down.is_empty());
    }

    #[test]
    fn aroon_period_zero() {
        let h = vec![10.0; 10];
        let l = vec![8.0; 10];
        let res = aroon(&h, &l, 0);
        assert!(res.aroon_up.is_empty());
    }

    #[test]
    fn aroon_range() {
        let n = 30_usize;
        let high: Vec<f64> = (0..n).map(|i| (i as f64 * 0.3).sin() * 5.0 + 50.0).collect();
        let low:  Vec<f64> = (0..n).map(|i| (i as f64 * 0.3).sin() * 5.0 + 48.0).collect();
        let res = aroon(&high, &low, 14);
        for v in &res.aroon_up {
            assert!(*v >= 0.0 && *v <= 100.0, "aroon_up out of range: {v}");
        }
        for v in &res.aroon_down {
            assert!(*v >= 0.0 && *v <= 100.0, "aroon_down out of range: {v}");
        }
    }

    #[test]
    fn aroon_rising_trend() {
        // Monotonically increasing highs → highest high is always most recent → aroon_up = 100
        let n = 20_usize;
        let high: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let low:  Vec<f64> = (0..n).map(|i| i as f64 - 1.0).collect();
        let res = aroon(&high, &low, 14);
        assert!(!res.aroon_up.is_empty());
        for v in &res.aroon_up {
            assert_close(*v, 100.0, 1e-10);
        }
    }

    #[test]
    fn aroon_falling_trend() {
        // Monotonically decreasing lows → lowest low is always most recent → aroon_down = 100
        let n = 20_usize;
        let high: Vec<f64> = (0..n).map(|i| 100.0 - i as f64 + 1.0).collect();
        let low:  Vec<f64> = (0..n).map(|i| 100.0 - i as f64).collect();
        let res = aroon(&high, &low, 14);
        assert!(!res.aroon_down.is_empty());
        for v in &res.aroon_down {
            assert_close(*v, 100.0, 1e-10);
        }
    }

    #[test]
    fn aroon_mismatched_lengths() {
        let h = vec![10.0; 20];
        let l = vec![8.0; 19]; // mismatch
        let res = aroon(&h, &l, 14);
        assert!(res.aroon_up.is_empty());
    }
}
