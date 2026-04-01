//! Math operator indicators: arithmetic operations and rolling window statistics
//! (wraps `polars_ta_core::math_ops`).

use polars_core::prelude::*;
use polars_ta_core::math_ops;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect())
}

fn vec_to_series(name: PlSmallStr, v: Vec<f64>) -> Series {
    Float64Chunked::from_vec(name, v).into_series()
}

// ── Two-series arithmetic ────────────────────────────────────────────────────

/// Element-wise addition: `real0[i] + real1[i]`.
pub fn add_series(real0: &Series, real1: &Series) -> PolarsResult<Series> {
    let a = series_to_f64(real0)?;
    let b = series_to_f64(real1)?;
    Ok(vec_to_series(real0.name().clone(), math_ops::add(&a, &b)))
}

/// Element-wise division: `real0[i] / real1[i]`.
pub fn div_series(real0: &Series, real1: &Series) -> PolarsResult<Series> {
    let a = series_to_f64(real0)?;
    let b = series_to_f64(real1)?;
    Ok(vec_to_series(real0.name().clone(), math_ops::div(&a, &b)))
}

/// Element-wise multiplication: `real0[i] * real1[i]`.
pub fn mult_series(real0: &Series, real1: &Series) -> PolarsResult<Series> {
    let a = series_to_f64(real0)?;
    let b = series_to_f64(real1)?;
    Ok(vec_to_series(real0.name().clone(), math_ops::mult(&a, &b)))
}

/// Element-wise subtraction: `real0[i] - real1[i]`.
pub fn sub_series(real0: &Series, real1: &Series) -> PolarsResult<Series> {
    let a = series_to_f64(real0)?;
    let b = series_to_f64(real1)?;
    Ok(vec_to_series(real0.name().clone(), math_ops::sub(&a, &b)))
}

// ── Rolling window — single output ──────────────────────────────────────────

/// Rolling maximum over `period` bars. Output length = `n - period + 1`.
pub fn max_series(s: &Series, period: usize) -> PolarsResult<Series> {
    let data = series_to_f64(s)?;
    Ok(vec_to_series(s.name().clone(), math_ops::max(&data, period)))
}

/// Rolling minimum over `period` bars. Output length = `n - period + 1`.
pub fn min_series(s: &Series, period: usize) -> PolarsResult<Series> {
    let data = series_to_f64(s)?;
    Ok(vec_to_series(s.name().clone(), math_ops::min(&data, period)))
}

/// Rolling sum over `period` bars. Output length = `n - period + 1`.
pub fn sum_series(s: &Series, period: usize) -> PolarsResult<Series> {
    let data = series_to_f64(s)?;
    Ok(vec_to_series(s.name().clone(), math_ops::sum(&data, period)))
}

/// Index of rolling maximum over `period` bars (0-based). Output length = `n - period + 1`.
pub fn maxindex_series(s: &Series, period: usize) -> PolarsResult<Series> {
    let data = series_to_f64(s)?;
    Ok(vec_to_series(s.name().clone(), math_ops::maxindex(&data, period)))
}

/// Index of rolling minimum over `period` bars (0-based). Output length = `n - period + 1`.
pub fn minindex_series(s: &Series, period: usize) -> PolarsResult<Series> {
    let data = series_to_f64(s)?;
    Ok(vec_to_series(s.name().clone(), math_ops::minindex(&data, period)))
}

// ── Rolling window — dual output ─────────────────────────────────────────────

/// Output of [`minmax_series`]: rolling minimum and maximum.
pub struct MinMaxSeriesOutput {
    pub min: Series,
    pub max: Series,
}

/// Rolling minimum and maximum simultaneously. Output length = `n - period + 1`.
pub fn minmax_series(s: &Series, period: usize) -> PolarsResult<MinMaxSeriesOutput> {
    let data = series_to_f64(s)?;
    let out = math_ops::minmax(&data, period);
    Ok(MinMaxSeriesOutput {
        min: vec_to_series(format!("{}_min", s.name()).into(), out.min),
        max: vec_to_series(format!("{}_max", s.name()).into(), out.max),
    })
}

/// Output of [`minmaxindex_series`]: indices of rolling minimum and maximum.
pub struct MinMaxIndexSeriesOutput {
    pub minidx: Series,
    pub maxidx: Series,
}

/// Indices of rolling minimum and maximum. Output length = `n - period + 1`.
pub fn minmaxindex_series(s: &Series, period: usize) -> PolarsResult<MinMaxIndexSeriesOutput> {
    let data = series_to_f64(s)?;
    let out = math_ops::minmaxindex(&data, period);
    Ok(MinMaxIndexSeriesOutput {
        minidx: vec_to_series(format!("{}_minidx", s.name()).into(), out.minidx),
        maxidx: vec_to_series(format!("{}_maxidx", s.name()).into(), out.maxidx),
    })
}
