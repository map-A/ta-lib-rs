use polars_core::prelude::*;
use polars_ta_core::oscillator::aroon as aroon_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?.f64()?.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect())
}

pub struct AroonSeriesOutput {
    pub aroon_down: Series,
    pub aroon_up: Series,
}

pub fn aroon_series(high: &Series, low: &Series, period: usize) -> PolarsResult<AroonSeriesOutput> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let out = aroon_core(&h, &l, period);
    Ok(AroonSeriesOutput {
        aroon_down: Float64Chunked::from_vec("aroon_down".into(), out.aroon_down).into_series(),
        aroon_up: Float64Chunked::from_vec("aroon_up".into(), out.aroon_up).into_series(),
    })
}
