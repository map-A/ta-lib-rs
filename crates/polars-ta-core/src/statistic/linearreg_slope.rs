//! LINEARREG_SLOPE — 线性回归斜率
//!
//! slope = (n*Σxy - Σx*Σy) / (n*Σx² - (Σx)²)

pub fn linearreg_slope(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }

    let pf = period as f64;
    let sum_x = pf * (pf - 1.0) / 2.0;
    let sum_x2 = pf * (pf - 1.0) * (2.0 * pf - 1.0) / 6.0;
    let divisor = pf * sum_x2 - sum_x * sum_x;

    let mut sum_y = 0.0_f64;
    let mut sum_xy = 0.0_f64;

    for (x, &y) in data[..period].iter().enumerate() {
        sum_y += y;
        sum_xy += x as f64 * y;
    }

    let calc_slope = |sy: f64, sxy: f64| -> f64 {
        (pf * sxy - sum_x * sy) / divisor
    };

    let out_len = n - (period - 1);
    let mut out = Vec::with_capacity(out_len);
    out.push(calc_slope(sum_y, sum_xy));

    for i in period..n {
        let y_old = data[i - period];
        sum_y -= y_old;
        sum_xy -= sum_y;
        sum_y += data[i];
        sum_xy += (pf - 1.0) * data[i];
        out.push(calc_slope(sum_y, sum_xy));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slope_unit() {
        let data: Vec<f64> = (0..5).map(|x| x as f64).collect();
        let result = linearreg_slope(&data, 5);
        assert_eq!(result.len(), 1);
        assert!((result[0] - 1.0).abs() < 1e-10, "got {}", result[0]);
    }

    #[test]
    fn slope_flat() {
        let data = vec![5.0f64; 10];
        let result = linearreg_slope(&data, 5);
        for v in &result {
            assert!(v.abs() < 1e-10, "got {}", v);
        }
    }
}
