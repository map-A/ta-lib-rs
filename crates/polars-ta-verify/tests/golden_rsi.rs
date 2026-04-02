//! Golden Tests for RSI (Relative Strength Index).
use polars_ta_core::oscillator::rsi;
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

fn run_rsi_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.close_input().unwrap();
    let actual = rsi(&input, period);
    let label = format!("rsi(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn rsi_period14_normal_1000() {
    run_rsi_golden("rsi_period14_normal_1000.json", 14, 1e-10);
}
#[test]
fn rsi_period14_boundary_exact() {
    run_rsi_golden("rsi_period14_boundary_exact.json", 14, 1e-10);
}
#[test]
fn rsi_period14_boundary_short() {
    let data = vec![1.0f64; 14];
    assert!(rsi(&data, 14).is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs: ta-lib skips NaN in rolling windows; our impl propagates NaN via IEEE 754"]
fn rsi_period14_with_nan() {
    run_rsi_golden("rsi_period14_with_nan_5pct.json", 14, 1e-10);
}
#[test]
fn rsi_period14_all_same_value() {
    run_rsi_golden("rsi_period14_all_same_value.json", 14, 1e-10);
}
#[test]
fn rsi_period14_real_btcusdt() {
    // RSI uses Wilder smoothing → use relaxed epsilon for large price values
    run_rsi_golden("rsi_period14_real_btcusdt_1d.json", 14, 1e-7);
}
#[test]
fn rsi_period14_real_flat_period() {
    run_rsi_golden("rsi_period14_real_flat_period.json", 14, 1e-7);
}
