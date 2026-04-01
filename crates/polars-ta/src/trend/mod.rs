//! Trend indicators: Polars Series wrappers.

pub mod adx;
pub mod bbands;
pub mod ema;
pub mod ma;
pub mod macd;
pub mod macdext;
pub mod macdfix;
pub mod sar;
pub mod sarext;
pub mod sma;

pub use adx::adx_series;
pub use bbands::{bbands_series, BbandsSeriesOutput};
pub use ema::{dema_series, ema_series, tema_series, wma_series};
pub use ma::ma_series;
pub use macd::{macd_series, MacdSeriesOutput};
pub use macdext::{macdext_series, MacdExtSeriesOutput};
pub use macdfix::{macdfix_series, MacdFixSeriesOutput};
pub use sar::sar_series;
pub use sarext::sarext_series;
pub use sma::sma_series;
