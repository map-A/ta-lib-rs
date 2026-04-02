use polars_core::prelude::*;
use polars_ta_core::trend::sarext as sarext_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect())
}

/// Extended Parabolic SAR with separate acceleration parameters for long/short positions.
/// Output length = `n - 1`.
#[allow(clippy::too_many_arguments)]
pub fn sarext_series(
    high: &Series,
    low: &Series,
    start_value: f64,
    offset_on_reverse: f64,
    accel_init_long: f64,
    accel_long: f64,
    accel_max_long: f64,
    accel_init_short: f64,
    accel_short: f64,
    accel_max_short: f64,
) -> PolarsResult<Series> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let result = sarext_core(
        &h,
        &l,
        start_value,
        offset_on_reverse,
        accel_init_long,
        accel_long,
        accel_max_long,
        accel_init_short,
        accel_short,
        accel_max_short,
    );
    Ok(Float64Chunked::from_vec("sarext".into(), result).into_series())
}
