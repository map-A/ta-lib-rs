//! Trend indicators: moving averages and trend-following oscillators.

pub mod adx;
pub mod bbands;
pub mod dema;
pub mod ema;
pub mod kama;
pub mod macd;
pub mod midpoint;
pub mod midprice;
pub mod sar;
pub mod sma;
pub mod t3;
pub mod tema;
pub mod trima;
pub mod wma;

pub use adx::adx;
pub use bbands::{bbands, BbandsOutput};
pub use dema::dema;
pub use ema::ema;
pub use kama::kama;
pub use macd::{macd, MacdOutput};
pub use midpoint::midpoint;
pub use midprice::midprice;
pub use sar::sar;
pub use sma::sma;
pub use t3::t3;
pub use tema::tema;
pub use trima::trima;
pub use wma::wma;
