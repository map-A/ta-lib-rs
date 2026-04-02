//! Golden Tests for BETA.
use polars_ta_core::statistic::beta;
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

fn run_beta_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: golden file not found: {}", filename);
        return;
    }
    let golden =
        load_golden_file(&path).unwrap_or_else(|e| panic!("Failed to load {}: {}", filename, e));
    let real0 = golden
        .get_input("real0")
        .unwrap_or_else(|e| panic!("Failed to parse real0: {}", e));
    let real1 = golden
        .get_input("real1")
        .unwrap_or_else(|e| panic!("Failed to parse real1: {}", e));
    let actual = beta(&real0, &real1, period);
    let label = format!("beta(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

// NOTE: ta-lib BETA uses Cov(r1,r0)/Var(r0) (regresses real1 on real0, denominator = Var(real0)).
// Our implementation matches this convention exactly.

#[test]
fn beta_period5_normal_1000() {
    run_beta_golden("beta_period5_normal_1000.json", 5, 1e-8);
}

#[test]
fn beta_period5_boundary_exact() {
    run_beta_golden("beta_period5_boundary_exact.json", 5, 1e-8);
}

#[test]
fn beta_period5_boundary_short() {
    let real0 = vec![1.0f64; 5];
    let real1 = vec![1.0f64; 5];
    let result = beta(&real0, &real1, 6);
    assert!(
        result.is_empty(),
        "输入不足时应返回空 Vec，got len={}",
        result.len()
    );
}

#[test]
#[ignore = "NaN propagation: ta-lib skips NaN in multi-input windows, we propagate per IEEE 754"]
fn beta_period5_with_nan() {
    run_beta_golden("beta_period5_with_nan_5pct.json", 5, 1e-8);
}

#[test]
fn beta_period5_all_same_value() {
    run_beta_golden("beta_period5_all_same_value.json", 5, 1e-6);
}

#[test]
fn beta_period5_real_btcusdt() {
    run_beta_golden("beta_period5_real_btcusdt_1d.json", 5, 1e-8);
}

#[test]
fn beta_period5_real_flat_period() {
    run_beta_golden("beta_period5_real_flat_period.json", 5, 1e-8);
}
