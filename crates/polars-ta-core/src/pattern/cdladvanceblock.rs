//! CDLADVANCEBLOCK — Advance Block
//! Three advancing bullish candles with progressively smaller bodies (bearish signal).
use super::helpers::*;

pub fn cdladvanceblock(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let n = open.len();
    let mut out = vec![0.0f64; n];
    // Lookback = max(ShadowShort=10, Near=5, Far=5, BodyLong=10) + 2 = 12
    let body_p = BODY_LONG_PERIOD; // 10
    let shad_p = SHADOW_SHORT_PERIOD; // 10
    let near_p = NEAR_PERIOD; // 5
    let far_p = FAR_PERIOD; // 5
    let lookback = body_p.max(shad_p).max(near_p).max(far_p) + 2;
    if n <= lookback {
        return out;
    }

    // Init: ShadowShort[k] anchored at i-k, trailing = startIdx - shad_p
    // trailing_ss = lookback - shad_p = 12 - 10 = 2
    // init loop i = 2..11 (while i < 12): adds bars [i-2, i-1, i]
    let ts = lookback - shad_p; // = 2
    let shadows = |i: usize| {
        upper_shadow(open[i], high[i], close[i]) + lower_shadow(open[i], low[i], close[i])
    };
    let mut ss: [f64; 3] = [
        (ts..lookback).map(&shadows).sum(),           // [0]: bars[2..11]
        (ts..lookback).map(|i| shadows(i - 1)).sum(), // [1]: bars[1..10]
        (ts..lookback).map(|i| shadows(i - 2)).sum(), // [0]: bars[0..9]
    ];
    // Near/Far: trailing = lookback - near_p = 7, init bars [i-2, i-1] for i=7..11
    let tn = lookback - near_p; // = 7
    let mut near: [f64; 3] = [
        0.0,
        (tn..lookback)
            .map(|i| hl_range(high[i - 1], low[i - 1]))
            .sum(), // [1]: bars[6..10]
        (tn..lookback)
            .map(|i| hl_range(high[i - 2], low[i - 2]))
            .sum(), // [2]: bars[5..9]
    ];
    let mut far: [f64; 3] = [
        0.0,
        (tn..lookback)
            .map(|i| hl_range(high[i - 1], low[i - 1]))
            .sum(),
        (tn..lookback)
            .map(|i| hl_range(high[i - 2], low[i - 2]))
            .sum(),
    ];
    // BodyLong: trailing = lookback - body_p = 2, init bars [i-2] for i=2..11
    let tb = lookback - body_p; // = 2
    let mut body_sum: f64 = (tb..lookback)
        .map(|i| real_body(open[i - 2], close[i - 2]))
        .sum();

    let mut ss_trail = ts;
    let mut n_trail = tn;
    let mut f_trail = tn;
    let mut b_trail = tb;

    for i in lookback..n {
        let avg_body = body_sum / body_p as f64;
        // ShadowShort averages (Shadows range, factor=0.5)
        let avg_ss2 = ss[2] / shad_p as f64 * SHADOW_SHORT_FACTOR; // for i-2
        let avg_ss1 = ss[1] / shad_p as f64 * SHADOW_SHORT_FACTOR; // for i-1
        let avg_ss0 = ss[0] / shad_p as f64 * SHADOW_SHORT_FACTOR; // for i
                                                                   // ShadowLong: period=0, direct comparison
        let _avg_sl0 = real_body(open[i], close[i]) * SHADOW_VERY_LONG_FACTOR / 2.0; // factor=1.0, ShadowLong
        let _avg_sl1 = real_body(open[i - 1], close[i - 1]) * SHADOW_VERY_LONG_FACTOR / 2.0;
        // Near/Far averages (HighLow range)
        let avg_near2 = near[2] / near_p as f64 * NEAR_FACTOR;
        let avg_near1 = near[1] / near_p as f64 * NEAR_FACTOR;
        let avg_far2 = far[2] / far_p as f64 * FAR_FACTOR;
        let avg_far1 = far[1] / far_p as f64 * FAR_FACTOR;

        let rb0 = real_body(open[i - 2], close[i - 2]);
        let rb1 = real_body(open[i - 1], close[i - 1]);
        let rb2 = real_body(open[i], close[i]);
        let us0 = upper_shadow(open[i - 2], high[i - 2], close[i - 2]);
        let us1 = upper_shadow(open[i - 1], high[i - 1], close[i - 1]);
        let us2 = upper_shadow(open[i], high[i], close[i]);
        // ShadowLong (period=0) for i and i-1
        let sl0 = rb2 * SHADOW_LONG_FACTOR; // ShadowLong avg = rb[i] * 1.0
        let _sl1 = rb1 * SHADOW_LONG_FACTOR;

        let is_pattern = candle_color(open[i-2], close[i-2]) == 1 &&
            candle_color(open[i-1], close[i-1]) == 1 &&
            candle_color(open[i],   close[i])   == 1 &&
            close[i] > close[i-1] && close[i-1] > close[i-2] &&
            // 2nd opens within/near 1st body
            open[i-1] > open[i-2] &&
            open[i-1] <= close[i-2] + avg_near2 &&
            // 3rd opens within/near 2nd body
            open[i] > open[i-1] &&
            open[i] <= close[i-1] + avg_near1 &&
            // 1st: long body
            rb0 > avg_body * BODY_LONG_FACTOR &&
            // 1st: short upper shadow
            us0 < avg_ss2 &&
            // weakening: one of four conditions
            (
                // 2 far smaller than 1 AND 3 not longer than 2
                (rb1 < rb0 - avg_far2 && rb2 < rb1 + avg_near1) ||
                // 3 far smaller than 2
                (rb2 < rb1 - avg_far1) ||
                // 3<2<1 and (3 or 2 has long upper shadow)
                (rb2 < rb1 && rb1 < rb0 && (us2 > avg_ss0 || us1 > avg_ss1)) ||
                // 3 smaller than 2 and 3 has long upper shadow (ShadowLong)
                (rb2 < rb1 && us2 > sl0)
            );

        if is_pattern {
            out[i] = -100.0;
        }

        // Update rolling sums (all anchored at current bar, trailing advances)
        for (k, item) in ss.iter_mut().enumerate() {
            *item += shadows(i - k) - shadows(ss_trail.wrapping_sub(k));
        }
        for k in 1..3usize {
            near[k] += hl_range(high[i - k], low[i - k])
                - hl_range(high[n_trail.wrapping_sub(k)], low[n_trail.wrapping_sub(k)]);
            far[k] += hl_range(high[i - k], low[i - k])
                - hl_range(high[f_trail.wrapping_sub(k)], low[f_trail.wrapping_sub(k)]);
        }
        body_sum += real_body(open[i - 2], close[i - 2])
            - real_body(
                open[b_trail.wrapping_sub(2)],
                close[b_trail.wrapping_sub(2)],
            );
        ss_trail += 1;
        n_trail += 1;
        f_trail += 1;
        b_trail += 1;
    }
    out
}
