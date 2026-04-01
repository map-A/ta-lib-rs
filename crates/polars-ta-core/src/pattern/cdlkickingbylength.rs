//! CDLKICKINGBYLENGTH — Kicking - bull/bear determined by the longer marubozu
//! Same as kicking but the dominant side is the one with the longer body.
use super::helpers::*;

pub fn cdlkickingbylength(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = SHADOW_VERY_SHORT_PERIOD;
    let lookback = period + 1;
    if n <= lookback { return out; }

    let mut shadow_sum: f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();

    for i in lookback..n {
        let avg_shadow = shadow_sum / period as f64;
        let thr = avg_shadow * SHADOW_VERY_SHORT_FACTOR;

        let c0 = candle_color(open[i-1], close[i-1]);
        let c1 = candle_color(open[i], close[i]);
        let rb0 = real_body(open[i-1], close[i-1]);
        let rb1 = real_body(open[i], close[i]);

        let maru0 = upper_shadow(open[i-1], high[i-1], close[i-1]) < thr
            && lower_shadow(open[i-1], low[i-1], close[i-1]) < thr;
        let maru1 = upper_shadow(open[i], high[i], close[i]) < thr
            && lower_shadow(open[i], low[i], close[i]) < thr;

        let gap_up = c0 == -1 && c1 == 1 && open[i] > open[i-1];
        let gap_dn = c0 == 1 && c1 == -1 && open[i] < open[i-1];

        if maru0 && maru1 {
            if gap_up {
                // Bullish kicking: which is longer determines result
                out[i] = if rb1 >= rb0 { 100.0 } else { -100.0 };
            } else if gap_dn {
                out[i] = if rb1 >= rb0 { -100.0 } else { 100.0 };
            }
        }

        shadow_sum += hl_range(high[i-1], low[i-1]);
        shadow_sum -= hl_range(high[i-1-period], low[i-1-period]);
    }
    out
}
