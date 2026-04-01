//! STDDEV — 标准差（总体方差，ta-lib 兼容）
//!
//! var_pop = (sum_sq - sum*sum/n) / n
//! stddev  = sqrt(var_pop) * nbdev

pub fn stddev(data: &[f64], period: usize, nbdev: f64) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }

    let pf = period as f64;
    let inv_pf = 1.0 / pf;
    let inv_pf2 = inv_pf * inv_pf;

    let out_len = n - (period - 1);
    let mut out = vec![0.0_f64; out_len];

    let mut sum = 0.0_f64;
    let mut sum_sq = 0.0_f64;

    for &y in &data[..period] {
        sum += y;
        sum_sq += y * y;
    }

    out[0] = (sum_sq * inv_pf - sum * sum * inv_pf2).max(0.0).sqrt() * nbdev;

    for i in 1..out_len {
        let yo = data[i - 1];
        let yn = data[i + period - 1];
        sum += yn - yo;
        sum_sq += yn * yn - yo * yo;
        let v = (sum_sq * inv_pf - sum * sum * inv_pf2).max(0.0).sqrt() * nbdev;
        out[i] = v;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stddev_known_values() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = stddev(&data, 5, 1.0);
        assert_eq!(result.len(), 1);
        assert!((result[0] - 2.0_f64.sqrt()).abs() < 1e-10, "got {}", result[0]);
    }

    #[test]
    fn stddev_flat() {
        let data = vec![5.0f64; 10];
        let result = stddev(&data, 5, 1.0);
        for v in &result {
            assert!(v.abs() < 1e-10, "got {}", v);
        }
    }

    #[test]
    fn stddev_boundary_short() {
        assert!(stddev(&[1.0, 2.0], 3, 1.0).is_empty());
    }
}
