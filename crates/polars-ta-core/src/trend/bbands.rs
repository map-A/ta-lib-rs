//! Bollinger Bands (BBands)
//!
//! Computes upper, middle, and lower bands around a Simple Moving Average,
//! using population standard deviation — numerically identical to ta-lib's
//! `TA_BBANDS`.
//!
//! # Algorithm
//!
//! ```text
//! middle = SMA(data, period)
//! std    = sqrt( Σ(x - mean)² / N )    // population stddev (÷ N, not N-1)
//! upper  = middle + nbdev_up * std
//! lower  = middle - nbdev_dn * std
//! ```
//!
//! Standard deviation is computed with a two-pass method for numerical
//! stability (first compute mean, then sum squared deviations).
//!
//! # Parameters
//!
//! - `data`     — input price series (typically `close`)
//! - `period`   — SMA window length (≥ 1)
//! - `nbdev_up` — multiplier for upper band (typically 2.0)
//! - `nbdev_dn` — multiplier for lower band (typically 2.0)
//!
//! # Output
//!
//! [`BbandsOutput`] with three equally-sized `Vec<f64>`:
//! - `upper`  — upper band
//! - `middle` — middle band (SMA)
//! - `lower`  — lower band
//!
//! Length = `data.len() - (period - 1)`.
//! Returns all empty `Vec`s when input is too short.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::bbands;
//!
//! let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
//! let out = bbands(&data, 4, 2.0, 2.0);
//! assert_eq!(out.middle.len(), 5);
//! ```

/// Output of [`bbands`]: upper, middle, and lower bands.
pub struct BbandsOutput {
    /// Upper band (middle + nbdev_up × std).
    pub upper: Vec<f64>,
    /// Middle band (SMA).
    pub middle: Vec<f64>,
    /// Lower band (middle − nbdev_dn × std).
    pub lower: Vec<f64>,
}

/// Bollinger Bands.
///
/// See [module documentation](self) for full details.
pub fn bbands(data: &[f64], period: usize, nbdev_up: f64, nbdev_dn: f64) -> BbandsOutput {
    let empty = BbandsOutput {
        upper: vec![],
        middle: vec![],
        lower: vec![],
    };

    let n = data.len();
    if period == 0 || n < period {
        return empty;
    }

    let out_len = n - (period - 1);
    let mut upper = Vec::with_capacity(out_len);
    let mut middle = Vec::with_capacity(out_len);
    let mut lower = Vec::with_capacity(out_len);

    let pf = period as f64;

    // 滑动窗口 O(n)：同时维护 sum 和 sum_sq，避免内层 O(period) 循环
    let mut sum: f64 = data[..period].iter().sum();
    let mut sum_sq: f64 = data[..period].iter().map(|&x| x * x).sum();

    for start in 0..out_len {
        let mean = sum / pf;
        // 总体方差 = E[X²] - E[X]²，max(0) 防止浮点误差导致负数
        let variance = (sum_sq / pf - mean * mean).max(0.0);
        let std = variance.sqrt();

        upper.push(mean + nbdev_up * std);
        middle.push(mean);
        lower.push(mean - nbdev_dn * std);

        // 滑动更新 sum 和 sum_sq
        if start + period < n {
            let out_val = data[start];
            let in_val = data[start + period];
            sum += in_val - out_val;
            sum_sq += in_val * in_val - out_val * out_val;
        }
    }

    BbandsOutput {
        upper,
        middle,
        lower,
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
    fn bbands_output_length() {
        let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        let out = bbands(&data, 5, 2.0, 2.0);
        let expected = 20 - 4;
        assert_eq!(out.upper.len(), expected);
        assert_eq!(out.middle.len(), expected);
        assert_eq!(out.lower.len(), expected);
    }

    #[test]
    fn bbands_middle_is_sma() {
        // 验证 middle = SMA
        use crate::trend::sma::sma;
        let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        let out = bbands(&data, 5, 2.0, 2.0);
        let sma_out = sma(&data, 5);
        assert_eq!(out.middle.len(), sma_out.len());
        for (m, s) in out.middle.iter().zip(sma_out.iter()) {
            assert_close(*m, *s, 1e-10);
        }
    }

    #[test]
    fn bbands_known_values() {
        // 来自维基百科 Bollinger Bands 示例
        // data: [2,4,4,4,5,5,7,9], period=4, std as population
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let out = bbands(&data, 4, 2.0, 2.0);
        assert_eq!(out.middle.len(), 5);
        // 第一窗口 [2,4,4,4]: mean=3.5, pop_std = sqrt(((2-3.5)^2+(4-3.5)^2+(4-3.5)^2+(4-3.5)^2)/4)
        //  = sqrt((2.25+0.25+0.25+0.25)/4) = sqrt(3/4) = sqrt(0.75)
        let expected_mean = 3.5_f64;
        let expected_std = (0.75_f64).sqrt();
        assert_close(out.middle[0], expected_mean, 1e-10);
        assert_close(out.upper[0], expected_mean + 2.0 * expected_std, 1e-10);
        assert_close(out.lower[0], expected_mean - 2.0 * expected_std, 1e-10);
    }

    #[test]
    fn bbands_upper_minus_lower() {
        // upper - lower = (nbdev_up + nbdev_dn) * std
        let data: Vec<f64> = (1..=30).map(|x| x as f64).collect();
        let out = bbands(&data, 10, 2.0, 2.0);
        for i in 0..out.upper.len() {
            let width = out.upper[i] - out.lower[i];
            let half = out.upper[i] - out.middle[i];
            assert_close(width, 4.0 * (out.middle[i] - out.lower[i]) / 2.0, 1e-8);
            assert_close(width / 2.0, half, 1e-10);
        }
    }

    #[test]
    fn bbands_constant_series_zero_std() {
        // 常数序列：std=0，三条带重合
        let data = vec![5.0f64; 30];
        let out = bbands(&data, 10, 2.0, 2.0);
        for i in 0..out.upper.len() {
            assert_close(out.upper[i], 5.0, 1e-10);
            assert_close(out.middle[i], 5.0, 1e-10);
            assert_close(out.lower[i], 5.0, 1e-10);
        }
    }

    #[test]
    fn bbands_boundary_short() {
        let data = vec![1.0, 2.0];
        let out = bbands(&data, 3, 2.0, 2.0);
        assert!(out.upper.is_empty());
    }

    #[test]
    fn bbands_period_zero() {
        let data = vec![1.0, 2.0, 3.0];
        let out = bbands(&data, 0, 2.0, 2.0);
        assert!(out.upper.is_empty());
    }

    #[test]
    fn bbands_asymmetric_nbdev() {
        let data = vec![10.0f64; 20];
        let out = bbands(&data, 5, 1.5, 2.5);
        // std=0, so all bands = middle = 10
        for i in 0..out.upper.len() {
            assert_close(out.upper[i], 10.0, 1e-10);
            assert_close(out.lower[i], 10.0, 1e-10);
        }
    }
}
