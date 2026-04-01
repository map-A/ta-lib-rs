//! CDLTRISTAR — Tristar Pattern
//! Three consecutive doji candles with gaps between them.
use super::helpers::*;

pub fn cdltristar(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_DOJI_PERIOD;
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;

        let d0 = real_body(open[i-2], close[i-2]) <= avg * BODY_DOJI_FACTOR;
        let d1 = real_body(open[i-1], close[i-1]) <= avg * BODY_DOJI_FACTOR;
        let d2 = real_body(open[i], close[i]) <= avg * BODY_DOJI_FACTOR;

        if d0 && d1 && d2 {
            // REALBODYGAPUP(i-1,i-2): min(o,c)[i-1] > max(o,c)[i-2]
            let gap_up = open[i-1].min(close[i-1]) > open[i-2].max(close[i-2]);
            // REALBODYGAPDOWN(i-1,i-2): max(o,c)[i-1] < min(o,c)[i-2]
            let gap_dn = open[i-1].max(close[i-1]) < open[i-2].min(close[i-2]);
            if gap_up && open[i].max(close[i]) < open[i-1].max(close[i-1]) {
                out[i] = -100.0;
            }
            if gap_dn && open[i].min(close[i]) > open[i-1].min(close[i-1]) {
                out[i] = 100.0;
            }
        }

        body_sum += hl_range(high[i-2], low[i-2]);
        body_sum -= hl_range(high[i-2-period], low[i-2-period]);
    }
    out
}
