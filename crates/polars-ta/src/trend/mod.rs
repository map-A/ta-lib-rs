//! Trend indicators: Polars Series wrappers.

pub mod adx;
pub mod bbands;
pub mod ema;
pub mod macd;
pub mod sar;
pub mod sma;

pub use adx::adx_series;
pub use bbands::{bbands_series, BbandsSeriesOutput};
pub use ema::{dema_series, ema_series, tema_series, wma_series};
pub use macd::{macd_series, MacdSeriesOutput};
pub use sar::sar_series;
pub use sma::sma_series;
