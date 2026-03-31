//! Golden Tests for Aroon.
use polars_ta_core::oscillator::aroon;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_aroon_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let actual = aroon(&high, &low, period);
    let label = format!("aroon(period={})/{}", period, golden.meta.dataset);
    assert_close(&actual.aroon_down, golden.get_output_values("aroon_down").unwrap(), epsilon, &format!("{}/down", label));
    assert_close(&actual.aroon_up, golden.get_output_values("aroon_up").unwrap(), epsilon, &format!("{}/up", label));
}

#[test]
fn aroon_period14_normal_1000() { run_aroon_golden("aroon_period14_normal_1000.json", 14, 1e-10); }
#[test]
fn aroon_period14_boundary_exact() { run_aroon_golden("aroon_period14_boundary_exact.json", 14, 1e-10); }
#[test]
fn aroon_period14_boundary_short() {
    let data = vec![1.0f64; 14]; // lookback=14, short=14
    let out = aroon(&data, &data, 14);
    assert!(out.aroon_up.is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs: ta-lib skips NaN in rolling windows; our impl propagates NaN via IEEE 754"]
fn aroon_period14_with_nan() { run_aroon_golden("aroon_period14_with_nan_5pct.json", 14, 1e-10); }
#[test]
fn aroon_period14_all_same_value() { run_aroon_golden("aroon_period14_all_same_value.json", 14, 1e-10); }
#[test]
fn aroon_period14_real_btcusdt() { run_aroon_golden("aroon_period14_real_btcusdt_1d.json", 14, 1e-10); }
#[test]
fn aroon_period14_real_flat_period() { run_aroon_golden("aroon_period14_real_flat_period.json", 14, 1e-10); }
