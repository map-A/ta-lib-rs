//! CDLMORNINGSTAR — Morning Star
//! Bearish candle + small-body star (gap down) + bullish candle closes into first body.
//! Uses `penetration=0.3` (default) — candle 2 close must enter 30% into candle 0 body.
use super::helpers::*;

const PENETRATION: f64 = 0.3;

pub fn cdlmorningstar(open: &[f64], _high: &[f64], _low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD.max(BODY_SHORT_PERIOD);
    let lookback = period + 2;
    if n <= lookback {
        return out;
    }

    // Three independent rolling sums (all RealBody), anchored at i-2, i-1, i
    let mut bl_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();
    let mut bs1_sum: f64 = (1..=period).map(|j| real_body(open[j], close[j])).sum();
    let mut bs2_sum: f64 = (2..period + 2).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let bl_avg = bl_sum / period as f64;
        let bs1_avg = bs1_sum / period as f64;
        let bs2_avg = bs2_sum / period as f64;

        let rb0 = real_body(open[i - 2], close[i - 2]);
        let rb1 = real_body(open[i - 1], close[i - 1]);
        let rb2 = real_body(open[i], close[i]);

        // Penetration level: close[i] must be above close[i-2] + penetration*(open[i-2]-close[i-2])
        let body0 = open[i - 2] - close[i - 2]; // positive for bearish
        let pen_level = close[i - 2] + PENETRATION * body0;

        let is_pattern =
            // Candle 0: long bearish
            candle_color(open[i-2], close[i-2]) == -1
            && rb0 > bl_avg * BODY_LONG_FACTOR
            // Candle 1: short body, gap down (star)
            && rb1 < bs1_avg * BODY_SHORT_FACTOR
            && open[i-1].max(close[i-1]) < close[i-2]
            // Candle 2: bullish, penetrates into first candle body
            && candle_color(open[i], close[i]) == 1
            && rb2 > bs2_avg * BODY_SHORT_FACTOR
            && close[i] > pen_level;

        if is_pattern {
            out[i] = 100.0;
        }

        // Advance each rolling sum by one bar
        bl_sum += real_body(open[i - 2], close[i - 2]);
        bl_sum -= real_body(open[i - 2 - period], close[i - 2 - period]);
        bs1_sum += real_body(open[i - 1], close[i - 1]);
        bs1_sum -= real_body(open[i - 1 - period], close[i - 1 - period]);
        bs2_sum += real_body(open[i], close[i]);
        bs2_sum -= real_body(open[i - period], close[i - period]);
    }
    out
}
