//! Golden Tests for WMA (Weighted Moving Average).
use polars_ta_core::trend::wma;
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

fn run_wma_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.close_input().unwrap();
    let actual = wma(&input, period);
    let label = format!("wma(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn wma_period20_normal_1000() {
    run_wma_golden("wma_period20_normal_1000.json", 20, 1e-10);
}
#[test]
fn wma_period20_boundary_exact() {
    run_wma_golden("wma_period20_boundary_exact.json", 20, 1e-10);
}
#[test]
fn wma_period20_boundary_short() {
    let data = vec![1.0f64; 19];
    assert!(wma(&data, 20).is_empty());
}
#[test]
fn wma_period20_with_nan() {
    run_wma_golden("wma_period20_with_nan_5pct.json", 20, 1e-10);
}
#[test]
fn wma_period20_all_same_value() {
    run_wma_golden("wma_period20_all_same_value.json", 20, 1e-10);
}
#[test]
fn wma_period20_real_btcusdt() {
    run_wma_golden("wma_period20_real_btcusdt_1d.json", 20, 1e-7);
}
#[test]
fn wma_period20_real_flat_period() {
    run_wma_golden("wma_period20_real_flat_period.json", 20, 1e-10);
}
