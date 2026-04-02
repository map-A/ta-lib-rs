//! Golden Tests for CCI (Commodity Channel Index).
use polars_ta_core::oscillator::cci;
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

fn run_cci_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = cci(&high, &low, &close, period);
    let label = format!("cci(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn cci_period20_normal_1000() {
    run_cci_golden("cci_period20_normal_1000.json", 20, 1e-10);
}
#[test]
fn cci_period20_boundary_exact() {
    run_cci_golden("cci_period20_boundary_exact.json", 20, 1e-10);
}
#[test]
fn cci_period20_boundary_short() {
    let data = vec![1.0f64; 19];
    assert!(cci(&data, &data, &data, 20).is_empty());
}
#[test]
fn cci_period20_with_nan() {
    run_cci_golden("cci_period20_with_nan_5pct.json", 20, 1e-10);
}
#[test]
fn cci_period20_all_same_value() {
    run_cci_golden("cci_period20_all_same_value.json", 20, 1e-10);
}
#[test]
fn cci_period20_real_btcusdt() {
    run_cci_golden("cci_period20_real_btcusdt_1d.json", 20, 1e-7);
}
#[test]
fn cci_period20_real_flat_period() {
    run_cci_golden("cci_period20_real_flat_period.json", 20, 1e-6);
}
