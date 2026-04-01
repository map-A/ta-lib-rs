//! Triangular Moving Average (TRIMA)
//!
//! A double-smoothed SMA — numerically identical to ta-lib's `TA_TRIMA`.
//!
//! # Algorithm
//!
//! ```text
//! For odd period:  first_period = second_period = (period + 1) / 2
//! For even period: first_period = period / 2 + 1,  second_period = period / 2
//! trima = SMA(SMA(data, first_period), second_period)
//! ```
//!
//! # Parameters
//!
//! - `data`   — input price series (typically `close`)
//! - `period` — window length (≥ 1)
//!
//! # Output
//!
//! - Length = `data.len() - (period - 1)`
//! - Returns an empty `Vec` when `data.len() < period`

/// Triangular Moving Average.
///
/// See [module documentation](self) for full details.
pub fn trima(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }

    let (p1, p2) = if period % 2 == 1 {
        let p = (period + 1) / 2;
        (p, p)
    } else {
        (period / 2 + 1, period / 2)
    };
    // 注: p1 + p2 = period + 1 (对奇/偶 period 均成立)

    let out_len = n - period + 1;
    let mut out = vec![0.0f64; out_len];
    let inv = 1.0 / (p1 as f64 * p2 as f64);

    unsafe {
        // SAFETY: 所有索引均在 [0, n-1] 内（已在循环范围分析中验证）：
        // - leave 初始化：k in 0..p1，p1 ≤ period ≤ n
        // - enter 滑动：j + p1 - 1 ≤ p2-1+p1-1 = period-2 < n
        // - 主循环 t in 1..out_len：
        //   t + p2 - 2 ≤ out_len-1+p2-2 = n-p1-2 < n
        //   t + p2 + p1 - 2 ≤ out_len-1+period-2 = n-2 < n
        //   t - 1 ≤ out_len - 2 < n
        //   t + p1 - 1 ≤ out_len-1+p1-1 ≤ n-p2-1 < n
        let data_ptr = data.as_ptr();
        let out_ptr = out.as_mut_ptr();

        // 初始化 leave = inner_sum[0]（data[0..p1] 之和）
        let mut leave: f64 = 0.0;
        for k in 0..p1 {
            leave += *data_ptr.add(k);
        }
        let mut enter = leave;
        let mut outer_sum = leave;

        // 将 enter 滑动到 inner_sum[p2-1]，累积外层和
        for j in 1..p2 {
            enter = enter - *data_ptr.add(j - 1) + *data_ptr.add(j + p1 - 1);
            outer_sum += enter;
        }
        *out_ptr = outer_sum * inv;

        for t in 1..out_len {
            enter = enter - *data_ptr.add(t + p2 - 2) + *data_ptr.add(t + p2 + p1 - 2);
            outer_sum = outer_sum - leave + enter;
            *out_ptr.add(t) = outer_sum * inv;
            leave = leave - *data_ptr.add(t - 1) + *data_ptr.add(t + p1 - 1);
        }
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
    fn trima_output_length_odd_period() {
        let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        let result = trima(&data, 5);
        // lookback = period - 1 = 4, output length = 20 - 4 = 16
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn trima_output_length_even_period() {
        let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        let result = trima(&data, 4);
        // lookback = 3, output length = 17
        assert_eq!(result.len(), 17);
    }

    #[test]
    fn trima_constant_series() {
        let data = vec![5.0f64; 50];
        let result = trima(&data, 7);
        for &v in &result {
            assert_close(v, 5.0, 1e-10);
        }
    }

    #[test]
    fn trima_period1() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = trima(&data, 1);
        assert_eq!(result.len(), 5);
        assert_close(result[0], 1.0, 1e-10);
        assert_close(result[4], 5.0, 1e-10);
    }

    #[test]
    fn trima_boundary_exact() {
        let data: Vec<f64> = (1..=5).map(|x| x as f64).collect();
        let result = trima(&data, 5);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn trima_boundary_short() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(trima(&data, 5).is_empty());
    }

    #[test]
    fn trima_period_zero() {
        assert!(trima(&[1.0, 2.0, 3.0], 0).is_empty());
    }

    #[test]
    fn trima_empty_input() {
        assert!(trima(&[], 5).is_empty());
    }
}
