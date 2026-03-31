use polars_core::prelude::*;
use polars_ta_core::volume::adosc as adosc_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?.f64()?.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect())
}

pub fn adosc_series(high: &Series, low: &Series, close: &Series, volume: &Series, fast_period: usize, slow_period: usize) -> PolarsResult<Series> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let c = series_to_f64(close)?;
    let v = series_to_f64(volume)?;
    let result = adosc_core(&h, &l, &c, &v, fast_period, slow_period);
    Ok(Float64Chunked::from_vec("adosc".into(), result).into_series())
}
