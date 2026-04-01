//! VAR — 方差（总体方差，ta-lib 兼容）
//!
//! var_pop = (sum_sq - sum*sum/n) / n
//! VAR     = var_pop * nbdev * nbdev

pub fn var(data: &[f64], period: usize, nbdev: f64) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }

    let pf = period as f64;
    let nbdev2 = nbdev * nbdev;
    // 预计算倒数，消除热路径中的除法
    let inv_pf = 1.0 / pf;
    let inv_pf2 = inv_pf * inv_pf;
    let coeff1 = nbdev2 * inv_pf;      // nbdev^2 / period
    let coeff2 = nbdev2 * inv_pf2;     // nbdev^2 / period^2

    let out_len = n - (period - 1);
    let mut out = vec![0.0_f64; out_len];

    let mut sum = 0.0_f64;
    let mut sum_sq = 0.0_f64;

    for &y in &data[..period] {
        sum += y;
        sum_sq += y * y;
    }

    // v = sum_sq/period - (sum/period)^2 = sum_sq*inv_pf - sum^2*inv_pf^2
    let v = (sum_sq * coeff1 - sum * sum * coeff2).max(0.0);
    out[0] = v;

    for i in 1..out_len {
        let yo = data[i - 1];
        let yn = data[i + period - 1];
        sum += yn - yo;
        sum_sq += yn * yn - yo * yo;
        let v = (sum_sq * coeff1 - sum * sum * coeff2).max(0.0);
        out[i] = v;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_known_values() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = var(&data, 5, 1.0);
        assert_eq!(result.len(), 1);
        assert!((result[0] - 2.0).abs() < 1e-10, "got {}", result[0]);
    }

    #[test]
    fn var_flat() {
        let data = vec![5.0f64; 10];
        let result = var(&data, 5, 1.0);
        for v in &result {
            assert!(v.abs() < 1e-10, "got {}", v);
        }
    }

    #[test]
    fn var_boundary_short() {
        assert!(var(&[1.0, 2.0], 3, 1.0).is_empty());
    }
}
