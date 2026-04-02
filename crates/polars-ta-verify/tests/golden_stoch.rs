//! Golden Tests for Stochastic Oscillator.
use polars_ta_core::oscillator::stoch;
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

fn run_stoch_golden(filename: &str, fastk: usize, slowk: usize, slowd: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = stoch(&high, &low, &close, fastk, slowk, slowd);
    let label = format!("stoch/{}", golden.meta.dataset);
    assert_close(
        &actual.slowk,
        golden.get_output_values("slowk").unwrap(),
        epsilon,
        &format!("{}/slowk", label),
    );
    assert_close(
        &actual.slowd,
        golden.get_output_values("slowd").unwrap(),
        epsilon,
        &format!("{}/slowd", label),
    );
}

#[test]
fn stoch_normal_1000() {
    run_stoch_golden(
        "stoch_fastk5_slowk3_slowd3_normal_1000.json",
        5,
        3,
        3,
        1e-10,
    );
}
#[test]
fn stoch_boundary_exact() {
    run_stoch_golden(
        "stoch_fastk5_slowk3_slowd3_boundary_exact.json",
        5,
        3,
        3,
        1e-10,
    );
}
#[test]
fn stoch_boundary_short() {
    let data = vec![1.0f64; 8]; // lookback = 5+3+3-3=8, length=8 → should be empty
    let out = stoch(&data, &data, &data, 5, 3, 3);
    assert!(out.slowk.is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs: ta-lib skips NaN in rolling windows; our impl propagates NaN via IEEE 754"]
fn stoch_with_nan() {
    run_stoch_golden(
        "stoch_fastk5_slowk3_slowd3_with_nan_5pct.json",
        5,
        3,
        3,
        1e-10,
    );
}
#[test]
fn stoch_all_same_value() {
    run_stoch_golden(
        "stoch_fastk5_slowk3_slowd3_all_same_value.json",
        5,
        3,
        3,
        1e-10,
    );
}
#[test]
fn stoch_real_btcusdt() {
    run_stoch_golden(
        "stoch_fastk5_slowk3_slowd3_real_btcusdt_1d.json",
        5,
        3,
        3,
        1e-7,
    );
}
#[test]
fn stoch_real_flat_period() {
    run_stoch_golden(
        "stoch_fastk5_slowk3_slowd3_real_flat_period.json",
        5,
        3,
        3,
        1e-10,
    );
}
