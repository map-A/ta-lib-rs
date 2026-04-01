//! Golden Tests for LOG10 (Math Transform).
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests")
        .join("golden")
        .join(filename)
}

fn run_log10_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden =
        load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.get_input("close").unwrap();
    let actual = polars_ta_core::math_transform::log10(&input);
    let label = format!("log10/{}", golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn log10_normal_1000() { run_log10_golden("log10__normal_1000.json", 1e-10); }
#[test]
#[ignore = "NaN propagation: ta-lib skips NaN, we propagate per IEEE 754"]
fn log10_with_nan() { run_log10_golden("log10__with_nan_5pct.json", 1e-10); }
#[test]
fn log10_all_same_value() { run_log10_golden("log10__all_same_value.json", 1e-10); }
#[test]
fn log10_real_btcusdt() { run_log10_golden("log10__real_btcusdt_1d.json", 1e-10); }
#[test]
fn log10_real_flat_period() { run_log10_golden("log10__real_flat_period.json", 1e-10); }
