//! Hilbert Transform indicators — 6 functions matching ta-lib exactly.
//!
//! Two groups share an engine but differ in warmup depth:
//! - Lookback 32 (DCPeriod, Phasor): WMA warmup i=9, HT from bar 12.
//! - Lookback 63 (DCPhase, Sine, Trendline, Trendmode): WMA warmup i=34, HT from bar 37.
//!
//! All functions return `Vec<f64>` of length `n`; positions before the lookback
//! contain `NaN` (except `ht_trendmode` which uses `0.0`).

pub(crate) mod core;

pub mod ht_dcperiod;
pub mod ht_dcphase;
pub mod ht_phasor;
pub mod ht_sine;
pub mod ht_trendline;
pub mod ht_trendmode;

pub use ht_dcperiod::ht_dcperiod;
pub use ht_dcphase::ht_dcphase;
pub use ht_phasor::ht_phasor;
pub use ht_sine::ht_sine;
pub use ht_trendline::ht_trendline;
pub use ht_trendmode::ht_trendmode;
