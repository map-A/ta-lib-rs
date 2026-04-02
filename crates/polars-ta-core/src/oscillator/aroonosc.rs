//! Aroon Oscillator
//!
//! The difference between Aroon Up and Aroon Down — numerically identical to
//! ta-lib's `TA_AROONOSC`.
//!
//! # Algorithm
//!
//! ```text
//! aroonosc[i] = aroon_up[i] - aroon_down[i]
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
//! - Range: −100 to +100
//! - Returns empty vec when input is too short
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::oscillator::aroonosc;
//!
//! let high = vec![10.0, 11.0, 12.0, 11.0, 10.0, 11.0];
//! let low  = vec![ 8.0,  9.0, 10.0,  9.0,  8.0,  9.0];
//! let result = aroonosc(&high, &low, 5);
//! assert_eq!(result.len(), 1);
//! ```

use crate::oscillator::aroon::aroon;

/// Aroon Oscillator.
///
/// See [module documentation](self) for full details.
pub fn aroonosc(high: &[f64], low: &[f64], period: usize) -> Vec<f64> {
    let result = aroon(high, low, period);
    result
        .aroon_up
        .iter()
        .zip(result.aroon_down.iter())
        .map(|(up, down)| up - down)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aroonosc_output_length() {
        let n = 20_usize;
        let h = vec![10.0_f64; n];
        let l = vec![8.0_f64; n];
        assert_eq!(aroonosc(&h, &l, 14).len(), n - 14);
    }

    #[test]
    fn aroonosc_too_short() {
        let h = vec![10.0; 5];
        let l = vec![8.0; 5];
        assert!(aroonosc(&h, &l, 14).is_empty());
    }

    #[test]
    fn aroonosc_period_zero() {
        let h = vec![10.0; 10];
        let l = vec![8.0; 10];
        assert!(aroonosc(&h, &l, 0).is_empty());
    }

    #[test]
    fn aroonosc_range() {
        let n = 30_usize;
        let high: Vec<f64> = (0..n)
            .map(|i| (i as f64 * 0.3).sin() * 5.0 + 50.0)
            .collect();
        let low: Vec<f64> = (0..n)
            .map(|i| (i as f64 * 0.3).sin() * 5.0 + 48.0)
            .collect();
        let res = aroonosc(&high, &low, 14);
        for v in &res {
            assert!(*v >= -100.0 && *v <= 100.0, "aroonosc out of range: {v}");
        }
    }

    #[test]
    fn aroonosc_rising_trend() {
        // Monotonically increasing highs → aroon_up=100, aroon_down=0 → osc=100
        let n = 20_usize;
        let high: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let low: Vec<f64> = (0..n).map(|i| i as f64 - 1.0).collect();
        let res = aroonosc(&high, &low, 14);
        assert!(!res.is_empty());
        for v in &res {
            assert!((*v - 100.0).abs() < 1e-10, "expected 100.0, got {v}");
        }
    }

    #[test]
    fn aroonosc_falling_trend() {
        // Monotonically decreasing lows → aroon_down=100, aroon_up=0 → osc=-100
        let n = 20_usize;
        let high: Vec<f64> = (0..n).map(|i| 100.0 - i as f64 + 1.0).collect();
        let low: Vec<f64> = (0..n).map(|i| 100.0 - i as f64).collect();
        let res = aroonosc(&high, &low, 14);
        assert!(!res.is_empty());
        for v in &res {
            assert!((*v + 100.0).abs() < 1e-10, "expected -100.0, got {v}");
        }
    }
}
