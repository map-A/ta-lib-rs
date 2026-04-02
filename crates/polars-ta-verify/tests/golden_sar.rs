//! Golden Tests for SAR (Parabolic SAR).
use polars_ta_core::trend::sar;
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

fn run_sar_golden(filename: &str, acceleration: f64, maximum: f64, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let actual = sar(&high, &low, acceleration, maximum);
    let label = format!("sar/{}", golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn sar_normal_1000() {
    run_sar_golden(
        "sar_acceleration0.02_maximum0.2_normal_1000.json",
        0.02,
        0.2,
        1e-10,
    );
}
#[test]
fn sar_boundary_exact() {
    run_sar_golden(
        "sar_acceleration0.02_maximum0.2_boundary_exact.json",
        0.02,
        0.2,
        1e-10,
    );
}
#[test]
fn sar_boundary_short() {
    let data = vec![1.0f64; 1];
    assert!(sar(&data, &data, 0.02, 0.2).is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs: ta-lib skips NaN in multi-input rolling windows"]
fn sar_with_nan() {
    run_sar_golden(
        "sar_acceleration0.02_maximum0.2_with_nan_5pct.json",
        0.02,
        0.2,
        1e-10,
    );
}
#[test]
fn sar_all_same_value() {
    run_sar_golden(
        "sar_acceleration0.02_maximum0.2_all_same_value.json",
        0.02,
        0.2,
        1e-10,
    );
}
#[test]
fn sar_real_btcusdt() {
    run_sar_golden(
        "sar_acceleration0.02_maximum0.2_real_btcusdt_1d.json",
        0.02,
        0.2,
        1e-7,
    );
}
#[test]
fn sar_real_flat_period() {
    run_sar_golden(
        "sar_acceleration0.02_maximum0.2_real_flat_period.json",
        0.02,
        0.2,
        1e-10,
    );
}
