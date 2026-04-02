//! Golden Tests for MACD.
use polars_ta_core::trend::macd;
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

fn run_macd_golden(filename: &str, fast: usize, slow: usize, signal: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.close_input().unwrap();
    let actual = macd(&input, fast, slow, signal);
    let label = format!("macd/{}", golden.meta.dataset);
    assert_close(
        &actual.macd,
        golden.get_output_values("macd").unwrap(),
        epsilon,
        &format!("{}/macd", label),
    );
    assert_close(
        &actual.signal,
        golden.get_output_values("signal").unwrap(),
        epsilon,
        &format!("{}/signal", label),
    );
    assert_close(
        &actual.hist,
        golden.get_output_values("hist").unwrap(),
        epsilon,
        &format!("{}/hist", label),
    );
}

#[test]
fn macd_normal_1000() {
    run_macd_golden(
        "macd_fast12_slow26_signal9_normal_1000.json",
        12,
        26,
        9,
        1e-10,
    );
}
#[test]
fn macd_boundary_exact() {
    run_macd_golden(
        "macd_fast12_slow26_signal9_boundary_exact.json",
        12,
        26,
        9,
        1e-10,
    );
}
#[test]
fn macd_boundary_short() {
    let data = vec![1.0f64; 33]; // lookback = 26+9-2=33, short = 33
    let out = macd(&data, 12, 26, 9);
    assert!(out.macd.is_empty());
}
#[test]
fn macd_with_nan() {
    run_macd_golden(
        "macd_fast12_slow26_signal9_with_nan_5pct.json",
        12,
        26,
        9,
        1e-10,
    );
}
#[test]
fn macd_all_same_value() {
    run_macd_golden(
        "macd_fast12_slow26_signal9_all_same_value.json",
        12,
        26,
        9,
        1e-10,
    );
}
#[test]
fn macd_real_btcusdt() {
    run_macd_golden(
        "macd_fast12_slow26_signal9_real_btcusdt_1d.json",
        12,
        26,
        9,
        1e-7,
    );
}
#[test]
fn macd_real_flat_period() {
    run_macd_golden(
        "macd_fast12_slow26_signal9_real_flat_period.json",
        12,
        26,
        9,
        1e-10,
    );
}
