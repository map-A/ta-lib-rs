//! # polars-ta
//!
//! Native Polars Series interface for technical analysis indicators.
//! Numerically identical to ta-lib's C implementation.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use polars::prelude::*;
//! use polars_ta::trend::sma_series;
//!
//! let close = Series::new("close".into(), &[1.0f64, 2.0, 3.0, 4.0, 5.0]);
//! let result = sma_series(&close, 3).unwrap();
//! // result: Series of length 3 = [2.0, 3.0, 4.0]
//! ```
//!
//! ## Output Convention
//!
//! Output Series are **shorter** than input Series by `lookback` elements,
//! matching ta-lib's C API. The caller is responsible for index alignment.
//!
//! ## Modules
//!
//! - [`trend`] — Moving averages and trend indicators
//! - [`oscillator`] — Momentum oscillators  
//! - [`volume`] — Volume-based indicators
//! - [`volatility`] — Volatility measures

pub mod oscillator;
pub mod trend;
pub mod volatility;
pub mod volume;
