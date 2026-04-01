//! Golden Tests for T3 (Triple Exponential Moving Average with Volume Factor).
use polars_ta_core::trend::t3;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_t3_golden(filename: &str, period: usize, vf: f64, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: golden file not found: {}", filename);
        return;
    }
    let golden = load_golden_file(&path)
        .unwrap_or_else(|e| panic!("Failed to load {}: {}", filename, e));
    let input = golden.close_input()
        .unwrap_or_else(|e| panic!("Failed to parse input: {}", e));
    let actual = t3(&input, period, vf);
    let label = format!("t3(period={},vf={})/{}", period, vf, golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn t3_period5_normal_1000() { run_t3_golden("t3_period5_vfactor0.7_normal_1000.json", 5, 0.7, 1e-8); }
#[test]
fn t3_period5_boundary_exact() { run_t3_golden("t3_period5_vfactor0.7_boundary_exact.json", 5, 0.7, 1e-8); }
#[test]
fn t3_period5_boundary_short() {
    // lookback = 24, n = 24 → empty
    let data = vec![1.0f64; 24];
    let result = t3(&data, 5, 0.7);
    assert!(result.is_empty(), "输入不足时应返回空 Vec，got len={}", result.len());
}
#[test]
fn t3_period5_with_nan() { run_t3_golden("t3_period5_vfactor0.7_with_nan_5pct.json", 5, 0.7, 1e-8); }
#[test]
fn t3_period5_all_same_value() { run_t3_golden("t3_period5_vfactor0.7_all_same_value.json", 5, 0.7, 1e-8); }
#[test]
fn t3_period5_real_btcusdt() { run_t3_golden("t3_period5_vfactor0.7_real_btcusdt_1d.json", 5, 0.7, 1e-8); }
#[test]
fn t3_period5_real_flat_period() { run_t3_golden("t3_period5_vfactor0.7_real_flat_period.json", 5, 0.7, 1e-8); }
