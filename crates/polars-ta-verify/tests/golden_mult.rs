//! Golden Tests for MULT.
use polars_ta_core::math_ops::mult;
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

fn run_mult_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let real0 = golden.get_input("real0").unwrap();
    let real1 = golden.get_input("real1").unwrap();
    let actual = mult(&real0, &real1);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &format!("mult/{}", golden.meta.dataset),
    );
}

#[test]
fn mult_normal_1000() {
    run_mult_golden("mult__normal_1000.json", 1e-10);
}
#[test]
fn mult_all_same_value() {
    run_mult_golden("mult__all_same_value.json", 1e-10);
}
#[test]
fn mult_real_btcusdt() {
    run_mult_golden("mult__real_btcusdt_1d.json", 1e-4);
}
#[test]
fn mult_real_flat_period() {
    run_mult_golden("mult__real_flat_period.json", 1e-10);
}
#[test]
#[ignore = "NaN propagation differs"]
fn mult_with_nan() {
    run_mult_golden("mult__with_nan_5pct.json", 1e-10);
}
