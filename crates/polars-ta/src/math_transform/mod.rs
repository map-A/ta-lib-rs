//! Element-wise mathematical transformations (wraps `polars_ta_core::math_transform`).
//! All functions have lookback = 0, so output length equals input length.

use polars_core::prelude::*;
use polars_ta_core::math_transform;

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

macro_rules! transform_fn {
    ($fn_name:ident, $core_fn:path, $doc:literal) => {
        #[doc = $doc]
        pub fn $fn_name(s: &Series) -> PolarsResult<Series> {
            let data = series_to_f64(s)?;
            Ok(vec_to_series(s.name().clone(), $core_fn(&data)))
        }
    };
}

transform_fn!(
    acos_series,
    math_transform::acos,
    "Arc-cosine of each element (radians)."
);
transform_fn!(
    asin_series,
    math_transform::asin,
    "Arc-sine of each element (radians)."
);
transform_fn!(
    atan_series,
    math_transform::atan,
    "Arc-tangent of each element (radians)."
);
transform_fn!(
    ceil_series,
    math_transform::ceil,
    "Ceiling (round toward +∞) of each element."
);
transform_fn!(
    cos_series,
    math_transform::cos,
    "Cosine of each element (radians input)."
);
transform_fn!(
    cosh_series,
    math_transform::cosh,
    "Hyperbolic cosine of each element."
);
transform_fn!(
    exp_series,
    math_transform::exp,
    "e raised to the power of each element."
);
transform_fn!(
    floor_series,
    math_transform::floor,
    "Floor (round toward −∞) of each element."
);
transform_fn!(
    ln_series,
    math_transform::ln,
    "Natural logarithm of each element."
);
transform_fn!(
    log10_series,
    math_transform::log10,
    "Base-10 logarithm of each element."
);
transform_fn!(
    sin_series,
    math_transform::sin,
    "Sine of each element (radians input)."
);
transform_fn!(
    sinh_series,
    math_transform::sinh,
    "Hyperbolic sine of each element."
);
transform_fn!(
    sqrt_series,
    math_transform::sqrt,
    "Square root of each element."
);
transform_fn!(
    tan_series,
    math_transform::tan,
    "Tangent of each element (radians input)."
);
transform_fn!(
    tanh_series,
    math_transform::tanh,
    "Hyperbolic tangent of each element."
);
