//! Chande Momentum Oscillator (CMO)
//!
//! ta-lib CMO uses Wilder-style smoothing (identical to RSI mechanism):
//! 1. Seed: average gain/loss over first `period` deltas
//! 2. Update: `avg_up = (avg_up * (period-1) + gain) / period`
//! 3. `CMO = 100 * (avg_up - avg_dn) / (avg_up + avg_dn)`
//!
//! Output length = `n - period` (lookback = period).

pub fn cmo(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n <= period {
        return vec![];
    }

    let out_len = n - period;
    let mut out = Vec::with_capacity(out_len);
    let inv = 1.0 / period as f64;
    let decay = (period - 1) as f64 * inv;

    // 用前 period 个 delta 初始化平均涨幅/跌幅
    let mut avg_up = 0.0_f64;
    let mut avg_dn = 0.0_f64;
    for i in 1..=period {
        let d = data[i] - data[i - 1];
        if d > 0.0 {
            avg_up += d;
        } else {
            avg_dn -= d;
        }
    }
    avg_up *= inv;
    avg_dn *= inv;

    let total = avg_up + avg_dn;
    out.push(if total == 0.0 { 0.0 } else { 100.0 * (avg_up - avg_dn) / total });

    // Wilder 平滑更新
    for i in (period + 1)..n {
        let d = data[i] - data[i - 1];
        let gain = if d > 0.0 { d } else { 0.0 };
        let loss = if d < 0.0 { -d } else { 0.0 };
        avg_up = avg_up * decay + gain * inv;
        avg_dn = avg_dn * decay + loss * inv;
        let total = avg_up + avg_dn;
        out.push(if total == 0.0 { 0.0 } else { 100.0 * (avg_up - avg_dn) / total });
    }

    out
}
