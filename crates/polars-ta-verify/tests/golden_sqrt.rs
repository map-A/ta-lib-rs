//! Golden Tests for SQRT (Math Transform).
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

fn run_sqrt_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden =
        load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.get_input("close").unwrap();
    let actual = polars_ta_core::math_transform::sqrt(&input);
    let label = format!("sqrt/{}", golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn sqrt_normal_1000() { run_sqrt_golden("sqrt__normal_1000.json", 1e-10); }
#[test]
#[ignore = "NaN propagation: ta-lib skips NaN, we propagate per IEEE 754"]
fn sqrt_with_nan() { run_sqrt_golden("sqrt__with_nan_5pct.json", 1e-10); }
#[test]
fn sqrt_all_same_value() { run_sqrt_golden("sqrt__all_same_value.json", 1e-10); }
#[test]
fn sqrt_real_btcusdt() { run_sqrt_golden("sqrt__real_btcusdt_1d.json", 1e-10); }
#[test]
fn sqrt_real_flat_period() { run_sqrt_golden("sqrt__real_flat_period.json", 1e-10); }
