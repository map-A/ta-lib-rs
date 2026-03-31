//! Simple Moving Average — Polars Series interface.
//!
//! Wraps [`polars_ta_core::trend::sma`] for ergonomic use with Polars DataFrames.
//!
//! # Example
//!
//! ```rust,no_run
//! use polars::prelude::*;
//! use polars_ta::trend::sma_series;
//!
//! let close = Series::new("close".into(), &[1.0f64, 2.0, 3.0, 4.0, 5.0]);
//! let sma20 = sma_series(&close, 3).unwrap();
//! // Output length = 5 - (3 - 1) = 3
//! assert_eq!(sma20.len(), 3);
//! ```

use polars_core::prelude::*;
use polars_ta_core::trend::sma as sma_core;

/// Compute Simple Moving Average on a Polars `Series`.
///
/// # Arguments
///
/// - `series` — input price series (Float64); NaN values are propagated
/// - `period` — averaging window size (≥ 1)
///
/// # Returns
///
/// A `Series` of length `series.len() - (period - 1)`.
/// Returns an empty `Series` when `series.len() < period`.
///
/// # Errors
///
/// Returns `PolarsError` if the series cannot be cast to `Float64`.
pub fn sma_series(series: &Series, period: usize) -> PolarsResult<Series> {
    let ca = series.cast(&DataType::Float64)?;
    let ca = ca.f64()?;

    let data: Vec<f64> = ca.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect();
    let result = sma_core(&data, period);

    Ok(Series::new(series.name().clone(), result))
}
