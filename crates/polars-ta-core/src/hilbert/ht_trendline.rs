//! HT_TRENDLINE — Hilbert Transform Instantaneous Trendline
//! The trendline is a smoothed price using the Hilbert Transform dominant cycle.
use super::core::{HtState, ht_step, HT_LOOKBACK};

/// Returns the trendline starting at index `HT_LOOKBACK`.
pub fn ht_trendline(close: &[f64]) -> Vec<f64> {
    let n = close.len();
    if n <= HT_LOOKBACK { return vec![]; }
    let out_len = n - HT_LOOKBACK;
    let mut out = vec![0.0f64; out_len];
    let mut state = HtState::new();
    // Price history for trendline calculation (circular buffer of 4 smoothed prices)
    let mut dc_period_int;
    for i in 0..HT_LOOKBACK {
        let _ = ht_step(close[i], &mut state);
    }
    for i in 0..out_len {
        let (dc, _) = ht_step(close[i + HT_LOOKBACK], &mut state);
        dc_period_int = dc.round() as usize;
        if dc_period_int < 1 { dc_period_int = 1; }
        // trendline = avg of last dc_period/2 prices, clipped at available
        let half = (dc_period_int / 2).max(1).min(i + 1);
        let start = (i + HT_LOOKBACK + 1).saturating_sub(half);
        let sum: f64 = close[start..=i + HT_LOOKBACK].iter().sum();
        out[i] = sum / half as f64;
    }
    out
}
