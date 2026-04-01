//! On Balance Volume (OBV)
//!
//! Cumulative volume indicator that adds volume on up days and subtracts it
//! on down days. Numerically identical to ta-lib's `TA_OBV`.
//!
//! # Algorithm
//!
//! ```text
//! obv[0] = volume[0]
//! for i in 1..n:
//!   if close[i] > close[i-1]:  obv[i] = obv[i-1] + volume[i]
//!   elif close[i] < close[i-1]: obv[i] = obv[i-1] - volume[i]
//!   else:                        obv[i] = obv[i-1]
//! ```
//!
//! # Parameters
//!
//! - `close`  — closing prices
//! - `volume` — trading volume
//!
//! # Output
//!
//! - Length = `close.len()` (lookback = 0)
//! - Returns empty `Vec` when input slices are empty or have mismatched lengths
//!
//! # NaN Handling
//!
//! NaN in `close` or `volume` propagates via IEEE 754 arithmetic.
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::volume::obv;
//!
//! let close  = vec![10.0, 11.0, 10.5, 11.0];
//! let volume = vec![100.0, 200.0, 150.0, 300.0];
//! let result = obv(&close, &volume);
//! assert_eq!(result.len(), 4);
//! assert_eq!(result[0], 100.0);     // seed = volume[0]
//! assert_eq!(result[1], 300.0);      // up: +200
//! assert_eq!(result[2], 150.0);      // down: -150
//! assert_eq!(result[3], 450.0);      // up: +300
//! ```

/// On Balance Volume.
///
/// See [module documentation](self) for full details.
pub fn obv(close: &[f64], volume: &[f64]) -> Vec<f64> {
    let n = close.len();
    if n == 0 || volume.len() != n {
        return vec![];
    }

    let mut out = vec![0.0_f64; n];

    // SAFETY: 所有索引均在 [0, n) 内，n 已验证非零
    unsafe {
        let c = close.as_ptr();
        let v = volume.as_ptr();
        let o = out.as_mut_ptr();

        let mut acc = *v;
        *o = acc;

        // 位掩码法：将 bool 扩展为全 1 / 全 0 的 u64，直接应用到 volume 的 IEEE 754 位上
        // 完全无分支，无 int→float 转换，LLVM 可识别为 pand/pandn 模式
        let mut prev = *c;
        for i in 1..n {
            let curr = *c.add(i);
            let vol = *v.add(i);
            let up = (curr > prev) as u64;
            let dn = (curr < prev) as u64;
            let bits = vol.to_bits();
            let pos = f64::from_bits(bits & up.wrapping_neg());
            let neg = f64::from_bits(bits & dn.wrapping_neg());
            acc += pos - neg;
            *o.add(i) = acc;
            prev = curr;
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
            "actual={actual:.10}, expected={expected:.10}",
        );
    }

    #[test]
    fn obv_basic() {
        let close  = vec![10.0, 11.0, 10.5, 11.0];
        let volume = vec![100.0, 200.0, 150.0, 300.0];
        let result = obv(&close, &volume);
        assert_eq!(result.len(), 4);
        assert_close(result[0], 100.0, 1e-10);
        assert_close(result[1], 300.0, 1e-10);
        assert_close(result[2], 150.0, 1e-10);
        assert_close(result[3], 450.0, 1e-10);
    }

    #[test]
    fn obv_flat_close() {
        // 价格不变 → OBV 不变（始终等于 volume[0]）
        let close  = vec![10.0, 10.0, 10.0];
        let volume = vec![100.0, 200.0, 300.0];
        let result = obv(&close, &volume);
        assert_eq!(result.len(), 3);
        for v in &result {
            assert_close(*v, 100.0, 1e-10);
        }
    }

    #[test]
    fn obv_empty() {
        assert!(obv(&[], &[]).is_empty());
    }

    #[test]
    fn obv_length_mismatch() {
        let close  = vec![1.0, 2.0];
        let volume = vec![100.0];
        assert!(obv(&close, &volume).is_empty());
    }

    #[test]
    fn obv_single_element() {
        let close  = vec![10.0];
        let volume = vec![500.0];
        let result = obv(&close, &volume);
        assert_eq!(result.len(), 1);
        assert_close(result[0], 500.0, 1e-10);
    }

    #[test]
    fn obv_output_length_equals_input() {
        let n = 50;
        let close: Vec<f64>  = (0..n).map(|i| i as f64).collect();
        let volume: Vec<f64> = vec![1.0; n];
        let result = obv(&close, &volume);
        assert_eq!(result.len(), n);
    }

    #[test]
    fn obv_strictly_up() {
        let close  = vec![1.0, 2.0, 3.0, 4.0];
        let volume = vec![100.0, 100.0, 100.0, 100.0];
        let result = obv(&close, &volume);
        // 100, 200, 300, 400
        assert_close(result[0], 100.0, 1e-10);
        assert_close(result[3], 400.0, 1e-10);
    }

    #[test]
    fn obv_strictly_down() {
        let close  = vec![4.0, 3.0, 2.0, 1.0];
        let volume = vec![100.0, 100.0, 100.0, 100.0];
        let result = obv(&close, &volume);
        // 100, 0, -100, -200
        assert_close(result[3], -200.0, 1e-10);
    }
}
