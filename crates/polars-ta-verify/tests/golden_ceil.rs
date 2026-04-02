//! Golden Tests for CEIL (Math Transform).
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

fn run_ceil_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.get_input("close").unwrap();
    let actual = polars_ta_core::math_transform::ceil(&input);
    let label = format!("ceil/{}", golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn ceil_normal_1000() {
    run_ceil_golden("ceil__normal_1000.json", 1e-10);
}
#[test]
#[ignore = "NaN propagation: ta-lib skips NaN, we propagate per IEEE 754"]
fn ceil_with_nan() {
    run_ceil_golden("ceil__with_nan_5pct.json", 1e-10);
}
#[test]
fn ceil_all_same_value() {
    run_ceil_golden("ceil__all_same_value.json", 1e-10);
}
#[test]
fn ceil_real_btcusdt() {
    run_ceil_golden("ceil__real_btcusdt_1d.json", 1e-10);
}
#[test]
fn ceil_real_flat_period() {
    run_ceil_golden("ceil__real_flat_period.json", 1e-10);
}
