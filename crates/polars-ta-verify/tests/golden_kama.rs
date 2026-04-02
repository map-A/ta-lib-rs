//! Golden Tests for KAMA (Kaufman Adaptive Moving Average).
use polars_ta_core::trend::kama;
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

fn run_kama_golden(filename: &str, period: usize, epsilon: f64) {
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
    let actual = kama(&input, period);
    let label = format!("kama(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn kama_period10_normal_1000() {
    run_kama_golden("kama_period10_normal_1000.json", 10, 1e-8);
}
#[test]
fn kama_period10_boundary_exact() {
    run_kama_golden("kama_period10_boundary_exact.json", 10, 1e-8);
}
#[test]
fn kama_period10_boundary_short() {
    // n = period → empty
    let data = vec![1.0f64; 10];
    let result = kama(&data, 10);
    assert!(
        result.is_empty(),
        "输入不足时应返回空 Vec，got len={}",
        result.len()
    );
}
#[test]
fn kama_period10_with_nan() {
    run_kama_golden("kama_period10_with_nan_5pct.json", 10, 1e-8);
}
#[test]
fn kama_period10_all_same_value() {
    run_kama_golden("kama_period10_all_same_value.json", 10, 1e-8);
}
#[test]
fn kama_period10_real_btcusdt() {
    run_kama_golden("kama_period10_real_btcusdt_1d.json", 10, 1e-8);
}
#[test]
fn kama_period10_real_flat_period() {
    run_kama_golden("kama_period10_real_flat_period.json", 10, 1e-8);
}
