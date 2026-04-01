//! Shared Wilder-smoothed DM/TR computation used by MINUS_DM, PLUS_DM,
//! MINUS_DI, PLUS_DI, and DX.

/// Compute Wilder-smoothed +DM and -DM using only high/low (no close).
/// Used by PLUS_DM and MINUS_DM.
///
/// - Lookback = `period - 1`
/// - Output length = `n - (period - 1)`
///
/// Initial value = sum of `period - 1` raw DM values (no Wilder step yet);
/// subsequent values apply Wilder smoothing.
pub(crate) fn compute_dm_smoothed_hl(
    high: &[f64],
    low: &[f64],
    period: usize,
) -> Option<(Vec<f64>, Vec<f64>)> {
    let n = high.len();
    if n < period {
        return None;
    }

    let pf = period as f64;
    let k = 1.0 - 1.0 / pf;

    let mut s_plus = 0.0_f64;
    let mut s_minus = 0.0_f64;

    unsafe {
        // SAFETY: 索引范围 1..period，period ≤ n，所以 i 和 i-1 均在 [0, n-1] 内
        let h = high.as_ptr();
        let l = low.as_ptr();

        for i in 1..period {
            let up_move = *h.add(i) - *h.add(i - 1);
            let dn_move = *l.add(i - 1) - *l.add(i);
            s_plus  += if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
            s_minus += if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
        }

        let out_len = n - (period - 1);
        let mut out_plus  = vec![0.0f64; out_len];
        let mut out_minus = vec![0.0f64; out_len];
        let op = out_plus.as_mut_ptr();
        let om = out_minus.as_mut_ptr();

        *op = s_plus;
        *om = s_minus;

        // SAFETY: j 遍历 1..out_len，i = period + j - 1 < n；索引 i 和 i-1 在 [0, n-1] 内
        for j in 1..out_len {
            let i = period - 1 + j;
            let up_move = *h.add(i) - *h.add(i - 1);
            let dn_move = *l.add(i - 1) - *l.add(i);
            s_plus  = s_plus  * k + if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
            s_minus = s_minus * k + if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
            *op.add(j) = s_plus;
            *om.add(j) = s_minus;
        }

        Some((out_plus, out_minus))
    }
}

/// Compute Wilder-smoothed +DM, -DM, and TR using high, low, close.
/// Used by PLUS_DI, MINUS_DI, and DX.
///
/// - Lookback = `period`
/// - Output length = `n - period`
///
/// Mirrors the ADX smoothing: accumulates `period - 1` raw values then applies
/// one Wilder step at index `period` before emitting the first output.
pub(crate) fn compute_dm_tr_smoothed(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let n = high.len();
    if n < period + 1 {
        return None;
    }

    let pf = period as f64;
    let k = 1.0 - 1.0 / pf;

    let mut s_plus  = 0.0_f64;
    let mut s_minus = 0.0_f64;
    let mut s_tr    = 0.0_f64;

    unsafe {
        // SAFETY: 所有索引均在 [0, n-1] 内：
        // - 初始化循环 1..period：i 和 i-1 在 [0, period-1] ⊂ [0, n-1]
        // - Wilder 步骤 period：索引 period 和 period-1，由 n ≥ period+1 保证
        // - 主循环 1..out_len：i = period+j < n（因 j < out_len = n-period）
        let h = high.as_ptr();
        let l = low.as_ptr();
        let c = close.as_ptr();

        for i in 1..period {
            let up_move = *h.add(i) - *h.add(i - 1);
            let dn_move = *l.add(i - 1) - *l.add(i);
            s_plus  += if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
            s_minus += if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
            let hl = *h.add(i) - *l.add(i);
            let hc = (*h.add(i) - *c.add(i - 1)).abs();
            let lc = (*l.add(i) - *c.add(i - 1)).abs();
            s_tr += hl.max(hc).max(lc);
        }

        // 第一个 Wilder 步骤（索引 period）
        {
            let up_move = *h.add(period) - *h.add(period - 1);
            let dn_move = *l.add(period - 1) - *l.add(period);
            s_plus  = s_plus  * k + if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
            s_minus = s_minus * k + if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
            let hl = *h.add(period) - *l.add(period);
            let hc = (*h.add(period) - *c.add(period - 1)).abs();
            let lc = (*l.add(period) - *c.add(period - 1)).abs();
            s_tr = s_tr * k + hl.max(hc).max(lc);
        }

        let out_len = n - period;
        let mut out_plus  = vec![0.0f64; out_len];
        let mut out_minus = vec![0.0f64; out_len];
        let mut out_tr    = vec![0.0f64; out_len];
        let op = out_plus.as_mut_ptr();
        let om = out_minus.as_mut_ptr();
        let ot = out_tr.as_mut_ptr();

        *op = s_plus;
        *om = s_minus;
        *ot = s_tr;

        for j in 1..out_len {
            let i = period + j;
            let up_move = *h.add(i) - *h.add(i - 1);
            let dn_move = *l.add(i - 1) - *l.add(i);
            s_plus  = s_plus  * k + if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
            s_minus = s_minus * k + if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
            let hl = *h.add(i) - *l.add(i);
            let hc = (*h.add(i) - *c.add(i - 1)).abs();
            let lc = (*l.add(i) - *c.add(i - 1)).abs();
            s_tr = s_tr * k + hl.max(hc).max(lc);
            *op.add(j) = s_plus;
            *om.add(j) = s_minus;
            *ot.add(j) = s_tr;
        }

        Some((out_plus, out_minus, out_tr))
    }
}
