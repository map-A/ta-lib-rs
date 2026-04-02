//! Moving Average with Variable Period (MAVP)
//!
//! 完全匹配 ta-lib 的 `TA_MAVP`（matype=0，即 SMA）。
//!
//! # Algorithm
//!
//! 对每个输出 bar i（对应输入 index `lookback + i`），
//! 从 `periods[lookback + i]` 取整并裁剪到 [min_period, max_period]，
//! 然后计算最近 period 个 close 的 SMA：
//!
//! ```text
//! lookback = max_period - 1
//!
//! for i in 0..out_len:
//!   idx    = lookback + i
//!   p      = clamp(round(periods[idx]), min_period, max_period)
//!   output[i] = mean(close[idx-p+1 ..= idx])
//! ```
//!
//! # Parameters
//!
//! - `data`       — 输入价格序列
//! - `periods`    — 每 bar 对应的 MA 周期（浮点，与 data 等长）
//! - `min_period` — 最小周期（默认 2）
//! - `max_period` — 最大周期（默认 30）
//!
//! # Output
//!
//! 长度 = `data.len() - (max_period - 1)`。输入太短时返回空 Vec。

/// Moving Average with Variable Period (SMA variant).
///
/// 详见 [模块文档](self)。
pub fn mavp(
    data: &[f64],
    periods: &[f64],
    min_period: usize,
    max_period: usize,
) -> Vec<f64> {
    if max_period == 0 || min_period == 0 || min_period > max_period {
        return vec![];
    }
    let n = data.len();
    if n != periods.len() {
        return vec![];
    }

    let lookback = max_period - 1;
    if n <= lookback {
        return vec![];
    }

    let out_len = n - lookback;
    let mut out = Vec::with_capacity(out_len);

    for i in 0..out_len {
        let idx = lookback + i;
        let raw_p = periods[idx];
        if raw_p.is_nan() {
            out.push(f64::NAN);
            continue;
        }
        // ta-lib rounds to nearest integer (ties go away from zero)
        let p = (raw_p.round() as usize).clamp(min_period, max_period);
        let start = idx + 1 - p;
        let sum: f64 = data[start..=idx].iter().sum();
        out.push(sum / p as f64);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mavp_output_length() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let periods = vec![14.0f64; 100];
        let out = mavp(&data, &periods, 2, 30);
        // lookback = 29, output_len = 71
        assert_eq!(out.len(), 71);
    }

    #[test]
    fn mavp_too_short() {
        let data = vec![1.0f64; 29]; // exactly lookback
        let periods = vec![14.0f64; 29];
        let out = mavp(&data, &periods, 2, 30);
        assert!(out.is_empty());
    }

    #[test]
    fn mavp_boundary_exact() {
        // lookback=29, so n=30 → exactly 1 output
        let data = vec![1.0f64; 30];
        let periods = vec![14.0f64; 30];
        let out = mavp(&data, &periods, 2, 30);
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn mavp_constant_series() {
        // 常数序列：SMA 始终等于常数
        let data = vec![5.0f64; 100];
        let periods = vec![14.0f64; 100];
        let out = mavp(&data, &periods, 2, 30);
        assert_eq!(out.len(), 71);
        for &v in &out {
            assert!((v - 5.0).abs() < 1e-12, "v={v}");
        }
    }

    #[test]
    fn mavp_variable_periods() {
        // 验证变化的 period
        let data: Vec<f64> = (0..50).map(|x| x as f64).collect();
        let mut periods = vec![5.0f64; 50];
        periods[49] = 3.0; // 最后一个 bar 用 period=3
        let out = mavp(&data, &periods, 2, 30);
        // lookback=29, out_len=21
        assert_eq!(out.len(), 21);
        // 最后一个输出：mean(47,48,49) = 48.0
        let last = *out.last().unwrap();
        assert!((last - 48.0).abs() < 1e-12, "last={last}");
    }

    #[test]
    fn mavp_mismatched_lengths() {
        let data = vec![1.0f64; 50];
        let periods = vec![5.0f64; 40];
        assert!(mavp(&data, &periods, 2, 30).is_empty());
    }
}
