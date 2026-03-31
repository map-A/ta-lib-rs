use polars_core::prelude::*;
use polars_ta_core::trend::sar as sar_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?.f64()?.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect())
}

pub fn sar_series(high: &Series, low: &Series, acceleration: f64, maximum: f64) -> PolarsResult<Series> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let result = sar_core(&h, &l, acceleration, maximum);
    Ok(Float64Chunked::from_vec("sar".into(), result).into_series())
}
