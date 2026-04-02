use polars_core::prelude::*;
use polars_ta_core::trend::macdext as macdext_core;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect())
}

pub struct MacdExtSeriesOutput {
    pub macd: Series,
    pub signal: Series,
    pub hist: Series,
}

/// MACD with configurable MA types for each stage.
///
/// `fast_matype`, `slow_matype`, `signal_matype` follow the same encoding as [`ma_series`].
#[allow(clippy::too_many_arguments)]
pub fn macdext_series(
    close: &Series,
    fast_period: usize,
    fast_matype: usize,
    slow_period: usize,
    slow_matype: usize,
    signal_period: usize,
    signal_matype: usize,
) -> PolarsResult<MacdExtSeriesOutput> {
    let data = series_to_f64(close)?;
    let out = macdext_core(
        &data,
        fast_period,
        fast_matype,
        slow_period,
        slow_matype,
        signal_period,
        signal_matype,
    );
    Ok(MacdExtSeriesOutput {
        macd: Float64Chunked::from_vec("macdext".into(), out.macd).into_series(),
        signal: Float64Chunked::from_vec("macdext_signal".into(), out.signal).into_series(),
        hist: Float64Chunked::from_vec("macdext_hist".into(), out.hist).into_series(),
    })
}
