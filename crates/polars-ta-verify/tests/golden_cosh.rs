//! Golden Tests for COSH (Math Transform).
use polars_ta_verify::golden::{assert_close_relative, load_golden_file};
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

fn run_cosh_golden(filename: &str) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.get_input("close").unwrap();
    let actual = polars_ta_core::math_transform::cosh(&input);
    let label = format!("cosh/{}", golden.meta.dataset);
    // COSH of large inputs produces astronomically large values; use relative epsilon
    assert_close_relative(
        &actual,
        golden.get_output_values("values").unwrap(),
        1.0,
        1e-10,
        &label,
    );
}

#[test]
fn cosh_normal_1000() {
    run_cosh_golden("cosh__normal_1000.json");
}
#[test]
#[ignore = "NaN propagation: ta-lib skips NaN, we propagate per IEEE 754"]
fn cosh_with_nan() {
    run_cosh_golden("cosh__with_nan_5pct.json");
}
#[test]
fn cosh_all_same_value() {
    run_cosh_golden("cosh__all_same_value.json");
}
#[test]
fn cosh_real_btcusdt() {
    run_cosh_golden("cosh__real_btcusdt_1d.json");
}
#[test]
fn cosh_real_flat_period() {
    run_cosh_golden("cosh__real_flat_period.json");
}
