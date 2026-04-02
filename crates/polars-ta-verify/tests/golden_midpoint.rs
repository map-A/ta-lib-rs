//! Golden Tests for MIDPOINT (Midpoint Over Period).
use polars_ta_core::trend::midpoint;
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

fn run_midpoint_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: golden file not found: {}", filename);
        return;
    }
    let golden =
        load_golden_file(&path).unwrap_or_else(|e| panic!("Failed to load {}: {}", filename, e));
    let input = golden
        .close_input()
        .unwrap_or_else(|e| panic!("Failed to parse input: {}", e));
    let actual = midpoint(&input, period);
    let label = format!("midpoint(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn midpoint_period14_normal_1000() {
    run_midpoint_golden("midpoint_period14_normal_1000.json", 14, 1e-10);
}
#[test]
fn midpoint_period14_boundary_exact() {
    run_midpoint_golden("midpoint_period14_boundary_exact.json", 14, 1e-10);
}
#[test]
fn midpoint_period14_boundary_short() {
    let data = vec![1.0f64; 13];
    let result = midpoint(&data, 14);
    assert!(
        result.is_empty(),
        "输入不足时应返回空 Vec，got len={}",
        result.len()
    );
}
#[test]
#[ignore = "NaN in sliding-window min/max: ta-lib skips NaN, we propagate per IEEE 754"]
#[test]
fn midpoint_period14_with_nan() {
    run_midpoint_golden("midpoint_period14_with_nan_5pct.json", 14, 1e-10);
}
#[test]
fn midpoint_period14_all_same_value() {
    run_midpoint_golden("midpoint_period14_all_same_value.json", 14, 1e-10);
}
#[test]
fn midpoint_period14_real_btcusdt() {
    run_midpoint_golden("midpoint_period14_real_btcusdt_1d.json", 14, 1e-10);
}
#[test]
fn midpoint_period14_real_flat_period() {
    run_midpoint_golden("midpoint_period14_real_flat_period.json", 14, 1e-10);
}
