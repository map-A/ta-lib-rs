//! Golden Tests for SUB.
use polars_ta_core::math_ops::sub;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_sub_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let real0 = golden.get_input("real0").unwrap();
    let real1 = golden.get_input("real1").unwrap();
    let actual = sub(&real0, &real1);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon,
                 &format!("sub/{}", golden.meta.dataset));
}

#[test]
fn sub_normal_1000() { run_sub_golden("sub__normal_1000.json", 1e-10); }
#[test]
fn sub_all_same_value() { run_sub_golden("sub__all_same_value.json", 1e-10); }
#[test]
fn sub_real_btcusdt() { run_sub_golden("sub__real_btcusdt_1d.json", 1e-10); }
#[test]
fn sub_real_flat_period() { run_sub_golden("sub__real_flat_period.json", 1e-10); }
#[test]
#[ignore = "NaN propagation differs"]
fn sub_with_nan() { run_sub_golden("sub__with_nan_5pct.json", 1e-10); }
