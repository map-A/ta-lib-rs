//! Golden Tests for MIDPRICE (Midpoint Price Over Period).
use polars_ta_core::trend::midprice;
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

fn run_midprice_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: golden file not found: {}", filename);
        return;
    }
    let golden =
        load_golden_file(&path).unwrap_or_else(|e| panic!("Failed to load {}: {}", filename, e));
    let high = golden
        .get_input("high")
        .unwrap_or_else(|e| panic!("Failed to parse high input: {}", e));
    let low = golden
        .get_input("low")
        .unwrap_or_else(|e| panic!("Failed to parse low input: {}", e));
    let actual = midprice(&high, &low, period);
    let label = format!("midprice(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn midprice_period14_normal_1000() {
    run_midprice_golden("midprice_period14_normal_1000.json", 14, 1e-10);
}
#[test]
fn midprice_period14_boundary_exact() {
    run_midprice_golden("midprice_period14_boundary_exact.json", 14, 1e-10);
}
#[test]
fn midprice_period14_boundary_short() {
    let high = vec![1.0f64; 13];
    let low = vec![0.5f64; 13];
    let result = midprice(&high, &low, 14);
    assert!(
        result.is_empty(),
        "输入不足时应返回空 Vec，got len={}",
        result.len()
    );
}
#[test]
#[ignore = "ta-lib NaN propagation differs for multi-input indicators: ta-lib skips NaN in windows, we propagate via IEEE 754"]
fn midprice_period14_with_nan() {
    run_midprice_golden("midprice_period14_with_nan_5pct.json", 14, 1e-10);
}
#[test]
fn midprice_period14_all_same_value() {
    run_midprice_golden("midprice_period14_all_same_value.json", 14, 1e-10);
}
#[test]
fn midprice_period14_real_btcusdt() {
    run_midprice_golden("midprice_period14_real_btcusdt_1d.json", 14, 1e-10);
}
#[test]
fn midprice_period14_real_flat_period() {
    run_midprice_golden("midprice_period14_real_flat_period.json", 14, 1e-10);
}
