use polars_core::prelude::*;
use polars_ta_core::oscillator::aroonosc as aroonosc_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?.f64()?.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect())
}

/// Aroon Oscillator (Aroon Up − Aroon Down). Output length = `n - period`.
pub fn aroonosc_series(high: &Series, low: &Series, period: usize) -> PolarsResult<Series> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let result = aroonosc_core(&h, &l, period);
    Ok(Float64Chunked::from_vec("aroonosc".into(), result).into_series())
}
