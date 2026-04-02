//! Average Directional Movement Index (ADX)
//!
//! Measures trend strength, regardless of direction — numerically identical to
//! ta-lib's `TA_ADX`.
//!
//! # Algorithm
//!
//! 1. **True Range**: `TR = max(H-L, |H-prevC|, |L-prevC|)`
//! 2. **Directional Movement**:
//!    ```text
//!    up   = H - prevH
//!    dn   = prevL - L
//!    +DM  = up  if up > 0 && up > dn  else 0
//!    -DM  = dn  if dn > 0 && dn > up  else 0
//!    ```
//! 3. **Wilder smoothing** over `period` (initial value = SUM of first `period`
//!    raw values; subsequent: `smooth = smooth - smooth/period + current`):
//!    `smoothed_TR`, `smoothed_+DM`, `smoothed_-DM`
//! 4. `+DI = 100 * smoothed_+DM / smoothed_TR`
//! 5. `-DI = 100 * smoothed_-DM / smoothed_TR`
//! 6. `DX  = 100 * |+DI - -DI| / (+DI + -DI)` (0 when denominator = 0)
//! 7. **ADX** = Wilder smooth DX over `period`
//!    (initial ADX = SMA of first `period` DX values)
//!
//! # Parameters
//!
//! - `high`   — high price series
//! - `low`    — low price series
//! - `close`  — closing price series (all must have the same length)
//! - `period` — smoothing period (≥ 1; typically 14)
//!
//! # Output
//!
//! - Length = `high.len() - (2 * period - 1)` (lookback = `2 * period - 1`)
//! - Returns an empty `Vec` when input is too short
//!
//! # Example
//!
//! ```rust
//! use polars_ta_core::trend::adx;
//!
//! let n = 50;
//! let high:  Vec<f64> = (0..n).map(|i| 100.0 + i as f64 * 0.5).collect();
//! let low:   Vec<f64> = high.iter().map(|&h| h - 1.0).collect();
//! let close: Vec<f64> = high.iter().map(|&h| h - 0.5).collect();
//! let result = adx(&high, &low, &close, 14);
//! // lookback = 2*14 - 1 = 27
//! assert_eq!(result.len(), 50 - 27);
//! ```

/// Average Directional Movement Index.
///
/// See [module documentation](self) for full details.
pub fn adx(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    let n = high.len();
    assert_eq!(n, low.len());
    assert_eq!(n, close.len());

    if period == 0 || n < 2 * period {
        return vec![];
    }

    let pf = period as f64;
    // 预计算 Wilder 平滑系数，避免热循环中的除法（除法延迟 ~14 周期 vs 乘法 ~3 周期）
    let k = 1.0 - 1.0 / pf; // = (period-1)/period
    let inv_pf = 1.0 / pf;

    // --- 阶段 1: 累加前 period-1 个原始值（不使用中间 Vec）---
    let mut s_tr = 0.0_f64;
    let mut s_pdm = 0.0_f64;
    let mut s_mdm = 0.0_f64;

    for i in 1..period {
        let h = high[i];
        let l = low[i];
        let pc = close[i - 1];
        let ph = high[i - 1];
        let pl = low[i - 1];
        s_tr += adx_tr(h, l, pc);
        let (apdm, amdm) = adx_dm(h, ph, l, pl);
        s_pdm += apdm;
        s_mdm += amdm;
    }

    // Wilder 初始化：用第 period 个原始值做第一次平滑
    {
        let h = high[period];
        let l = low[period];
        let pc = close[period - 1];
        let ph = high[period - 1];
        let pl = low[period - 1];
        let tr = adx_tr(h, l, pc);
        let (pdm, mdm) = adx_dm(h, ph, l, pl);
        s_tr  = s_tr  * k + tr;
        s_pdm = s_pdm * k + pdm;
        s_mdm = s_mdm * k + mdm;
    }

    // --- 阶段 2: 计算前 period 个 DX 值用于 ADX 初始化（仅缓存小数组）---
    // 不再分配 O(n) 的 dx Vec；只需缓存 period 个值用于 SMA 初始化
    let mut dx_init = Vec::with_capacity(period);

    // 第一个 DX（来自初始化块）
    {
        let dm_sum = s_pdm + s_mdm;
        let dx0 = if dm_sum == 0.0 { 0.0 } else { 100.0 * (s_pdm - s_mdm).abs() / dm_sum };
        dx_init.push(dx0);
    }

    // 计算 DX[1..period-1] 并继续 Wilder 平滑
    for i in (period + 1)..(2 * period) {
        let h = high[i];
        let l = low[i];
        let pc = close[i - 1];
        let ph = high[i - 1];
        let pl = low[i - 1];
        let tr = adx_tr(h, l, pc);
        let (pdm, mdm) = adx_dm(h, ph, l, pl);
        s_tr  = s_tr  * k + tr;
        s_pdm = s_pdm * k + pdm;
        s_mdm = s_mdm * k + mdm;
        let dm_sum = s_pdm + s_mdm;
        let dx = if dm_sum == 0.0 { 0.0 } else { 100.0 * (s_pdm - s_mdm).abs() / dm_sum };
        dx_init.push(dx);
    }

    // ADX 初始值 = 前 period 个 DX 的均值
    let adx_len = n - 2 * period + 1;
    let mut out = Vec::with_capacity(adx_len);
    let mut prev_adx: f64 = dx_init.iter().sum::<f64>() * inv_pf;
    out.push(prev_adx);

    // --- 阶段 3: 流式计算剩余 DX 并直接更新 ADX（单遍，无大 Vec）---
    for i in (2 * period)..n {
        let h = high[i];
        let l = low[i];
        let pc = close[i - 1];
        let ph = high[i - 1];
        let pl = low[i - 1];
        let tr = adx_tr(h, l, pc);
        let (pdm, mdm) = adx_dm(h, ph, l, pl);
        s_tr  = s_tr  * k + tr;
        s_pdm = s_pdm * k + pdm;
        s_mdm = s_mdm * k + mdm;
        let dm_sum = s_pdm + s_mdm;
        let dx = if dm_sum == 0.0 { 0.0 } else { 100.0 * (s_pdm - s_mdm).abs() / dm_sum };
        prev_adx = prev_adx * k + dx * inv_pf;
        out.push(prev_adx);
    }

    out
}

