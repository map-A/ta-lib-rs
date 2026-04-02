//! Golden Tests for APO (Absolute Price Oscillator).
use polars_ta_core::oscillator::apo;
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

fn run_apo_golden(filename: &str, fast: usize, slow: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.close_input().unwrap();
    let actual = apo(&input, fast, slow);
    let label = format!("apo(fast={},slow={})/{}", fast, slow, golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn apo_fast12_slow26_normal_1000() {
    run_apo_golden("apo_fast12_slow26_normal_1000.json", 12, 26, 1e-10);
}
#[test]
fn apo_fast12_slow26_boundary_exact() {
    run_apo_golden("apo_fast12_slow26_boundary_exact.json", 12, 26, 1e-10);
}
#[test]
fn apo_fast12_slow26_boundary_short() {
    let data = vec![1.0f64; 25];
    assert!(apo(&data, 12, 26).is_empty());
}
#[test]
#[ignore = "NaN propagation: ta-lib skips NaN in multi-input windows, we propagate per IEEE 754"]
fn apo_fast12_slow26_with_nan() {
    run_apo_golden("apo_fast12_slow26_with_nan_5pct.json", 12, 26, 1e-10);
}
#[test]
fn apo_fast12_slow26_all_same_value() {
    run_apo_golden("apo_fast12_slow26_all_same_value.json", 12, 26, 1e-10);
}
#[test]
fn apo_fast12_slow26_real_btcusdt() {
    run_apo_golden("apo_fast12_slow26_real_btcusdt_1d.json", 12, 26, 1e-6);
}
#[test]
fn apo_fast12_slow26_real_flat_period() {
    run_apo_golden("apo_fast12_slow26_real_flat_period.json", 12, 26, 1e-6);
}
