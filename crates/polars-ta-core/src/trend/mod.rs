//! Trend indicators: moving averages and trend-following oscillators.

pub mod adx;
pub mod bbands;
pub mod dema;
pub mod ema;
pub mod kama;
pub mod ma;
pub mod macd;
pub mod macdext;
pub mod macdfix;
pub mod midpoint;
pub mod midprice;
pub mod sar;
pub mod sarext;
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
pub use ma::ma;
pub use macd::{macd, MacdOutput};
pub use macdext::macdext;
pub use macdfix::macdfix;
pub use midpoint::midpoint;
pub use midprice::midprice;
pub use sar::sar;
pub use sarext::sarext;
pub use sma::sma;
pub use t3::t3;
pub use tema::tema;
pub use trima::trima;
pub use wma::wma;
