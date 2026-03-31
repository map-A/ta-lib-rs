//! Statistical indicators: regression, correlation, variance, and forecast functions.
//!
//! All functions operate on rolling windows of fixed size over `&[f64]` slices.
//! Numerically compatible with ta-lib C (population/sample variance as per ta-lib behavior).

pub mod beta;
pub mod correl;
pub mod linearreg;
pub mod linearreg_angle;
pub mod linearreg_intercept;
pub mod linearreg_slope;
pub mod stddev;
pub mod tsf;
pub mod var;

pub use beta::beta;
pub use correl::correl;
pub use linearreg::linearreg;
pub use linearreg_angle::linearreg_angle;
pub use linearreg_intercept::linearreg_intercept;
pub use linearreg_slope::linearreg_slope;
pub use stddev::stddev;
pub use tsf::tsf;
pub use var::var;
