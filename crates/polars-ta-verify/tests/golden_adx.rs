//! Golden Tests for ADX (Average Directional Index).
use polars_ta_core::trend::adx;
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

fn run_adx_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = adx(&high, &low, &close, period);
    let label = format!("adx(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn adx_period14_normal_1000() {
    run_adx_golden("adx_period14_normal_1000.json", 14, 1e-10);
}
#[test]
fn adx_period14_boundary_exact() {
    run_adx_golden("adx_period14_boundary_exact.json", 14, 1e-10);
}
#[test]
fn adx_period14_boundary_short() {
    let data = vec![1.0f64; 26]; // lookback=27, short=27
    assert!(adx(&data, &data, &data, 14).is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs for multi-input indicators"]
fn adx_period14_with_nan() {
    run_adx_golden("adx_period14_with_nan_5pct.json", 14, 1e-10);
}
#[test]
fn adx_period14_all_same_value() {
    run_adx_golden("adx_period14_all_same_value.json", 14, 1e-10);
}
#[test]
fn adx_period14_real_btcusdt() {
    // ADX uses Wilder smoothing (recursive) → larger BTC price accumulation
    run_adx_golden("adx_period14_real_btcusdt_1d.json", 14, 1e-7);
}
#[test]
fn adx_period14_real_flat_period() {
    run_adx_golden("adx_period14_real_flat_period.json", 14, 1e-8);
}
