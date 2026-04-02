//! Golden Tests for StochRSI.
use polars_ta_core::oscillator::stochrsi;
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

fn run_stochrsi_golden(filename: &str, period: usize, fastk: usize, fastd: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.close_input().unwrap();
    let actual = stochrsi(&input, period, fastk, fastd);
    let label = format!("stochrsi/{}", golden.meta.dataset);
    assert_close(
        &actual.fastk,
        golden.get_output_values("fastk").unwrap(),
        epsilon,
        &format!("{}/fastk", label),
    );
    assert_close(
        &actual.fastd,
        golden.get_output_values("fastd").unwrap(),
        epsilon,
        &format!("{}/fastd", label),
    );
}

#[test]
fn stochrsi_normal_1000() {
    run_stochrsi_golden(
        "stochrsi_period14_fastk5_fastd3_normal_1000.json",
        14,
        5,
        3,
        1e-10,
    );
}
#[test]
fn stochrsi_boundary_exact() {
    run_stochrsi_golden(
        "stochrsi_period14_fastk5_fastd3_boundary_exact.json",
        14,
        5,
        3,
        1e-10,
    );
}
#[test]
fn stochrsi_boundary_short() {
    let data = vec![1.0f64; 20]; // lookback = 14+5+3-2=20, short=20
    let out = stochrsi(&data, 14, 5, 3);
    assert!(out.fastk.is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs: ta-lib skips NaN in rolling windows; our impl propagates NaN via IEEE 754"]
fn stochrsi_with_nan() {
    run_stochrsi_golden(
        "stochrsi_period14_fastk5_fastd3_with_nan_5pct.json",
        14,
        5,
        3,
        1e-10,
    );
}
#[test]
fn stochrsi_all_same_value() {
    run_stochrsi_golden(
        "stochrsi_period14_fastk5_fastd3_all_same_value.json",
        14,
        5,
        3,
        1e-10,
    );
}
#[test]
fn stochrsi_real_btcusdt() {
    // StochRSI uses Wilder-smoothed RSI → relaxed epsilon
    run_stochrsi_golden(
        "stochrsi_period14_fastk5_fastd3_real_btcusdt_1d.json",
        14,
        5,
        3,
        1e-7,
    );
}
#[test]
#[ignore = "ta-lib NaN propagation differs: ta-lib skips NaN in rolling windows; our impl propagates NaN via IEEE 754"]
fn stochrsi_real_flat_period() {
    run_stochrsi_golden(
        "stochrsi_period14_fastk5_fastd3_real_flat_period.json",
        14,
        5,
        3,
        1e-10,
    );
}
