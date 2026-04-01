use polars_core::prelude::*;
use polars_ta_core::trend::ma as ma_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?.f64()?.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect())
}

/// Generic Moving Average dispatcher.
///
/// `matype` selects the MA algorithm: 0=SMA, 1=EMA, 2=WMA, 3=DEMA, 4=TEMA,
/// 5=TRIMA, 6=KAMA, 8=T3. Unknown values fall back to EMA.
pub fn ma_series(data: &Series, period: usize, matype: usize) -> PolarsResult<Series> {
    let d = series_to_f64(data)?;
    let result = ma_core(&d, period, matype);
    Ok(Float64Chunked::from_vec(data.name().clone(), result).into_series())
}
