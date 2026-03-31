use polars_core::prelude::*;
use polars_ta_core::trend::bbands as bbands_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?.f64()?.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect())
}

pub struct BbandsSeriesOutput {
    pub upper: Series,
    pub middle: Series,
    pub lower: Series,
}

pub fn bbands_series(close: &Series, period: usize, nbdevup: f64, nbdevdn: f64) -> PolarsResult<BbandsSeriesOutput> {
    let data = series_to_f64(close)?;
    let out = bbands_core(&data, period, nbdevup, nbdevdn);
    Ok(BbandsSeriesOutput {
        upper: Float64Chunked::from_vec("bb_upper".into(), out.upper).into_series(),
        middle: Float64Chunked::from_vec("bb_middle".into(), out.middle).into_series(),
        lower: Float64Chunked::from_vec("bb_lower".into(), out.lower).into_series(),
    })
}
