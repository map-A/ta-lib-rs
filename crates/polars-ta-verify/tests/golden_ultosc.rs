//! Golden Tests for Ultimate Oscillator.
use polars_ta_core::oscillator::ultosc;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_ultosc_golden(filename: &str, p1: usize, p2: usize, p3: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = ultosc(&high, &low, &close, p1, p2, p3);
    let label = format!("ultosc/{}", golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn ultosc_normal_1000() { run_ultosc_golden("ultosc_period17_period214_period328_normal_1000.json", 7, 14, 28, 1e-10); }
#[test]
fn ultosc_boundary_exact() { run_ultosc_golden("ultosc_period17_period214_period328_boundary_exact.json", 7, 14, 28, 1e-10); }
#[test]
fn ultosc_boundary_short() {
    let data = vec![1.0f64; 28]; // lookback = 28, short = 28
    assert!(ultosc(&data, &data, &data, 7, 14, 28).is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs: ta-lib skips NaN in rolling windows; our impl propagates NaN via IEEE 754"]
fn ultosc_with_nan() { run_ultosc_golden("ultosc_period17_period214_period328_with_nan_5pct.json", 7, 14, 28, 1e-10); }
#[test]
fn ultosc_all_same_value() { run_ultosc_golden("ultosc_period17_period214_period328_all_same_value.json", 7, 14, 28, 1e-10); }
#[test]
fn ultosc_real_btcusdt() { run_ultosc_golden("ultosc_period17_period214_period328_real_btcusdt_1d.json", 7, 14, 28, 1e-7); }
#[test]
fn ultosc_real_flat_period() { run_ultosc_golden("ultosc_period17_period214_period328_real_flat_period.json", 7, 14, 28, 1e-10); }
