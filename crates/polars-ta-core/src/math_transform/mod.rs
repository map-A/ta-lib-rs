//! Math Transform indicators — element-wise application of standard math functions.
//! All functions have lookback=0 (output length = input length).
//! NaN inputs propagate NaN outputs automatically via IEEE 754 arithmetic.

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
    data.iter().map(|&x| x.cosh()).collect()
}

pub fn exp(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| x.exp()).collect()
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
    data.iter().map(|&x| x.sinh()).collect()
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
