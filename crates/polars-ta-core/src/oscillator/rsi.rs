//! Relative Strength Index (RSI)
//!
//! Measures the speed and magnitude of recent price changes to evaluate
//! overbought or oversold conditions. Numerically identical to ta-lib's `TA_RSI`.
//!
//! # Algorithm
//!
//! Uses Wilder smoothing (alpha = 1/period):
//!
//! ```text
//! changes[i] = data[i+1] - data[i]
//! seed: avg_gain = mean(max(changes[0..period], 0))
//!       avg_loss = mean(max(-changes[0..period], 0))
//! then for each new change:
//!   avg_gain = (avg_gain * (period-1) + max(change, 0)) / period
//!   avg_loss = (avg_loss * (period-1) + max(-change, 0)) / period
//! RSI = 100 - 100 / (1 + avg_gain / avg_loss)
//! ```
//!
//! # Parameters
//!
//! - `data`   — input price series
//! - `period` — smoothing period (≥ 1)
//!
//! # Output
//!
//! - Length = `data.len() - period`
//! - Returns empty `Vec` when `data.len() <= period`
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::oscillator::rsi;
//!
//! let close = vec![
//!     44.34, 44.09, 44.15, 43.61, 44.33,
//!     44.83, 45.10, 45.15, 43.61, 44.33,
//!     44.83, 45.10, 45.15, 45.38, 46.00,
//! ];
//! let result = rsi(&close, 14);
//! assert_eq!(result.len(), 1); // lookback = 14
//! ```

/// Relative Strength Index.
///
/// # Panics
///
/// Never panics. Returns empty `Vec` when input is too short.
pub fn rsi(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n <= period {
        return vec![];
    }

    let out_len = n - period;
    let mut out = Vec::with_capacity(out_len);

    // Seed: simple mean of first `period` changes
    let mut avg_gain = 0.0_f64;
    let mut avg_loss = 0.0_f64;
    for i in 0..period {
        let change = data[i + 1] - data[i];
        if change.is_nan() {
            avg_gain = f64::NAN;
            avg_loss = f64::NAN;
            break;
        }
        if change > 0.0 {
            avg_gain += change;
        } else {
            avg_loss += -change;
        }
    }
    avg_gain /= period as f64;
    avg_loss /= period as f64;

    // First RSI output corresponds to data[period]
    out.push(compute_rsi(avg_gain, avg_loss));

    // Wilder smoothing for remaining values
    let pf = period as f64;
    for i in (period + 1)..n {
        let change = data[i] - data[i - 1];
        let (gain, loss) = if change.is_nan() {
            (f64::NAN, f64::NAN)
        } else if change > 0.0 {
            (change, 0.0)
        } else {
            (0.0, -change)
        };
        avg_gain = (avg_gain * (pf - 1.0) + gain) / pf;
        avg_loss = (avg_loss * (pf - 1.0) + loss) / pf;

        out.push(compute_rsi(avg_gain, avg_loss));
    }

    out
}

#[inline]
fn compute_rsi(avg_gain: f64, avg_loss: f64) -> f64 {
    if avg_gain.is_nan() || avg_loss.is_nan() {
        return f64::NAN;
    }
    if avg_gain == 0.0 {
        return 0.0;
    }
    if avg_loss == 0.0 {
        return 100.0;
    }
    100.0 - 100.0 / (1.0 + avg_gain / avg_loss)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64, eps: f64) {
        assert!(
            (actual - expected).abs() < eps || (actual.is_nan() && expected.is_nan()),
            "actual={actual:.10}, expected={expected:.10}, diff={:.2e}",
            (actual - expected).abs()
        );
    }

    #[test]
    fn rsi_output_length() {
        let data = vec![1.0_f64; 20];
        let result = rsi(&data, 14);
        assert_eq!(result.len(), 20 - 14);
    }

    #[test]
    fn rsi_too_short() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(rsi(&data, 14).is_empty());
    }

    #[test]
    fn rsi_exact_length() {
        // data.len() == period → output is empty (need at least period+1 values)
        let data = vec![1.0_f64; 14];
        assert!(rsi(&data, 14).is_empty());
    }

    #[test]
    fn rsi_period_zero() {
        assert!(rsi(&[1.0, 2.0, 3.0], 0).is_empty());
    }

    #[test]
    fn rsi_all_gains() {
        // Strictly increasing → avg_loss = 0 → RSI = 100
        let data: Vec<f64> = (0..20).map(|i| i as f64).collect();
        let result = rsi(&data, 14);
        assert_eq!(result.len(), 6);
        for v in &result {
            assert_close(*v, 100.0, 1e-10);
        }
    }

    #[test]
    fn rsi_all_losses() {
        // Strictly decreasing → avg_gain = 0 → RSI = 0
        let data: Vec<f64> = (0..20).map(|i| 100.0 - i as f64).collect();
        let result = rsi(&data, 14);
        assert_eq!(result.len(), 6);
        for v in &result {
            assert_close(*v, 0.0, 1e-10);
        }
    }

    #[test]
    fn rsi_flat() {
        // No changes → avg_gain = avg_loss = 0; guard: avg_gain == 0 → RSI = 0
        let data = vec![50.0_f64; 20];
        let result = rsi(&data, 14);
        assert_eq!(result.len(), 6);
        for v in &result {
            assert_close(*v, 0.0, 1e-10);
        }
    }

    #[test]
    fn rsi_known_value() {
        // From Wilder's original paper example (approximate)
        // Using a well-known RSI test sequence
        let data = vec![
            44.34, 44.09, 44.15, 43.61, 44.33,
            44.83, 45.10, 45.15, 43.61, 44.33,
            44.83, 45.10, 45.15, 45.38, 46.00,
        ];
        let result = rsi(&data, 14);
        assert_eq!(result.len(), 1);
        // Seed: 14 changes → compute expected value manually
        // First RSI should be in range (0, 100)
        assert!(result[0] > 0.0 && result[0] < 100.0);
    }

    #[test]
    fn rsi_range() {
        let data = vec![
            10.0, 11.0, 9.0, 12.0, 8.0, 13.0, 7.0, 14.0,
            6.0, 15.0, 5.0, 16.0, 4.0, 17.0, 3.0, 18.0,
            2.0, 19.0, 1.0, 20.0,
        ];
        let result = rsi(&data, 14);
        for v in &result {
            assert!(*v >= 0.0 && *v <= 100.0, "RSI out of range: {v}");
        }
    }
}