#[inline]
fn adx_tr(h: f64, l: f64, pc: f64) -> f64 {
    if h.is_nan() || l.is_nan() || pc.is_nan() {
        return f64::NAN;
    }
    let hl = h - l;
    let hc = (h - pc).abs();
    let lc = (l - pc).abs();
    hl.max(hc).max(lc)
}

#[inline]
fn adx_dm(h: f64, ph: f64, l: f64, pl: f64) -> (f64, f64) {
    let up = h - ph;
    let dn = pl - l;
    if up.is_nan() || dn.is_nan() {
        return (f64::NAN, f64::NAN);
    }
    let pdm = if up > dn && up > 0.0 { up } else { 0.0 };
    let mdm = if dn > up && dn > 0.0 { dn } else { 0.0 };
    (pdm, mdm)
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
    fn adx_output_length() {
        let period = 14;
        let n = 100;
        let high: Vec<f64> = (0..n).map(|i| 100.0 + i as f64 * 0.5).collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 1.0).collect();
        let close: Vec<f64> = high.iter().map(|&h| h - 0.5).collect();
        let result = adx(&high, &low, &close, period);
        let expected = n - (2 * period - 1);
        assert_eq!(result.len(), expected);
    }

    #[test]
    fn adx_boundary_exact() {
        let period = 5;
        let n = 2 * period; // 最小输入使 lookback 恰好满足
        let high: Vec<f64> = (0..n).map(|i| 10.0 + i as f64).collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 1.0).collect();
        let close: Vec<f64> = high.iter().map(|&h| h - 0.5).collect();
        let result = adx(&high, &low, &close, period);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn adx_too_short() {
        let period = 5;
        let n = 2 * period - 1; // 比 lookback 少一个
        let high = vec![10.0f64; n];
        let low = vec![9.0f64; n];
        let close = vec![9.5f64; n];
        assert!(adx(&high, &low, &close, period).is_empty());
    }

    #[test]
    fn adx_period_zero() {
        let high = vec![10.0f64; 50];
        let low = vec![9.0f64; 50];
        let close = vec![9.5f64; 50];
        assert!(adx(&high, &low, &close, 0).is_empty());
    }

    #[test]
    fn adx_range_0_to_100() {
        // ADX 值应在 [0, 100] 范围内
        let n = 200;
        let high: Vec<f64> = (0..n)
            .map(|i| 100.0 + (i as f64 * 0.1).sin() * 5.0 + i as f64 * 0.05)
            .collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 2.0).collect();
        let close: Vec<f64> = high.iter().map(|&h| h - 1.0).collect();
        let result = adx(&high, &low, &close, 14);
        for &v in &result {
            assert!(
                v >= 0.0 && v <= 100.0,
                "ADX out of range: {v}"
            );
        }
    }

    #[test]
    fn adx_strong_trend() {
        // 持续上涨趋势：ADX 应相对较高
        let n = 100;
        let high: Vec<f64> = (0..n).map(|i| 100.0 + i as f64 * 2.0).collect();
        let low: Vec<f64> = (0..n).map(|i| 99.0 + i as f64 * 2.0).collect();
        let close: Vec<f64> = (0..n).map(|i| 100.0 + i as f64 * 2.0 - 0.5).collect();
        let result = adx(&high, &low, &close, 14);
        // 在稳定趋势中 ADX 应逐步升高并趋近 100
        let last = *result.last().unwrap();
        assert!(last > 50.0, "Expected ADX > 50 in strong uptrend, got {last}");
    }

    #[test]
    fn adx_lookback() {
        let period = 14;
        let n = 150;
        let high = vec![100.0f64; n];
        let low = vec![99.0f64; n];
        let close = vec![99.5f64; n];
        let result = adx(&high, &low, &close, period);
        assert_close(result.len() as f64, (n - (2 * period - 1)) as f64, 0.5);
    }
}
