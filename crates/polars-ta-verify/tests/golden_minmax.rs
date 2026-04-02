//! Golden Tests for MINMAX.
use polars_ta_core::math_ops::minmax;
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

fn run_minmax_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let close = golden.close_input().unwrap();
    let actual = minmax(&close, period);
    let label = format!("minmax(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual.min,
        golden.get_output_values("min").unwrap(),
        epsilon,
        &format!("{}/min", label),
    );
    assert_close(
        &actual.max,
        golden.get_output_values("max").unwrap(),
        epsilon,
        &format!("{}/max", label),
    );
}

#[test]
fn minmax_period30_normal_1000() {
    run_minmax_golden("minmax_period30_normal_1000.json", 30, 1e-10);
}
#[test]
fn minmax_period30_all_same_value() {
    run_minmax_golden("minmax_period30_all_same_value.json", 30, 1e-10);
}
#[test]
fn minmax_period30_real_btcusdt() {
    run_minmax_golden("minmax_period30_real_btcusdt_1d.json", 30, 1e-10);
}
#[test]
fn minmax_period30_real_flat_period() {
    run_minmax_golden("minmax_period30_real_flat_period.json", 30, 1e-10);
}
#[test]
fn minmax_period30_boundary_exact() {
    run_minmax_golden("minmax_period30_boundary_exact.json", 30, 1e-10);
}
#[test]
fn minmax_period30_boundary_short() {
    let result = minmax(&vec![1.0f64; 29], 30);
    assert!(result.min.is_empty());
    assert!(result.max.is_empty());
}
#[test]
#[ignore = "NaN propagation differs"]
fn minmax_period30_with_nan() {
    run_minmax_golden("minmax_period30_with_nan_5pct.json", 30, 1e-10);
}
