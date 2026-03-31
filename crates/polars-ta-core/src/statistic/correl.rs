//! CORREL — Pearson 相关系数（滚动窗口）
//!
//! r = (n*Σxy - Σx*Σy) / sqrt((n*Σx² - (Σx)²) * (n*Σy² - (Σy)²))
//!
//! lookback = period - 1

pub fn correl(real0: &[f64], real1: &[f64], period: usize) -> Vec<f64> {
    let n = real0.len().min(real1.len());
    if period == 0 || n < period {
        return vec![];
    }

    let p = period as f64;
    let out_len = n - (period - 1);
    let mut out = Vec::with_capacity(out_len);

    let mut sum_x = 0.0_f64;
    let mut sum_y = 0.0_f64;
    let mut sum_xy = 0.0_f64;
    let mut sum_x2 = 0.0_f64;
    let mut sum_y2 = 0.0_f64;

    for i in 0..period {
        let x = real0[i];
        let y = real1[i];
        sum_x += x;
        sum_y += y;
        sum_xy += x * y;
        sum_x2 += x * x;
        sum_y2 += y * y;
    }

    let calc = |sx: f64, sy: f64, sxy: f64, sx2: f64, sy2: f64| -> f64 {
        let num = p * sxy - sx * sy;
        let den = ((p * sx2 - sx * sx) * (p * sy2 - sy * sy)).sqrt();
        if den == 0.0 { 0.0 } else { num / den }
    };

    out.push(calc(sum_x, sum_y, sum_xy, sum_x2, sum_y2));

    for i in period..n {
        let xo = real0[i - period];
        let yo = real1[i - period];
        let xn = real0[i];
        let yn = real1[i];
        sum_x += xn - xo;
        sum_y += yn - yo;
        sum_xy += xn * yn - xo * yo;
        sum_x2 += xn * xn - xo * xo;
        sum_y2 += yn * yn - yo * yo;
        out.push(calc(sum_x, sum_y, sum_xy, sum_x2, sum_y2));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correl_identical() {
        let data: Vec<f64> = (1..=10).map(|x| x as f64).collect();
        let result = correl(&data, &data, 5);
        assert_eq!(result.len(), 6);
        for v in &result {
            assert!((v - 1.0).abs() < 1e-10, "identical series r=1, got {}", v);
        }
    }

    #[test]
    fn correl_boundary_short() {
        let data = vec![1.0, 2.0];
        assert!(correl(&data, &data, 3).is_empty());
    }

    #[test]
    fn correl_period_zero() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(correl(&data, &data, 0).is_empty());
    }
}
