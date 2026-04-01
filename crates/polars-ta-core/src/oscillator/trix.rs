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
    let out_len = e3.len() - 1;
    let mut out = Vec::with_capacity(out_len);
    // Safety: e3 has e3.len() >= 2 elements. p0 starts at e3[0], p1 at e3[1].
    // Both pointers advance out_len times, staying within e3's allocation.
    // dst advances out_len times within out's allocation.
    unsafe {
        out.set_len(out_len);
        let mut p0 = e3.as_ptr();
        let mut p1 = e3.as_ptr().add(1);
        let mut dst = out.as_mut_ptr();
        for _ in 0..out_len {
            let prev = *p0;
            *dst = if prev == 0.0 { 0.0 } else { (*p1 - prev) / prev * 100.0 };
            p0 = p0.add(1);
            p1 = p1.add(1);
            dst = dst.add(1);
        }
    }
    out
}
