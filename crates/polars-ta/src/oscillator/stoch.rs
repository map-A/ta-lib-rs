use polars_core::prelude::*;
use polars_ta_core::oscillator::stoch as stoch_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect())
}

pub struct StochSeriesOutput {
    pub slowk: Series,
    pub slowd: Series,
}

pub fn stoch_series(
    high: &Series,
    low: &Series,
    close: &Series,
    fastk_period: usize,
    slowk_period: usize,
    slowd_period: usize,
) -> PolarsResult<StochSeriesOutput> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let c = series_to_f64(close)?;
    let out = stoch_core(&h, &l, &c, fastk_period, slowk_period, slowd_period);
    Ok(StochSeriesOutput {
        slowk: Float64Chunked::from_vec("stoch_slowk".into(), out.slowk).into_series(),
        slowd: Float64Chunked::from_vec("stoch_slowd".into(), out.slowd).into_series(),
    })
}
