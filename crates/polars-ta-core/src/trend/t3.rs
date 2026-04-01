//! T3 Moving Average (Triple Exponential with Volume Factor)
//!
//! Numerically identical to ta-lib's `TA_T3`.
//!
//! # Algorithm
//!
//! ```text
//! k  = 2 / (period + 1)
//! vf = volume factor (default 0.7)
//!
//! Compute 6 successive EMAs each seeded with SMA:
//!   e1 = EMA(data,  period)
//!   e2 = EMA(e1,    period)
//!   e3 = EMA(e2,    period)
//!   e4 = EMA(e3,    period)
//!   e5 = EMA(e4,    period)
//!   e6 = EMA(e5,    period)
//!
//! Coefficients:
//!   c1 = -(vf^3)
//!   c2 =  3*vf^2 + 3*vf^3
//!   c3 = -6*vf^2 - 3*vf - 3*vf^3
//!   c4 =  1 + 3*vf + vf^3 + 3*vf^2
//!
//! Alignment (output index i corresponds to data position 6*(period-1)+i):
//!   T3[i] = c1*e6[i] + c2*e5[i+(p-1)] + c3*e4[i+2*(p-1)] + c4*e3[i+3*(p-1)]
//! ```
//!
//! # Parameters
//!
//! - `data`   — input price series
//! - `period` — smoothing window length (≥ 1)
//! - `vf`     — volume factor, typically 0.7
//!
//! # Output
//!
//! - Length = `data.len() - 6 * (period - 1)`
//! - Returns an empty `Vec` when `data.len() <= 6 * (period - 1)`


/// T3 Moving Average.
///
/// See [module documentation](self) for full details.
pub fn t3(data: &[f64], period: usize, vf: f64) -> Vec<f64> {
    let n = data.len();
    if period == 0 {
        return vec![];
    }

    let p1 = period - 1;
    let lookback = 6 * p1;
    if n <= lookback {
        return vec![];
    }

    let vf2 = vf * vf;
    let vf3 = vf2 * vf;
    let c1 = -vf3;
    let c2 = 3.0 * vf2 + 3.0 * vf3;
    let c3 = -6.0 * vf2 - 3.0 * vf - 3.0 * vf3;
    let c4 = 1.0 + 3.0 * vf + vf3 + 3.0 * vf2;

    if period == 1 {
        return data.to_vec();
    }

    let k = 2.0 / (period as f64 + 1.0);
    let km1 = 1.0 - k;
    let inv_p = 1.0 / period as f64;

    let out_len = n - lookback;
    let mut out = vec![0.0f64; out_len];

    // 预热阶段：使用带分支的通用逻辑初始化 6 个 EMA 状态
    let mut e = [0.0f64; 6];
    let mut sums = [0.0f64; 6];
    let mut cnt = [0usize; 6];

    for (i, &x) in data[..=lookback].iter().enumerate() {
        let mut val = x;
        for s in 0..6usize {
            if i < s * p1 {
                break;
            }
            cnt[s] += 1;
            if cnt[s] <= period {
                sums[s] += val;
                if cnt[s] < period {
                    break;
                }
                e[s] = sums[s] * inv_p;
                val = e[s];
            } else {
                e[s] = val * k + e[s] * km1;
                val = e[s];
            }
        }
    }

    out[0] = c1 * e[5] + c2 * e[4] + c3 * e[3] + c4 * e[2];

    for (i, &x) in data[lookback + 1..].iter().enumerate() {
        e[0] = x    * k + e[0] * km1;
        e[1] = e[0] * k + e[1] * km1;
        e[2] = e[1] * k + e[2] * km1;
        e[3] = e[2] * k + e[3] * km1;
        e[4] = e[3] * k + e[4] * km1;
        e[5] = e[4] * k + e[5] * km1;
        out[i + 1] = c1 * e[5] + c2 * e[4] + c3 * e[3] + c4 * e[2];
    }

    out
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
    fn t3_output_length() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let result = t3(&data, 5, 0.7);
        // lookback = 6 * 4 = 24, output = 100 - 24 = 76
        assert_eq!(result.len(), 76);
    }

    #[test]
    fn t3_constant_series() {
        let data = vec![5.0f64; 100];
        let result = t3(&data, 5, 0.7);
        for &v in &result {
            assert_close(v, 5.0, 1e-10);
        }
    }

    #[test]
    fn t3_period1() {
        // lookback = 0, 系数之和 = 1, T3 = data
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = t3(&data, 1, 0.7);
        assert_eq!(result.len(), 5);
        assert_close(result[0], 1.0, 1e-10);
        assert_close(result[4], 5.0, 1e-10);
    }

    #[test]
    fn t3_boundary_short() {
        let data = vec![1.0f64; 24];
        // lookback = 24, n = 24 <= lookback → empty
        assert!(t3(&data, 5, 0.7).is_empty());
    }

    #[test]
    fn t3_boundary_exact() {
        let data = vec![5.0f64; 25];
        // lookback = 24, n = 25 > 24, output = 1
        let result = t3(&data, 5, 0.7);
        assert_eq!(result.len(), 1);
        assert_close(result[0], 5.0, 1e-10);
    }

    #[test]
    fn t3_period_zero() {
        assert!(t3(&[1.0, 2.0], 0, 0.7).is_empty());
    }

    #[test]
    fn t3_empty_input() {
        assert!(t3(&[], 5, 0.7).is_empty());
    }
}
