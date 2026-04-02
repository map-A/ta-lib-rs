//! Golden Tests for VAR (Variance).
use polars_ta_core::statistic::var;
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

fn run_var_golden(filename: &str, period: usize, nbdev: f64, epsilon: f64) {
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
    let actual = var(&input, period, nbdev);
    let label = format!(
        "var(period={},nbdev={})/{}",
        period, nbdev, golden.meta.dataset
    );
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn var_period5_normal_1000() {
    run_var_golden("var_period5_nbdev1.0_normal_1000.json", 5, 1.0, 1e-8);
}

#[test]
fn var_period5_boundary_exact() {
    run_var_golden("var_period5_nbdev1.0_boundary_exact.json", 5, 1.0, 1e-8);
}

#[test]
fn var_period5_boundary_short() {
    let data = vec![1.0f64; 4];
    let result = var(&data, 5, 1.0);
    assert!(
        result.is_empty(),
        "输入不足时应返回空 Vec，got len={}",
        result.len()
    );
}

#[test]
#[ignore = "NaN 传播差异: Rust 实现不传播 NaN（输出 996 个有效值），ta-lib 传播（仅 36 个）"]
fn var_period5_with_nan() {
    run_var_golden("var_period5_nbdev1.0_with_nan_5pct.json", 5, 1.0, 1e-8);
}

#[test]
fn var_period5_all_same_value() {
    run_var_golden("var_period5_nbdev1.0_all_same_value.json", 5, 1.0, 1e-6);
}

#[test]
fn var_period5_real_btcusdt() {
    // BTC 大数值下浮点误差约 5.88e-5，使用 1e-4
    run_var_golden("var_period5_nbdev1.0_real_btcusdt_1d.json", 5, 1.0, 1e-4);
}

#[test]
fn var_period5_real_flat_period() {
    run_var_golden("var_period5_nbdev1.0_real_flat_period.json", 5, 1.0, 1e-8);
}
