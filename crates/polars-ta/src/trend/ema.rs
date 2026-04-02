use polars_core::prelude::*;
use polars_ta_core::trend::{
    dema as dema_core, ema as ema_core, tema as tema_core, wma as wma_core,
};

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect())
}

pub fn ema_series(close: &Series, period: usize) -> PolarsResult<Series> {
    let data = series_to_f64(close)?;
    let result = ema_core(&data, period);
    Ok(Float64Chunked::from_vec("ema".into(), result).into_series())
}

pub fn wma_series(close: &Series, period: usize) -> PolarsResult<Series> {
    let data = series_to_f64(close)?;
    let result = wma_core(&data, period);
    Ok(Float64Chunked::from_vec("wma".into(), result).into_series())
}

pub fn dema_series(close: &Series, period: usize) -> PolarsResult<Series> {
    let data = series_to_f64(close)?;
    let result = dema_core(&data, period);
    Ok(Float64Chunked::from_vec("dema".into(), result).into_series())
}

pub fn tema_series(close: &Series, period: usize) -> PolarsResult<Series> {
    let data = series_to_f64(close)?;
    let result = tema_core(&data, period);
    Ok(Float64Chunked::from_vec("tema".into(), result).into_series())
}
