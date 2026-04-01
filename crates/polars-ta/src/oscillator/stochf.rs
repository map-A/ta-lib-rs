use polars_core::prelude::*;
use polars_ta_core::oscillator::stochf as stochf_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?.f64()?.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect())
}

pub struct StochFSeriesOutput {
    pub fastk: Series,
    pub fastd: Series,
}

/// Fast Stochastic Oscillator.
/// Output length = `n - (fastk_period + fastd_period - 2)`.
pub fn stochf_series(
    high: &Series,
    low: &Series,
    close: &Series,
    fastk_period: usize,
    fastd_period: usize,
) -> PolarsResult<StochFSeriesOutput> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let c = series_to_f64(close)?;
    let out = stochf_core(&h, &l, &c, fastk_period, fastd_period);
    Ok(StochFSeriesOutput {
        fastk: Float64Chunked::from_vec("stochf_fastk".into(), out.fastk).into_series(),
        fastd: Float64Chunked::from_vec("stochf_fastd".into(), out.fastd).into_series(),
    })
}
