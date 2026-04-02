//! CDLBREAKAWAY — Breakaway (5-candle pattern)
use super::helpers::*;

pub fn cdlbreakaway(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_LONG_PERIOD;
    let lookback = period + 4;
    if n <= lookback {
        return out;
    }

    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg = body_sum / period as f64;
        let rb0 = real_body(open[i - 4], close[i - 4]);

        // Bullish: bearish[i-4] long, gap down [i-3], 2 more down below gap, bullish[i] closes within gap
        let bull = candle_color(open[i-4], close[i-4]) == -1 &&
            rb0 > avg * BODY_LONG_FACTOR &&
            candle_color(open[i-3], close[i-3]) == -1 &&
            high[i-3] < low[i-4] &&     // strict gap down (no shadow overlap)
            candle_color(open[i-2], close[i-2]) == -1 &&
            candle_color(open[i-1], close[i-1]) == -1 &&
            close[i-3] > close[i-2] && close[i-2] > close[i-1] &&
            high[i-2] < low[i-4] &&     // middle candles stay below gap
            high[i-1] < low[i-4] &&
            candle_color(open[i], close[i]) == 1 &&
            close[i] > open[i-3] && close[i] < close[i-4];

        // Bearish: bullish[i-4] long, gap up [i-3], 2 more up above gap, bearish[i] closes within gap
        let bear = candle_color(open[i-4], close[i-4]) == 1 &&
            rb0 > avg * BODY_LONG_FACTOR &&
            candle_color(open[i-3], close[i-3]) == 1 &&
            low[i-3] > high[i-4] &&     // strict gap up (no shadow overlap)
            candle_color(open[i-2], close[i-2]) == 1 &&
            candle_color(open[i-1], close[i-1]) == 1 &&
            close[i-3] < close[i-2] && close[i-2] < close[i-1] &&
            low[i-2] > high[i-4] &&     // middle candles stay above gap
            low[i-1] > high[i-4] &&
            candle_color(open[i], close[i]) == -1 &&
            close[i] < open[i-3] && close[i] > close[i-4];

        if bull {
            out[i] = 100.0;
        }
        if bear {
            out[i] = -100.0;
        }

        body_sum += real_body(open[i - 4], close[i - 4]);
        body_sum -= real_body(open[i - 4 - period], close[i - 4 - period]);
    }
    out
}
