//! Fast Stochastic Oscillator (STOCHF)
//!
//! Raw %K without the slowK smoothing step — numerically identical to
//! ta-lib's `TA_STOCHF`.
//!
//! # Algorithm
//!
//! ```text
//! rawK[i] = (close[i] - lowest_low(i, fastk_period))
//!         / (highest_high(i, fastk_period) - lowest_low(i, fastk_period)) * 100
//! fastK = rawK  (no smoothing, unlike STOCH which adds a slowK SMA step)
//! fastD = SMA(fastK, fastd_period)
//! ```
//!
//! Both `fastK` and `fastD` are output starting from the combined lookback:
//! `fastk_period + fastd_period − 2`.  The returned `fastk` slice has already
//! been trimmed to match `fastd`'s length (ta-lib convention).
//!
//! # Parameters
//!
//! - `high`          — high price series
//! - `low`           — low price series
//! - `close`         — close price series
//! - `fastk_period`  — raw %K window
//! - `fastd_period`  — %D smoothing period (SMA)
//!
//! # Output
//!
//! [`StochFOutput`] with two equally-sized `Vec<f64>`:
//! - `fastk` — trimmed fast %K
//! - `fastd` — fast %D (SMA of raw fastK)
//!
//! Length = `n − (fastk_period + fastd_period − 2)`.
//! Returns empty vecs when input is too short.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::oscillator::stochf;
//!
//! let high  = vec![10.0, 11.0, 12.0, 11.0, 10.0, 11.0, 12.0, 11.0, 10.0];
//! let low   = vec![ 9.0, 10.0, 11.0, 10.0,  9.0, 10.0, 11.0, 10.0,  9.0];
//! let close = vec![ 9.5, 10.5, 11.5, 10.5,  9.5, 10.5, 11.5, 10.5,  9.5];
//! let result = stochf(&high, &low, &close, 5, 3);
//! assert_eq!(result.fastk.len(), result.fastd.len());
//! ```

/// Output of the Fast Stochastic Oscillator.
pub struct StochFOutput {
    /// Fast %K (trimmed to match fastD length).
    pub fastk: Vec<f64>,
    /// Fast %D (SMA of raw fast %K).
    pub fastd: Vec<f64>,
}

/// Fast Stochastic Oscillator.
///
/// See [module documentation](self) for full details.
pub fn stochf(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    fastk_period: usize,
    fastd_period: usize,
) -> StochFOutput {
    let empty = StochFOutput { fastk: vec![], fastd: vec![] };

    let n = close.len();
    if fastk_period == 0 || fastd_period == 0 {
        return empty;
    }
    if high.len() != n || low.len() != n {
        return empty;
    }

    // 完整 lookback = fastk_period - 1 (rawK) + fastd_period - 1 (fastD SMA)
    let lookback = fastk_period + fastd_period - 2;
    if n <= lookback {
        return empty;
    }

    // Step 1: 计算原始 fastK，长度 = n - (fastk_period - 1)
    let rawk_len = n - (fastk_period - 1);
    let mut raw_fastk = Vec::with_capacity(rawk_len);
    for i in (fastk_period - 1)..n {
        let start = i + 1 - fastk_period;
        let mut hh = high[start];
        let mut ll = low[start];
        for j in (start + 1)..=i {
            if high[j] > hh { hh = high[j]; }
            if low[j] < ll { ll = low[j]; }
        }
        let fk = if (hh - ll).abs() < f64::EPSILON {
            0.0
        } else {
            (close[i] - ll) / (hh - ll) * 100.0
        };
        raw_fastk.push(fk);
    }

    // Step 2: fastD = SMA(rawFastK, fastd_period)
    let fastd = sma(&raw_fastk, fastd_period);

    // Step 3: 将 rawFastK 截断以匹配 fastD 的长度（ta-lib 惯例）
    let trim = fastd_period - 1;
    let fastk = raw_fastk[trim..].to_vec();

    debug_assert_eq!(fastk.len(), fastd.len());
    StochFOutput { fastk, fastd }
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

    fn assert_close(actual: f64, expected: f64, eps: f64) {
        assert!(
            (actual - expected).abs() < eps || (actual.is_nan() && expected.is_nan()),
            "actual={actual:.10}, expected={expected:.10}",
        );
    }

    #[test]
    fn stochf_output_length() {
        let n = 50_usize;
        let high: Vec<f64> = (0..n).map(|i| i as f64 + 1.0).collect();
        let low: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let close: Vec<f64> = (0..n).map(|i| i as f64 + 0.5).collect();
        let res = stochf(&high, &low, &close, 5, 3);
        let expected_len = n - (5 + 3 - 2);
        assert_eq!(res.fastk.len(), expected_len);
        assert_eq!(res.fastd.len(), expected_len);
    }

    #[test]
    fn stochf_too_short() {
        let h = vec![1.0, 2.0];
        let l = vec![0.0, 1.0];
        let c = vec![0.5, 1.5];
        let res = stochf(&h, &l, &c, 5, 3);
        assert!(res.fastk.is_empty());
        assert!(res.fastd.is_empty());
    }

    #[test]
    fn stochf_period_zero() {
        let h = vec![1.0; 20];
        let l = vec![0.0; 20];
        let c = vec![0.5; 20];
        assert!(stochf(&h, &l, &c, 0, 3).fastk.is_empty());
        assert!(stochf(&h, &l, &c, 5, 0).fastk.is_empty());
    }

    #[test]
    fn stochf_range() {
        let n = 100_usize;
        let high: Vec<f64> = (0..n).map(|i| (i as f64).sin() * 10.0 + 50.0 + 1.0).collect();
        let low: Vec<f64> = (0..n).map(|i| (i as f64).sin() * 10.0 + 50.0 - 1.0).collect();
        let close: Vec<f64> = (0..n).map(|i| (i as f64).sin() * 10.0 + 50.0).collect();
        let res = stochf(&high, &low, &close, 5, 3);
        for k in &res.fastk {
            assert!(*k >= 0.0 && *k <= 100.0, "fastk out of range: {k}");
        }
        for d in &res.fastd {
            assert!(*d >= 0.0 && *d <= 100.0, "fastd out of range: {d}");
        }
    }

    #[test]
    fn stochf_equal_high_low() {
        let h = vec![5.0_f64; 20];
        let l = vec![5.0_f64; 20];
        let c = vec![5.0_f64; 20];
        let res = stochf(&h, &l, &c, 5, 3);
        assert!(!res.fastk.is_empty());
        for v in &res.fastk {
            assert_close(*v, 0.0, 1e-10);
        }
    }

    #[test]
    fn stochf_fastk_fastd_same_len() {
        let n = 30_usize;
        let h: Vec<f64> = (0..n).map(|i| i as f64 + 2.0).collect();
        let l: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let c: Vec<f64> = (0..n).map(|i| i as f64 + 1.0).collect();
        let res = stochf(&h, &l, &c, 5, 3);
        assert_eq!(res.fastk.len(), res.fastd.len());
    }

    #[test]
    fn stochf_fastd1_equals_fastk() {
        // fastd_period=1 → SMA(fastK, 1) = fastK, no trimming needed
        let n = 20_usize;
        let h: Vec<f64> = (0..n).map(|i| i as f64 + 2.0).collect();
        let l: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let c: Vec<f64> = (0..n).map(|i| i as f64 + 1.0).collect();
        let res = stochf(&h, &l, &c, 5, 1);
        assert_eq!(res.fastk.len(), res.fastd.len());
        for (k, d) in res.fastk.iter().zip(res.fastd.iter()) {
            assert_close(*k, *d, 1e-10);
        }
    }
}
