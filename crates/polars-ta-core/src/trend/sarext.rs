//! Extended Parabolic SAR (SAREXT)
//!
//! Parabolic SAR with separate acceleration parameters for long and short
//! positions, plus an optional start value and reversal offset — numerically
//! identical to ta-lib's `TA_SAREXT`.
//!
//! With default parameters (`start_value=0`, `offset_on_reverse=0`,
//! `accel_init=accel_step=0.02`, `accel_max=0.2` for both directions) the
//! output is identical to `TA_SAR`.
//!
//! # Parameters
//!
//! - `high`               — high price series
//! - `low`                — low price series (same length as `high`)
//! - `start_value`        — initial SAR override (0.0 = use standard first-bar logic)
//! - `offset_on_reverse`  — added/subtracted from SAR when trend flips (typically 0.0)
//! - `accel_init_long`    — initial acceleration for long trend (default 0.02)
//! - `accel_long`         — acceleration step for long trend (default 0.02)
//! - `accel_max_long`     — maximum acceleration for long trend (default 0.20)
//! - `accel_init_short`   — initial acceleration for short trend (default 0.02)
//! - `accel_short`        — acceleration step for short trend (default 0.02)
//! - `accel_max_short`    — maximum acceleration for short trend (default 0.20)
//!
//! # Output
//!
//! - Length = `high.len() - 1` (lookback = 1)
//! - Returns an empty `Vec` when `high.len() < 2`
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::sarext;
//!
//! let high = vec![10.0, 11.0, 12.0, 11.5, 10.5];
//! let low  = vec![ 9.0,  9.5, 10.5, 10.0,  9.0];
//! // Default params = same as SAR(0.02, 0.20)
//! let result = sarext(&high, &low, 0.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20);
//! assert_eq!(result.len(), 4);
//! ```

/// Extended Parabolic Stop and Reverse.
///
/// See [module documentation](self) for full details.
#[allow(clippy::too_many_arguments)]
pub fn sarext(
    high: &[f64],
    low: &[f64],
    start_value: f64,
    offset_on_reverse: f64,
    accel_init_long: f64,
    accel_long: f64,
    accel_max_long: f64,
    accel_init_short: f64,
    accel_short: f64,
    accel_max_short: f64,
) -> Vec<f64> {
    let n = high.len();
    assert_eq!(n, low.len(), "high and low must have equal length");

    if n < 2 {
        return vec![];
    }

    let out_len = n - 1;
    let mut out = Vec::with_capacity(out_len);

    let mut is_long = high[1] >= high[0];

    let (mut ep, mut sar) = if is_long {
        let initial_sar = if start_value != 0.0 { start_value } else { low[0] };
        (high[1], initial_sar)
    } else {
        let initial_sar = if start_value != 0.0 { start_value } else { high[0] };
        (low[1], initial_sar)
    };

    let mut acc = if is_long { accel_init_long } else { accel_init_short };

    for i in 1..n {
        let raw_sar = sar + acc * (ep - sar);

        let new_sar = if is_long {
            let mut clamped = raw_sar.min(low[i - 1]);
            if i >= 3 { clamped = clamped.min(low[i - 2]); }

            if low[i] <= clamped {
                // 翻转到空头
                let old_ep = ep;
                is_long = false;
                ep = low[i];
                acc = accel_init_short;
                let mut s = old_ep.max(high[i - 1]);
                if i >= 3 { s = s.max(high[i - 2]); }
                s + offset_on_reverse
            } else {
                if high[i] > ep {
                    ep = high[i];
                    acc = (acc + accel_long).min(accel_max_long);
                }
                clamped
            }
        } else {
            let mut clamped = raw_sar.max(high[i - 1]);
            if i >= 3 { clamped = clamped.max(high[i - 2]); }

            if high[i] >= clamped {
                // 翻转到多头
                let old_ep = ep;
                is_long = true;
                ep = high[i];
                acc = accel_init_long;
                let mut s = old_ep.min(low[i - 1]);
                if i >= 3 { s = s.min(low[i - 2]); }
                s - offset_on_reverse
            } else {
                if low[i] < ep {
                    ep = low[i];
                    acc = (acc + accel_short).min(accel_max_short);
                }
                clamped
            }
        };

        // ta-lib SAREXT convention: negative output signals a short position
        out.push(if is_long { new_sar } else { -new_sar });
        sar = new_sar;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trend::sar::sar;

    fn assert_close(actual: f64, expected: f64, eps: f64) {
        assert!(
            (actual - expected).abs() < eps || (actual.is_nan() && expected.is_nan()),
            "actual={actual:.15}, expected={expected:.15}",
        );
    }

    fn sarext_default(high: &[f64], low: &[f64]) -> Vec<f64> {
        sarext(high, low, 0.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20)
    }

    #[test]
    fn sarext_output_length() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 10.5];
        let low = vec![9.0, 9.5, 10.5, 10.0, 9.0];
        assert_eq!(sarext_default(&high, &low).len(), 4);
    }

    #[test]
    fn sarext_too_short() {
        assert!(sarext_default(&[10.0], &[9.0]).is_empty());
        assert!(sarext_default(&[], &[]).is_empty());
    }

    #[test]
    fn sarext_default_equals_sar() {
        // SAREXT(default) == SAR(0.02, 0.20) in absolute value;
        // SAREXT negates the output for short positions while SAR does not.
        let n = 50;
        let high: Vec<f64> = (0..n).map(|i| 100.0 + (i as f64).sin() * 5.0 + i as f64 * 0.1).collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 2.0).collect();
        let ext = sarext_default(&high, &low);
        let std = sar(&high, &low, 0.02, 0.20);
        assert_eq!(ext.len(), std.len());
        for (a, b) in ext.iter().zip(std.iter()) {
            assert_close(a.abs(), *b, 1e-12);
        }
    }

    #[test]
    fn sarext_all_finite() {
        let n = 50;
        let high: Vec<f64> = (0..n).map(|i| 100.0 + (i as f64).sin() * 5.0 + i as f64 * 0.1).collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 2.0).collect();
        for &v in &sarext_default(&high, &low) {
            assert!(v.is_finite());
        }
    }

    #[test]
    fn sarext_separate_accel() {
        // Different accel for long/short should still produce finite results
        let n = 50;
        let high: Vec<f64> = (0..n).map(|i| 100.0 + (i as f64).sin() * 5.0 + i as f64 * 0.1).collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 2.0).collect();
        let res = sarext(&high, &low, 0.0, 0.0, 0.01, 0.01, 0.10, 0.03, 0.03, 0.30);
        assert_eq!(res.len(), n - 1);
        for &v in &res {
            assert!(v.is_finite());
        }
    }
}
