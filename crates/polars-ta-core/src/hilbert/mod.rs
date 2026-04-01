//! Hilbert Transform indicators — 6 functions matching ta-lib.
//!
//! These use the same Hilbert Transform kernel as ta-lib's `TA_INT_*` internal functions.
//! The lookback for all HT indicators is 63 bars.
//!
//! # References
//! John Ehlers, "Rocket Science for Traders", 2001.

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
