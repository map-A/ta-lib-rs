//! Golden Tests for WCLPRICE (Weighted Close Price).
use polars_ta_core::price_transform::wclprice;
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

fn run_wclprice_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden =
        load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = wclprice(&high, &low, &close);
    let label = format!("wclprice/{}", golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn wclprice_normal_1000() { run_wclprice_golden("wclprice__normal_1000.json", 1e-10); }
#[test]
#[ignore = "NaN propagation: ta-lib skips NaN in multi-input windows, we propagate per IEEE 754"]
fn wclprice_with_nan() { run_wclprice_golden("wclprice__with_nan_5pct.json", 1e-10); }
#[test]
fn wclprice_all_same_value() { run_wclprice_golden("wclprice__all_same_value.json", 1e-10); }
#[test]
fn wclprice_real_btcusdt() { run_wclprice_golden("wclprice__real_btcusdt_1d.json", 1e-10); }
#[test]
fn wclprice_real_flat_period() { run_wclprice_golden("wclprice__real_flat_period.json", 1e-10); }
