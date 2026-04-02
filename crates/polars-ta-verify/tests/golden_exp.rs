//! Golden Tests for EXP (Math Transform).
use polars_ta_verify::golden::{assert_close_relative, load_golden_file};
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

fn run_exp_golden(filename: &str) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.get_input("close").unwrap();
    let actual = polars_ta_core::math_transform::exp(&input);
    let label = format!("exp/{}", golden.meta.dataset);
    // EXP of large inputs overflows to infinity (serialized as null); use relative epsilon for finite values
    assert_close_relative(
        &actual,
        golden.get_output_values("values").unwrap(),
        1.0,
        1e-10,
        &label,
    );
}

#[test]
fn exp_normal_1000() {
    run_exp_golden("exp__normal_1000.json");
}
#[test]
#[ignore = "NaN propagation: ta-lib skips NaN, we propagate per IEEE 754"]
fn exp_with_nan() {
    run_exp_golden("exp__with_nan_5pct.json");
}
#[test]
fn exp_all_same_value() {
    run_exp_golden("exp__all_same_value.json");
}
#[test]
fn exp_real_btcusdt() {
    run_exp_golden("exp__real_btcusdt_1d.json");
}
#[test]
fn exp_real_flat_period() {
    run_exp_golden("exp__real_flat_period.json");
}
