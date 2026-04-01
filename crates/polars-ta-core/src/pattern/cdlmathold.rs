//! CDLMATHOLD — Mat Hold
//! Bullish continuation: strong bullish + 4 small candles contained + closing bullish.
use super::helpers::*;

pub fn cdlmathold(open: &[f64], _high: &[f64], _low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_SHORT_PERIOD.max(BODY_LONG_PERIOD);
    let lookback = period + 4;
    if n <= lookback { return out; }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;

        // First candle: long bullish
        let c0_bull = candle_color(open[i-4], close[i-4]) == 1
            && real_body(open[i-4], close[i-4]) > avg * BODY_LONG_FACTOR;
        // Next 3 candles: short bodies
        let mid_small = real_body(open[i-3], close[i-3]) < avg * BODY_SHORT_FACTOR
            && real_body(open[i-2], close[i-2]) < avg * BODY_SHORT_FACTOR
            && real_body(open[i-1], close[i-1]) < avg * BODY_SHORT_FACTOR;
        // All middles contained within first candle's range
        let mid_contained = open[i-3].max(close[i-3]) < close[i-4]
            && open[i-2].max(close[i-2]) < close[i-4]
            && open[i-1].max(close[i-1]) < close[i-4]
            && open[i-3].min(close[i-3]) > open[i-4]
            && open[i-2].min(close[i-2]) > open[i-4]
            && open[i-1].min(close[i-1]) > open[i-4];
        // Last candle: bullish, closes above first close
        let c4_bull = candle_color(open[i], close[i]) == 1 && close[i] > close[i-4];

        if c0_bull && mid_small && mid_contained && c4_bull { out[i] = 100.0; }

        body_sum += real_body(open[i-4], close[i-4]);
        body_sum -= real_body(open[i-4-period], close[i-4-period]);
    }
    out
}
