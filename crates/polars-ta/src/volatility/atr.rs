use polars_core::prelude::*;
use polars_ta_core::volatility::{atr as atr_core, natr as natr_core};

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect())
}

pub fn atr_series(
    high: &Series,
    low: &Series,
    close: &Series,
    period: usize,
) -> PolarsResult<Series> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let c = series_to_f64(close)?;
    let result = atr_core(&h, &l, &c, period);
    Ok(Float64Chunked::from_vec("atr".into(), result).into_series())
}

pub fn natr_series(
    high: &Series,
    low: &Series,
    close: &Series,
    period: usize,
) -> PolarsResult<Series> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let c = series_to_f64(close)?;
    let result = natr_core(&h, &l, &c, period);
    Ok(Float64Chunked::from_vec("natr".into(), result).into_series())
}
