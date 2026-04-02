use polars_core::prelude::*;
use polars_ta_core::trend::macd as macd_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect())
}

pub struct MacdSeriesOutput {
    pub macd: Series,
    pub signal: Series,
    pub hist: Series,
}

pub fn macd_series(
    close: &Series,
    fast: usize,
    slow: usize,
    signal: usize,
) -> PolarsResult<MacdSeriesOutput> {
    let data = series_to_f64(close)?;
    let out = macd_core(&data, fast, slow, signal);
    Ok(MacdSeriesOutput {
        macd: Float64Chunked::from_vec("macd".into(), out.macd).into_series(),
        signal: Float64Chunked::from_vec("macd_signal".into(), out.signal).into_series(),
        hist: Float64Chunked::from_vec("macd_hist".into(), out.hist).into_series(),
    })
}
