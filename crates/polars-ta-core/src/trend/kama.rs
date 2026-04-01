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

    // 初始化波动性累加和
    let mut vol_sum = 0.0f64;
    for k in 1..=period {
        vol_sum += (data[k] - data[k - 1]).abs();
    }

    let mut prev_kama = data[period - 1];

    let main_len = out_len.saturating_sub(1);
    for i in 0..main_len {
        let idx = period + i;
        let cur = data[idx];
        let old = data[i];

        let direction = (cur - old).abs();
        let inv_vol = if vol_sum > 0.0 { 1.0 / vol_sum } else { 0.0 };
        let er = direction * inv_vol;
        let sc = er * sc_diff + slow_sc;
        let cur_kama = prev_kama + sc * sc * (cur - prev_kama);
        out[i] = cur_kama;
        prev_kama = cur_kama;

        let old_diff = (data[i + 1] - old).abs();
        let new_diff = (data[idx + 1] - cur).abs();
        vol_sum = (vol_sum - old_diff + new_diff).max(0.0);
    }
    if out_len > 0 {
        let i = out_len - 1;
        let cur = data[period + i];
        let old = data[i];
        let direction = (cur - old).abs();
        let inv_vol = if vol_sum > 0.0 { 1.0 / vol_sum } else { 0.0 };
        let er = direction * inv_vol;
        let sc = er * sc_diff + slow_sc;
        out[i] = prev_kama + sc * sc * (cur - prev_kama);
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
