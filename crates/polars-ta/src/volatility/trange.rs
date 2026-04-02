use polars_core::prelude::*;
use polars_ta_core::volatility::trange as trange_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect())
}

pub fn trange_series(high: &Series, low: &Series, close: &Series) -> PolarsResult<Series> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let c = series_to_f64(close)?;
    let result = trange_core(&h, &l, &c);
    Ok(Float64Chunked::from_vec("trange".into(), result).into_series())
}
