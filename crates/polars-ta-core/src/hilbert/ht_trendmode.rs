//! HT_TRENDMODE — Hilbert Transform Trend vs Cycle Mode.
//!
//! Lookback = 63, but returns `Vec<f64>` of length `n` with `0.0` (not NaN)
//! for the first 63 bars, matching ta-lib's integer output behaviour.
use super::core::{run_ht_engine, HT_LOOKBACK_LARGE};

pub fn ht_trendmode(close: &[f64]) -> Vec<f64> {
    let n = close.len();
    let results = run_ht_engine(close, 34);
    let mut out = vec![0.0_f64; n];

    let mut days_in_trend: i32 = 0;
    let mut prev_sine = 0.0_f64;
    let mut prev_lead_sine = 0.0_f64;

    // State machine runs from the first bar that has HT results.
    // Output 0.0 for bars before HT_LOOKBACK_LARGE; computed trend afterwards.
    for bar in 0..n {
        let r = match &results[bar] {
            Some(r) => r,
            None => continue,
        };

        let sp = r.smooth_period;
        let sine = r.sine;
        let lead_sine = r.lead_sine;
        let dc_phase = r.dc_phase;
        let prev_dc_phase = r.prev_dc_phase;
        let trendline = r.trendline;
        let smooth_cur = r.smooth_price_cur;

        let mut trend = 1_i32;
        if (sine > lead_sine && prev_sine <= prev_lead_sine)
            || (sine < lead_sine && prev_sine >= prev_lead_sine)
        {
            days_in_trend = 0;
            trend = 0;
        }
        days_in_trend += 1;
        if (days_in_trend as f64) < 0.5 * sp {
            trend = 0;
        }
        let temp = dc_phase - prev_dc_phase;
        if sp != 0.0 && temp > 0.67 * 360.0 / sp && temp < 1.5 * 360.0 / sp {
            trend = 0;
        }
        if trendline != 0.0 && ((smooth_cur - trendline) / trendline).abs() >= 0.015 {
            trend = 1;
        }

        prev_sine = sine;
        prev_lead_sine = lead_sine;

        if bar >= HT_LOOKBACK_LARGE {
            out[bar] = trend as f64;
        }
        // bars 0..HT_LOOKBACK_LARGE stay 0.0
    }

    out
}
