//! Price transform indicators (wraps `polars_ta_core::price_transform`).
//! All functions have lookback = 0, so output length equals input length.

use polars_core::prelude::*;
use polars_ta_core::price_transform;

fn series_to_f64(s: &Series) -> PolarsResult<Vec<f64>> {
    Ok(s.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect())
}

fn vec_to_series(name: PlSmallStr, v: Vec<f64>) -> Series {
    Float64Chunked::from_vec(name, v).into_series()
}

/// Average Price: `(open + high + low + close) / 4`.
pub fn avgprice_series(
    open: &Series,
    high: &Series,
    low: &Series,
    close: &Series,
) -> PolarsResult<Series> {
    let o = series_to_f64(open)?;
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let c = series_to_f64(close)?;
    Ok(vec_to_series(
        "avgprice".into(),
        price_transform::avgprice(&o, &h, &l, &c),
    ))
}

/// Median Price: `(high + low) / 2`.
pub fn medprice_series(high: &Series, low: &Series) -> PolarsResult<Series> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    Ok(vec_to_series(
        "medprice".into(),
        price_transform::medprice(&h, &l),
    ))
}

/// Typical Price: `(high + low + close) / 3`.
pub fn typprice_series(high: &Series, low: &Series, close: &Series) -> PolarsResult<Series> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let c = series_to_f64(close)?;
    Ok(vec_to_series(
        "typprice".into(),
        price_transform::typprice(&h, &l, &c),
    ))
}

/// Weighted Close Price: `(high + low + close * 2) / 4`.
pub fn wclprice_series(high: &Series, low: &Series, close: &Series) -> PolarsResult<Series> {
    let h = series_to_f64(high)?;
    let l = series_to_f64(low)?;
    let c = series_to_f64(close)?;
    Ok(vec_to_series(
        "wclprice".into(),
        price_transform::wclprice(&h, &l, &c),
    ))
}
