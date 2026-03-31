use polars_core::prelude::*;
use polars_ta_core::oscillator::stochrsi as stochrsi_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?.f64()?.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect())
}

pub struct StochRsiSeriesOutput {
    pub fastk: Series,
    pub fastd: Series,
}

pub fn stochrsi_series(close: &Series, period: usize, fastk_period: usize, fastd_period: usize) -> PolarsResult<StochRsiSeriesOutput> {
    let data = series_to_f64(close)?;
    let out = stochrsi_core(&data, period, fastk_period, fastd_period);
    Ok(StochRsiSeriesOutput {
        fastk: Float64Chunked::from_vec("stochrsi_fastk".into(), out.fastk).into_series(),
        fastd: Float64Chunked::from_vec("stochrsi_fastd".into(), out.fastd).into_series(),
    })
}
