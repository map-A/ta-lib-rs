//! CDLEVENINGDOJISTAR — Evening Doji Star (bearish reversal)
use super::helpers::*;

pub fn cdleveningdojistar(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    cdleveningdojistar_with_penetration(open, high, low, close, 0.3)
}

pub fn cdleveningdojistar_with_penetration(
    open: &[f64], high: &[f64], low: &[f64], close: &[f64], penetration: f64
) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = BODY_DOJI_PERIOD.max(BODY_LONG_PERIOD);
    let lookback = period + 2;
    if n <= lookback { return out; }

    let mut hl_sum:   f64 = (0..period).map(|j| hl_range(high[j], low[j])).sum();
    let mut body_sum: f64 = (0..period).map(|j| real_body(open[j], close[j])).sum();

    for i in lookback..n {
        let avg_hl   = hl_sum   / period as f64;
        let avg_body = body_sum / period as f64;
        let rb0 = real_body(open[i-2], close[i-2]);
        let rb1 = real_body(open[i-1], close[i-1]);

        let is_pattern =
            candle_color(open[i-2], close[i-2]) == 1 &&          // 1st: bullish long
            rb0 > avg_body * BODY_LONG_FACTOR &&
            rb1 <= avg_hl * BODY_DOJI_FACTOR &&                   // 2nd: doji
            open[i-1].min(close[i-1]) > close[i-2] &&            // 2nd gaps above 1st
            candle_color(open[i], close[i]) == -1 &&             // 3rd: bearish
            close[i] < close[i-2] - rb0 * penetration;           // 3rd closes well into 1st

        if is_pattern { out[i] = -100.0; }

        hl_sum   += hl_range(high[i-1], low[i-1]);
        hl_sum   -= hl_range(high[i-1-period], low[i-1-period]);
        body_sum += real_body(open[i-2], close[i-2]);
        body_sum -= real_body(open[i-2-period], close[i-2-period]);
    }
    out
}
