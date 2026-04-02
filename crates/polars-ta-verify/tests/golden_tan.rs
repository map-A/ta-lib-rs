//! Golden Tests for TAN (Math Transform).
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

fn run_tan_golden(filename: &str) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.get_input("close").unwrap();
    let actual = polars_ta_core::math_transform::tan(&input);
    let label = format!("tan/{}", golden.meta.dataset);
    // TAN near singularities produces very large values; use relative epsilon
    assert_close_relative(
        &actual,
        golden.get_output_values("values").unwrap(),
        1e-8,
        1e-10,
        &label,
    );
}

#[test]
fn tan_normal_1000() {
    run_tan_golden("tan__normal_1000.json");
}
#[test]
#[ignore = "NaN propagation: ta-lib skips NaN, we propagate per IEEE 754"]
fn tan_with_nan() {
    run_tan_golden("tan__with_nan_5pct.json");
}
#[test]
fn tan_all_same_value() {
    run_tan_golden("tan__all_same_value.json");
}
#[test]
fn tan_real_btcusdt() {
    run_tan_golden("tan__real_btcusdt_1d.json");
}
#[test]
fn tan_real_flat_period() {
    run_tan_golden("tan__real_flat_period.json");
}
