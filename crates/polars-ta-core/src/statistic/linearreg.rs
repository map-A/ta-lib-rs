//! LINEARREG — 线性回归终点值
//!
//! LINEARREG = intercept + slope * (period - 1)
//!
//! O(n) 滑动窗口：维护 sum_y 和 sum_xy（x=[0..period-1]）
//!
//! 更新规则（移除旧值 y_old，添加新值 y_new）：
//!   sum_y  -= y_old
//!   sum_xy -= sum_y        // 所有剩余 x 下标减 1（y_old 在 x=0，贡献为 0）
//!   sum_y  += y_new
//!   sum_xy += (period-1) * y_new

pub fn linearreg(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }

    let pf = period as f64;
    let sum_x = pf * (pf - 1.0) / 2.0;
    let sum_x2 = pf * (pf - 1.0) * (2.0 * pf - 1.0) / 6.0;
    let divisor = pf * sum_x2 - sum_x * sum_x;

    let calc = |sy: f64, sxy: f64| -> f64 {
        let slope = (pf * sxy - sum_x * sy) / divisor;
        let intercept = (sy - slope * sum_x) / pf;
        intercept + slope * (pf - 1.0)
    };

    let mut nan_count: usize = data[..period].iter().filter(|&&x| x.is_nan()).count();
    let mut sum_y = 0.0_f64;
    let mut sum_xy = 0.0_f64;

    for (x, &y) in data[..period].iter().enumerate() {
        if !y.is_nan() {
            sum_y += y;
            sum_xy += x as f64 * y;
        }
    }

    let out_len = n - (period - 1);
    let mut out = Vec::with_capacity(out_len);
    if nan_count > 0 {
        out.push(f64::NAN);
    } else {
        out.push(calc(sum_y, sum_xy));
    }

    for i in period..n {
        let y_old = data[i - period];
        let y_new = data[i];
        let was_dirty = nan_count > 0;
        if y_old.is_nan() {
            nan_count -= 1;
        }
        if y_new.is_nan() {
            nan_count += 1;
        }

        if nan_count > 0 {
            out.push(f64::NAN);
        } else if was_dirty {
            // 刚从脏状态恢复：从头重新计算
            sum_y = 0.0;
            sum_xy = 0.0;
            for (x, &y) in data[i - period + 1..=i].iter().enumerate() {
                if !y.is_nan() {
                    sum_y += y;
                    sum_xy += x as f64 * y;
                }
            }
            out.push(calc(sum_y, sum_xy));
        } else {
            sum_y -= y_old;
            sum_xy -= sum_y;
            sum_y += y_new;
            sum_xy += (pf - 1.0) * y_new;
            out.push(calc(sum_y, sum_xy));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linearreg_linear_data() {
        let data: Vec<f64> = (0..5).map(|x| x as f64).collect();
        let result = linearreg(&data, 5);
        assert_eq!(result.len(), 1);
        assert!((result[0] - 4.0).abs() < 1e-10, "got {}", result[0]);
    }

    #[test]
    fn linearreg_flat_data() {
        let data = vec![3.0f64; 10];
        let result = linearreg(&data, 5);
        for v in &result {
            assert!((v - 3.0).abs() < 1e-10, "got {}", v);
        }
    }

    #[test]
    fn linearreg_boundary_short() {
        assert!(linearreg(&[1.0, 2.0], 3).is_empty());
    }
}
