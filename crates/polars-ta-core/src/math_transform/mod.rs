//! Math Transform — Element-wise Mathematical Functions
//!
//! 数学变换 — 对价格序列逐元素应用标准数学函数。
//!
//! All 15 functions operate element-wise: `output[i] = f(input[i])`.
//! Lookback = 0, so output length always equals input length.
//! NaN inputs propagate to NaN outputs automatically via IEEE 754 arithmetic —
//! no special NaN handling is required.
//!
//! Numerically identical to the corresponding `TA_*` functions in ta-lib C.
//!
//! # Available Functions
//!
//! | Function | ta-lib Name | Notes |
//! |----------|-------------|-------|
//! | [`acos`]  | `TA_ACOS`  | Arc cosine; NaN for \|x\| > 1 |
//! | [`asin`]  | `TA_ASIN`  | Arc sine; NaN for \|x\| > 1 |
//! | [`atan`]  | `TA_ATAN`  | Arc tangent |
//! | [`ceil`]  | `TA_CEIL`  | Round up to nearest integer |
//! | [`cos`]   | `TA_COS`   | Cosine (radians) |
//! | [`cosh`]  | `TA_COSH`  | Hyperbolic cosine |
//! | [`exp`]   | `TA_EXP`   | Natural exponential eˣ |
//! | [`floor`] | `TA_FLOOR` | Round down to nearest integer |
//! | [`ln`]    | `TA_LN`    | Natural logarithm; NaN for x ≤ 0 |
//! | [`log10`] | `TA_LOG10` | Base-10 logarithm; NaN for x ≤ 0 |
//! | [`sin`]   | `TA_SIN`   | Sine (radians) |
//! | [`sinh`]  | `TA_SINH`  | Hyperbolic sine |
//! | [`sqrt`]  | `TA_SQRT`  | Square root; NaN for x < 0 |
//! | [`tan`]   | `TA_TAN`   | Tangent (radians) |
//! | [`tanh`]  | `TA_TANH`  | Hyperbolic tangent |
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::math_transform::{sqrt, ln, exp};
//!
//! let data = vec![1.0, 4.0, 9.0, 16.0];
//!
//! let roots = sqrt(&data);
//! assert!((roots[2] - 3.0).abs() < 1e-10);  // sqrt(9) = 3
//!
//! let logs = ln(&data);
//! assert!((logs[0] - 0.0).abs() < 1e-10);   // ln(1) = 0
//!
//! let exps = exp(&[0.0, 1.0]);
//! assert!((exps[1] - std::f64::consts::E).abs() < 1e-10);
//! ```

pub fn acos(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.acos()).collect()
}

pub fn asin(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.asin()).collect()
}

pub fn atan(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.atan()).collect()
}

pub fn ceil(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.ceil()).collect()
}

pub fn cos(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.cos()).collect()
}

pub fn cosh(data: &[f64]) -> Vec<f64> {
    data.iter()
        .map(|&x| {
            let r = x.cosh();
            if r.is_finite() {
                r
            } else {
                f64::NAN
            }
        })
        .collect()
}

pub fn exp(data: &[f64]) -> Vec<f64> {
    data.iter()
        .map(|&x| {
            let r = x.exp();
            if r.is_finite() {
                r
            } else {
                f64::NAN
            }
        })
        .collect()
}

pub fn floor(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.floor()).collect()
}

pub fn ln(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.ln()).collect()
}

pub fn log10(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.log10()).collect()
}

pub fn sin(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.sin()).collect()
}

pub fn sinh(data: &[f64]) -> Vec<f64> {
    data.iter()
        .map(|&x| {
            let r = x.sinh();
            if r.is_finite() {
                r
            } else {
                f64::NAN
            }
        })
        .collect()
}

pub fn sqrt(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.sqrt()).collect()
}

pub fn tan(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.tan()).collect()
}

pub fn tanh(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.tanh()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acos_known_values() {
        let r = acos(&[1.0, 0.0, -1.0]);
        assert!((r[0] - 0.0).abs() < 1e-10);
        assert!((r[1] - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
        assert!((r[2] - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn acos_out_of_range_is_nan() {
        let r = acos(&[2.0]);
        assert!(r[0].is_nan());
    }

    #[test]
    fn sqrt_known() {
        let r = sqrt(&[4.0, 9.0, 16.0]);
        assert!((r[0] - 2.0).abs() < 1e-10);
        assert!((r[1] - 3.0).abs() < 1e-10);
        assert!((r[2] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn ceil_floor_known() {
        let r_ceil = ceil(&[1.2, 2.8, -1.2]);
        assert_eq!(r_ceil, vec![2.0, 3.0, -1.0]);
        let r_floor = floor(&[1.2, 2.8, -1.2]);
        assert_eq!(r_floor, vec![1.0, 2.0, -2.0]);
    }
}
