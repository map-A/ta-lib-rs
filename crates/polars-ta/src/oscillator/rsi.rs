use polars_core::prelude::*;
use polars_ta_core::oscillator::rsi as rsi_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect())
}

pub fn rsi_series(close: &Series, period: usize) -> PolarsResult<Series> {
    let data = series_to_f64(close)?;
    let result = rsi_core(&data, period);
    Ok(Float64Chunked::from_vec("rsi".into(), result).into_series())
}
