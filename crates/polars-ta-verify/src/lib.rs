//! # polars-ta-verify
//!
//! Golden Test and benchmark framework for `polars-ta`.
//!
//! ## Overview
//!
//! - [`golden`] — Load golden JSON files and verify numerical parity with ta-lib
//! - [`bench`] — Benchmark utilities for comparing performance with ta-lib C

pub mod bench;
pub mod golden;
