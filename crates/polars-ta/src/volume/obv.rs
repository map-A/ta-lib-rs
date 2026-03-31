use polars_core::prelude::*;
use polars_ta_core::volume::obv as obv_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?.f64()?.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect())
}

pub fn obv_series(close: &Series, volume: &Series) -> PolarsResult<Series> {
    let c = series_to_f64(close)?;
    let v = series_to_f64(volume)?;
    let result = obv_core(&c, &v);
    Ok(Float64Chunked::from_vec("obv".into(), result).into_series())
}
