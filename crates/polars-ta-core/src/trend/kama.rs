//! Kaufman Adaptive Moving Average (KAMA)
//!
//! Numerically identical to ta-lib's `TA_KAMA`.
//!
//! # Algorithm
//!
//! ```text
//! fast_sc = 2 / (2 + 1)  = 2/3
//! slow_sc = 2 / (30 + 1) = 2/31
//!
//! kama[period] = close[period]   (SMA seed)
//! for i in period+1..n:
//!     direction  = |close[i] - close[i - period]|
//!     volatility = Σ |close[k] - close[k-1]|  for k in i-period+1..=i
//!     er  = direction / volatility   (0.0 if volatility == 0)
//!     sc  = (er * (fast_sc - slow_sc) + slow_sc)²
//!     kama[i] = kama[i-1] + sc * (close[i] - kama[i-1])
//! ```
//!
//! # Parameters
//!
//! - `data`   — input price series (typically `close`)
//! - `period` — efficiency-ratio window (≥ 1)
//!
//! # Output
//!
//! - Length = `data.len() - period`
//! - First value = first KAMA update computed using `close[period]` as the new price,
//!   with `close[period-1]` as the internal seed (not output, matching ta-lib behavior)
//! - Returns an empty `Vec` when `data.len() <= period`

/// Kaufman Adaptive Moving Average.
///
/// See [module documentation](self) for full details.
pub fn kama(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n <= period {
        return vec![];
    }

    let fast_sc = 2.0 / 3.0;
    let slow_sc = 2.0 / 31.0;
    let sc_diff = fast_sc - slow_sc;

    let out_len = n - period;
    let mut out = vec![0.0f64; out_len];

    // 初始化波动性累加和（索引 1..=period）
    let mut vol_sum: f64 = 0.0;
    unsafe {
        // SAFETY: k 遍历 1..=period，period < n，所以 k 和 k-1 均在 [0, n-1] 内
        let ptr = data.as_ptr();
        for k in 1..=period {
            vol_sum += (*ptr.add(k) - *ptr.add(k - 1)).abs();
        }
    }

    // ta-lib 以 close[period-1] 为内部种子（不输出），第一个输出是 close[period] 的第一次 KAMA 更新
    let mut prev_kama = data[period - 1];

    unsafe {
        // SAFETY: 循环中所有索引均在 [0, n-1] 内：
        // - idx = period + i，i < out_len = n - period，所以 idx < n
        // - i 作为"old"索引，i < out_len ≤ n - 1
        // - idx + 1 仅在 i < out_len - 1 时访问，此时 idx + 1 = period + i + 1 ≤ n - 1
        let data_ptr = data.as_ptr();
        let out_ptr = out.as_mut_ptr();

        for i in 0..out_len {
            let idx = period + i;
            let cur = *data_ptr.add(idx);
            let old = *data_ptr.add(i);

            let direction = (cur - old).abs();
            let er = if vol_sum > 0.0 { direction / vol_sum } else { 0.0 };
            let sc = (er * sc_diff + slow_sc).powi(2);

            let cur_kama = prev_kama + sc * (cur - prev_kama);
            *out_ptr.add(i) = cur_kama;
            prev_kama = cur_kama;

            // 滑动窗口：移出最旧差值，移入最新差值
            if i < out_len - 1 {
                let next = *data_ptr.add(idx + 1);
                let old_diff = (*data_ptr.add(i + 1) - old).abs();
                let new_diff = (next - cur).abs();
                vol_sum = (vol_sum - old_diff + new_diff).max(0.0);
            }
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
    fn kama_output_length() {
        let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
        let result = kama(&data, 10);
        // lookback = 10, output = 50 - 10 = 40
        assert_eq!(result.len(), 40);
    }

    #[test]
    fn kama_constant_series() {
        // 常数序列：direction=0, er=0, sc=slow_sc^2, KAMA 收敛到常数
        let data = vec![100.0f64; 50];
        let result = kama(&data, 10);
        // 常数序列中第一个值为 close[period] = 100.0
        // 每步 sc * (close[i] - kama) = 0, 所以 kama 保持 100.0
        assert_eq!(result.len(), 40);
        for &v in &result {
            assert_close(v, 100.0, 1e-10);
        }
    }

    #[test]
    fn kama_seed_value() {
        // data = [0,2,4,...,40], period=10
        // seed = data[9]=18, first update uses data[10]=20
        // ER=1, sc=(2/3)^2=4/9, kama=18+4/9*(20-18)=18.888...
        let data: Vec<f64> = (0..=20).map(|x| x as f64 * 2.0).collect();
        let result = kama(&data, 10);
        assert!(!result.is_empty());
        let expected = 18.0 + (4.0 / 9.0) * (20.0 - 18.0);
        assert_close(result[0], expected, 1e-10);
    }

    #[test]
    fn kama_boundary_short() {
        let data = vec![1.0f64; 10];
        // n = 10 = period → n <= period → empty
        assert!(kama(&data, 10).is_empty());
    }

    #[test]
    fn kama_boundary_exact() {
        // n = period + 1: 只有一个输出
        let data = vec![5.0f64; 11];
        let result = kama(&data, 10);
        assert_eq!(result.len(), 1);
        assert_close(result[0], 5.0, 1e-10);
    }

    #[test]
    fn kama_period_zero() {
        assert!(kama(&[1.0, 2.0, 3.0], 0).is_empty());
    }

    #[test]
    fn kama_empty_input() {
        assert!(kama(&[], 10).is_empty());
    }
}
