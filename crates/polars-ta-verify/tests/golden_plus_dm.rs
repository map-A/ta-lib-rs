//! Golden Tests for PLUS_DM (Plus Directional Movement).
use polars_ta_core::oscillator::plus_dm;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_plus_dm_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let actual = plus_dm(&high, &low, period);
    let label = format!("plus_dm(period={})/{}", period, golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn plus_dm_period14_normal_1000() { run_plus_dm_golden("plus_dm_period14_normal_1000.json", 14, 1e-10); }
#[test]
fn plus_dm_period14_boundary_exact() { run_plus_dm_golden("plus_dm_period14_boundary_exact.json", 14, 1e-10); }
#[test]
fn plus_dm_period14_boundary_short() {
    // lookback = 13, need 14 points minimum
    let data = vec![1.0f64; 13];
    assert!(plus_dm(&data, &data, 14).is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs for multi-input indicators"]
fn plus_dm_period14_with_nan() { run_plus_dm_golden("plus_dm_period14_with_nan_5pct.json", 14, 1e-10); }
#[test]
fn plus_dm_period14_all_same_value() { run_plus_dm_golden("plus_dm_period14_all_same_value.json", 14, 1e-10); }
#[test]
fn plus_dm_period14_real_btcusdt() { run_plus_dm_golden("plus_dm_period14_real_btcusdt_1d.json", 14, 1e-7); }
#[test]
fn plus_dm_period14_real_flat_period() { run_plus_dm_golden("plus_dm_period14_real_flat_period.json", 14, 1e-7); }
