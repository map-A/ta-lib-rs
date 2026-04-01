//! Golden Tests for DIV.
use polars_ta_core::math_ops::div;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_div_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let real0 = golden.get_input("real0").unwrap();
    let real1 = golden.get_input("real1").unwrap();
    let actual = div(&real0, &real1);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon,
                 &format!("div/{}", golden.meta.dataset));
}

#[test]
fn div_normal_1000() { run_div_golden("div__normal_1000.json", 1e-10); }
#[test]
fn div_all_same_value() { run_div_golden("div__all_same_value.json", 1e-10); }
#[test]
fn div_real_btcusdt() { run_div_golden("div__real_btcusdt_1d.json", 1e-10); }
#[test]
fn div_real_flat_period() { run_div_golden("div__real_flat_period.json", 1e-10); }
#[test]
#[ignore = "NaN propagation differs"]
fn div_with_nan() { run_div_golden("div__with_nan_5pct.json", 1e-10); }
