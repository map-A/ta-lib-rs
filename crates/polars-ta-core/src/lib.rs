//! # polars-ta-core
//!
//! Pure algorithm layer for `polars-ta`. All functions operate on `&[f64]` slices
//! and have **zero external dependencies**, making this crate suitable for
//! embedded systems, WASM, and `no_std` environments.
//!
//! ## Output Convention
//!
//! Output arrays are **shorter** than input arrays by `lookback` elements,
//! matching ta-lib's C API behavior exactly:
//!
//! ```text
//! Input:  [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9]  len=10
//! SMA(3): lookback=2
//! Output:       [v2', v3', v4', v5', v6', v7', v8', v9']  len=8
//! ```
//!
//! The **caller** is responsible for aligning output indices with the original input.
//!
//! ## Error Handling
//!
//! - Input length < `lookback + 1`: returns an empty `Vec` (never panics)
//! - NaN inputs: once a NaN enters the sliding sum, **all subsequent outputs
//!   are NaN** (IEEE 754 arithmetic, identical to ta-lib C behavior)
//! - All-NaN inputs: returns all-NaN output of correct length
//!
//! ## Modules
//!
//! - [`trend`] — SMA, EMA, WMA, DEMA, TEMA, MACD, BBands, SAR, ADX
//! - [`oscillator`] — RSI, Stoch, StochRSI, CCI, Williams %R, ULTOSC, Aroon, MFI
//! - [`volume`] — OBV, AD, ADOSC
//! - [`volatility`] — ATR, NATR, TRange

pub mod oscillator;
pub mod statistic;
pub mod trend;
pub mod volatility;
pub mod volume;
