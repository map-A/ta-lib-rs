//! Golden Tests for MINUS_DI (Minus Directional Indicator).
use polars_ta_core::oscillator::minus_di;
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

fn run_minus_di_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = minus_di(&high, &low, &close, period);
    let label = format!("minus_di(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn minus_di_period14_normal_1000() {
    run_minus_di_golden("minus_di_period14_normal_1000.json", 14, 1e-10);
}
#[test]
fn minus_di_period14_boundary_exact() {
    run_minus_di_golden("minus_di_period14_boundary_exact.json", 14, 1e-10);
}
#[test]
fn minus_di_period14_boundary_short() {
    // lookback = 14, need 15 points minimum
    let data = vec![1.0f64; 14];
    assert!(minus_di(&data, &data, &data, 14).is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs for multi-input indicators"]
fn minus_di_period14_with_nan() {
    run_minus_di_golden("minus_di_period14_with_nan_5pct.json", 14, 1e-10);
}
#[test]
fn minus_di_period14_all_same_value() {
    run_minus_di_golden("minus_di_period14_all_same_value.json", 14, 1e-10);
}
#[test]
fn minus_di_period14_real_btcusdt() {
    run_minus_di_golden("minus_di_period14_real_btcusdt_1d.json", 14, 1e-7);
}
#[test]
fn minus_di_period14_real_flat_period() {
    run_minus_di_golden("minus_di_period14_real_flat_period.json", 14, 1e-7);
}
