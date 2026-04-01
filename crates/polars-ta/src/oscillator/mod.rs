//! Oscillator indicators: Polars Series wrappers.

pub mod aroon;
pub mod aroonosc;
pub mod cci;
pub mod mfi;
pub mod rsi;
pub mod stoch;
pub mod stochf;
pub mod stochrsi;
pub mod ultosc;
pub mod willr;

pub use aroon::{aroon_series, AroonSeriesOutput};
pub use aroonosc::aroonosc_series;
pub use cci::cci_series;
pub use mfi::mfi_series;
pub use rsi::rsi_series;
pub use stoch::{stoch_series, StochSeriesOutput};
pub use stochf::{stochf_series, StochFSeriesOutput};
pub use stochrsi::{stochrsi_series, StochRsiSeriesOutput};
pub use ultosc::ultosc_series;
pub use willr::willr_series;
