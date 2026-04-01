//! CDL3BLACKCROWS — Three Black Crows
//! Three consecutive long bearish candles, each opening within previous body.
use super::helpers::*;

pub fn cdl3blackcrows(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    let period = SHADOW_VERY_SHORT_PERIOD;
    let lookback = period + 2;
    if n <= lookback { return out; }

    // Shadow avg (HL range) for very short shadow check
    let mut shadow_sum: [f64; 3] = [
        (0..period).map(|j| hl_range(high[j], low[j])).sum(),
        (0..period).map(|j| hl_range(high[j], low[j])).sum(),
        (0..period).map(|j| hl_range(high[j], low[j])).sum(),
    ];

    for i in lookback..n {
        // shadow_sum[k] tracks the HL avg for candle i-2+k
        let avg0 = shadow_sum[0] / period as f64;
        let avg1 = shadow_sum[1] / period as f64;
        let avg2 = shadow_sum[2] / period as f64;

        let is_pattern =
            candle_color(open[i-2], close[i-2]) == -1 &&         // all 3 bearish
            candle_color(open[i-1], close[i-1]) == -1 &&
            candle_color(open[i],   close[i])   == -1 &&
            close[i] < close[i-1] && close[i-1] < close[i-2] &&  // each closes lower
            open[i-1] < open[i-2] && open[i-1] > close[i-2] &&   // 2nd opens in 1st body
            open[i]   < open[i-1] && open[i]   > close[i-1] &&   // 3rd opens in 2nd body
            upper_shadow(open[i-2], high[i-2], close[i-2]) < avg0 * SHADOW_VERY_SHORT_FACTOR &&
            upper_shadow(open[i-1], high[i-1], close[i-1]) < avg1 * SHADOW_VERY_SHORT_FACTOR &&
            upper_shadow(open[i],   high[i],   close[i])   < avg2 * SHADOW_VERY_SHORT_FACTOR;

        if is_pattern { out[i] = -100.0; }

        // Slide all 3 shadow windows
        for k in 0..3usize {
            let j = i - 2 + k;
            shadow_sum[k] += hl_range(high[j], low[j]);
            shadow_sum[k] -= hl_range(high[j - period], low[j - period]);
        }
    }
    out
}
