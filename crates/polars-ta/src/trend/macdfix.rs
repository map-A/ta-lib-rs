use polars_core::prelude::*;
use polars_ta_core::trend::macdfix as macdfix_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect())
}

pub struct MacdFixSeriesOutput {
    pub macd: Series,
    pub signal: Series,
    pub hist: Series,
}

/// MACD with fixed fast=12 / slow=26 periods and fixed EMA multipliers.
/// Output length = `n - (26 + signal_period - 2)`.
pub fn macdfix_series(close: &Series, signal_period: usize) -> PolarsResult<MacdFixSeriesOutput> {
    let data = series_to_f64(close)?;
    let out = macdfix_core(&data, signal_period);
    Ok(MacdFixSeriesOutput {
        macd: Float64Chunked::from_vec("macdfix".into(), out.macd).into_series(),
        signal: Float64Chunked::from_vec("macdfix_signal".into(), out.signal).into_series(),
        hist: Float64Chunked::from_vec("macdfix_hist".into(), out.hist).into_series(),
    })
}
