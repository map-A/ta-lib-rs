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

    let raw_dm = |i: usize| -> (f64, f64) {
        let up_move = high[i] - high[i - 1];
        let dn_move = low[i - 1] - low[i];
        let plus = if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
        let minus = if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
        (plus, minus)
    };

    let pf = period as f64;
    let k = 1.0 - 1.0 / pf;

    // Accumulate raw DM for indices 1..period (= period-1 values)
    let mut s_plus = 0.0_f64;
    let mut s_minus = 0.0_f64;
    for i in 1..period {
        let (p, m) = raw_dm(i);
        s_plus += p;
        s_minus += m;
    }

    let out_len = n - (period - 1);
    let mut out_plus = Vec::with_capacity(out_len);
    let mut out_minus = Vec::with_capacity(out_len);

    // First output = initial sum (no Wilder step)
    out_plus.push(s_plus);
    out_minus.push(s_minus);

    // Wilder smoothing for remaining values
    for i in period..n {
        let (p, m) = raw_dm(i);
        s_plus = s_plus * k + p;
        s_minus = s_minus * k + m;
        out_plus.push(s_plus);
        out_minus.push(s_minus);
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

    let tr = |i: usize| -> f64 {
        let hl = high[i] - low[i];
        let hc = (high[i] - close[i - 1]).abs();
        let lc = (low[i] - close[i - 1]).abs();
        hl.max(hc).max(lc)
    };

    let raw_dm = |i: usize| -> (f64, f64) {
        let up_move = high[i] - high[i - 1];
        let dn_move = low[i - 1] - low[i];
        let plus = if up_move > dn_move && up_move > 0.0 { up_move } else { 0.0 };
        let minus = if dn_move > up_move && dn_move > 0.0 { dn_move } else { 0.0 };
        (plus, minus)
    };

    let pf = period as f64;
    let k = 1.0 - 1.0 / pf;

    // Accumulate for indices 1..period (= period-1 values)
    let mut s_plus = 0.0_f64;
    let mut s_minus = 0.0_f64;
    let mut s_tr = 0.0_f64;
    for i in 1..period {
        let (p, m) = raw_dm(i);
        s_plus += p;
        s_minus += m;
        s_tr += tr(i);
    }

    // One Wilder step at index period to get first smoothed value
    {
        let (p, m) = raw_dm(period);
        s_plus = s_plus * k + p;
        s_minus = s_minus * k + m;
        s_tr = s_tr * k + tr(period);
    }

    let out_len = n - period;
    let mut out_plus = Vec::with_capacity(out_len);
    let mut out_minus = Vec::with_capacity(out_len);
    let mut out_tr = Vec::with_capacity(out_len);

    out_plus.push(s_plus);
    out_minus.push(s_minus);
    out_tr.push(s_tr);

    for i in (period + 1)..n {
        let (p, m) = raw_dm(i);
        s_plus = s_plus * k + p;
        s_minus = s_minus * k + m;
        s_tr = s_tr * k + tr(i);
        out_plus.push(s_plus);
        out_minus.push(s_minus);
        out_tr.push(s_tr);
    }

    Some((out_plus, out_minus, out_tr))
}
