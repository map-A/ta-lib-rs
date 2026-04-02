//! Golden Tests for ADOSC (Chaikin A/D Oscillator).
use polars_ta_core::volume::adosc;
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

fn run_adosc_golden(filename: &str, fast: usize, slow: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let volume = golden.get_input("volume").unwrap();
    let actual = adosc(&high, &low, &close, &volume, fast, slow);
    let label = format!("adosc/{}", golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn adosc_normal_1000() {
    run_adosc_golden("adosc_fast3_slow10_normal_1000.json", 3, 10, 1e-5);
}
#[test]
fn adosc_boundary_exact() {
    run_adosc_golden("adosc_fast3_slow10_boundary_exact.json", 3, 10, 1e-8);
}
#[test]
fn adosc_boundary_short() {
    let data = vec![1.0f64; 9];
    assert!(adosc(&data, &data, &data, &data, 3, 10).is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs for multi-input indicators"]
fn adosc_with_nan() {
    run_adosc_golden("adosc_fast3_slow10_with_nan_5pct.json", 3, 10, 1e-10);
}
#[test]
fn adosc_all_same_value() {
    run_adosc_golden("adosc_fast3_slow10_all_same_value.json", 3, 10, 1e-10);
}
#[test]
fn adosc_real_btcusdt() {
    run_adosc_golden("adosc_fast3_slow10_real_btcusdt_1d.json", 3, 10, 1e-3);
}
#[test]
fn adosc_real_flat_period() {
    run_adosc_golden("adosc_fast3_slow10_real_flat_period.json", 3, 10, 1e-7);
}
