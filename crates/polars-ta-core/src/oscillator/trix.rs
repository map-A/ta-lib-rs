//! 1-day Rate-Of-Change of a Triple Smooth EMA (TRIX)
//!
//! `TRIX[i] = (EMA3[i+1] - EMA3[i]) / EMA3[i] * 100`
//! Lookback = `3 * (period - 1) + 1`.

use crate::trend::ema::ema;

pub fn trix(data: &[f64], period: usize) -> Vec<f64> {
    if period == 0 || data.len() < 3 * (period - 1) + 2 {
        return vec![];
    }
    let e1 = ema(data, period);
    let e2 = ema(&e1, period);
    let e3 = ema(&e2, period);
    if e3.len() < 2 {
        return vec![];
    }
    let out: Vec<f64> = e3
        .windows(2)
        .map(|w| {
            let prev = w[0];
            if prev == 0.0 {
                0.0
            } else {
                (w[1] - prev) / prev * 100.0
            }
        })
        .collect();
    out
}
