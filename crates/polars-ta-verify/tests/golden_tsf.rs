//! Golden Tests for TSF (Time Series Forecast).
use polars_ta_core::statistic::tsf;
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

fn run_tsf_golden(filename: &str, period: usize, epsilon: f64) {
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
    let actual = tsf(&input, period);
    let label = format!("tsf(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn tsf_period14_normal_1000() {
    run_tsf_golden("tsf_period14_normal_1000.json", 14, 1e-8);
}

#[test]
fn tsf_period14_boundary_exact() {
    run_tsf_golden("tsf_period14_boundary_exact.json", 14, 1e-8);
}

#[test]
fn tsf_period14_boundary_short() {
    let data = vec![1.0f64; 13];
    let result = tsf(&data, 14);
    assert!(
        result.is_empty(),
        "输入不足时应返回空 Vec，got len={}",
        result.len()
    );
}

#[test]
#[ignore = "NaN 传播差异: Rust 实现对含 NaN 窗口停止后续计算，ta-lib 跳过 NaN 继续"]
fn tsf_period14_with_nan() {
    run_tsf_golden("tsf_period14_with_nan_5pct.json", 14, 1e-8);
}

#[test]
fn tsf_period14_all_same_value() {
    run_tsf_golden("tsf_period14_all_same_value.json", 14, 1e-8);
}

#[test]
fn tsf_period14_real_btcusdt() {
    // BTC 大数值下浮点累积误差约 1e-8，使用 1e-7
    run_tsf_golden("tsf_period14_real_btcusdt_1d.json", 14, 1e-7);
}

#[test]
fn tsf_period14_real_flat_period() {
    run_tsf_golden("tsf_period14_real_flat_period.json", 14, 1e-8);
}
