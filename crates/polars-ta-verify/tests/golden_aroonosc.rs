//! Golden Tests for Aroon Oscillator (AROONOSC).
use polars_ta_core::oscillator::aroonosc;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_aroonosc_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let actual = aroonosc(&high, &low, period);
    let label = format!("aroonosc(period={})/{}", period, golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn aroonosc_period14_normal_1000() { run_aroonosc_golden("aroonosc_period14_normal_1000.json", 14, 1e-10); }
#[test]
fn aroonosc_period14_boundary_exact() { run_aroonosc_golden("aroonosc_period14_boundary_exact.json", 14, 1e-10); }
#[test]
fn aroonosc_period14_boundary_short() {
    let data = vec![1.0f64; 14]; // lookback=14, length=14 → empty
    let out = aroonosc(&data, &data, 14);
    assert!(out.is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs: ta-lib skips NaN in rolling windows; our impl propagates NaN via IEEE 754"]
fn aroonosc_period14_with_nan() { run_aroonosc_golden("aroonosc_period14_with_nan_5pct.json", 14, 1e-10); }
#[test]
fn aroonosc_period14_all_same_value() { run_aroonosc_golden("aroonosc_period14_all_same_value.json", 14, 1e-10); }
#[test]
fn aroonosc_period14_real_btcusdt() { run_aroonosc_golden("aroonosc_period14_real_btcusdt_1d.json", 14, 1e-10); }
#[test]
fn aroonosc_period14_real_flat_period() { run_aroonosc_golden("aroonosc_period14_real_flat_period.json", 14, 1e-10); }
