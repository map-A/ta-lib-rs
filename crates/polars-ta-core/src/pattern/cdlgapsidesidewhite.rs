//! CDLGAPSIDESIDEWHITE — Up/Down-Gap Side-By-Side White Lines
use super::helpers::*;

pub fn cdlgapsidesidewhite(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = NEAR_PERIOD.max(BODY_SHORT_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut near_sum: f64 = (0..NEAR_PERIOD).map(|j| hl_range(high[j], low[j])).sum();
    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg_near = near_sum / NEAR_PERIOD as f64;
        let avg_body = body_sum / period as f64;
        let rb1 = real_body(open[i-1], close[i-1]);
        let rb2 = real_body(open[i], close[i]);

        // Up gap: candle[i-2] bullish, gap up between i-2 and i-1, 
        //         candle[i-1] and candle[i] both bullish with similar opens and bodies
        let up = candle_color(open[i-2], close[i-2]) == 1 &&
            candle_color(open[i-1], close[i-1]) == 1 &&
            candle_color(open[i], close[i]) == 1 &&
            open[i-1] > close[i-2] &&           // gap up
            (open[i] - open[i-1]).abs() <= avg_near * NEAR_FACTOR &&  // similar opens
            (rb2 - rb1).abs() <= avg_near * NEAR_FACTOR;              // similar bodies

        // Down gap: candle[i-2] bearish, gap down, both i-1 and i bullish
        let dn = candle_color(open[i-2], close[i-2]) == -1 &&
            candle_color(open[i-1], close[i-1]) == 1 &&
            candle_color(open[i], close[i]) == 1 &&
            open[i-1] < close[i-2] &&           // gap down
            (open[i] - open[i-1]).abs() <= avg_near * NEAR_FACTOR &&
            (rb2 - rb1).abs() <= avg_near * NEAR_FACTOR;

        if up { out[i] = 100.0; }
        if dn { out[i] = -100.0; }

        near_sum += hl_range(high[i-1], low[i-1]);
        near_sum -= hl_range(high[i-1-NEAR_PERIOD], low[i-1-NEAR_PERIOD]);
        body_sum += real_body(open[i-1], close[i-1]);
        body_sum -= real_body(open[i-1-period], close[i-1-period]);
    }
    out
}
