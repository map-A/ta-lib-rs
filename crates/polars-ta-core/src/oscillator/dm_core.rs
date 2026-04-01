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

    for i in 1..period {
        let up_move = high[i] - high[i - 1];
        let dn_move = low[i - 1] - low[i];
        s_plus  += if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
        s_minus += if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
    }

    let out_len = n - (period - 1);
    let mut out_plus  = vec![0.0f64; out_len];
    let mut out_minus = vec![0.0f64; out_len];

    out_plus[0] = s_plus;
    out_minus[0] = s_minus;

    for j in 1..out_len {
        let i = period - 1 + j;
        let up_move = high[i] - high[i - 1];
        let dn_move = low[i - 1] - low[i];
        s_plus  = s_plus  * k + if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
        s_minus = s_minus * k + if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
        out_plus[j] = s_plus;
        out_minus[j] = s_minus;
    }

    Some((out_plus, out_minus))
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

    for i in 1..period {
        let up_move = high[i] - high[i - 1];
        let dn_move = low[i - 1] - low[i];
        s_plus  += if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
        s_minus += if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
        let hl = high[i] - low[i];
        let hc = (high[i] - close[i - 1]).abs();
        let lc = (low[i] - close[i - 1]).abs();
        s_tr += hl.max(hc).max(lc);
    }

    {
        let up_move = high[period] - high[period - 1];
        let dn_move = low[period - 1] - low[period];
        s_plus  = s_plus  * k + if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
        s_minus = s_minus * k + if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
        let hl = high[period] - low[period];
        let hc = (high[period] - close[period - 1]).abs();
        let lc = (low[period] - close[period - 1]).abs();
        s_tr = s_tr * k + hl.max(hc).max(lc);
    }

    let out_len = n - period;
    let mut out_plus  = vec![0.0f64; out_len];
    let mut out_minus = vec![0.0f64; out_len];
    let mut out_tr    = vec![0.0f64; out_len];

    out_plus[0] = s_plus;
    out_minus[0] = s_minus;
    out_tr[0] = s_tr;

    for j in 1..out_len {
        let i = period + j;
        let up_move = high[i] - high[i - 1];
        let dn_move = low[i - 1] - low[i];
        s_plus  = s_plus  * k + if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
        s_minus = s_minus * k + if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
        let hl = high[i] - low[i];
        let hc = (high[i] - close[i - 1]).abs();
        let lc = (low[i] - close[i - 1]).abs();
        s_tr = s_tr * k + hl.max(hc).max(lc);
        out_plus[j] = s_plus;
        out_minus[j] = s_minus;
        out_tr[j] = s_tr;
    }

    Some((out_plus, out_minus, out_tr))
}
