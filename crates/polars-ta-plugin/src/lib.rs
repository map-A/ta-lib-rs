//! # polars-ta-plugin
//!
//! Polars Python plugin exposing `polars-ta` indicators as native Polars expressions.
//!
//! ## Usage (Python)
//!
//! ```python
//! import polars as pl
//! import polars_ta_plugin as ta
//!
//! df = pl.DataFrame({"close": [1.0, 2.0, 3.0, 4.0, 5.0]})
//! result = df.select(ta.sma("close", period=3))
//! ```

use pyo3::prelude::*;

// Plugin 实现将在 Phase 3 完善
// 当前仅提供骨架，确保 cdylib 可编译

#[pymodule]
fn polars_ta_plugin(_py: Python<'_>, _m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
