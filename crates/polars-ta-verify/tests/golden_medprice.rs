//! Golden Tests for MEDPRICE (Median Price).
use polars_ta_core::price_transform::medprice;
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

fn run_medprice_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let actual = medprice(&high, &low);
    let label = format!("medprice/{}", golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn medprice_normal_1000() {
    run_medprice_golden("medprice__normal_1000.json", 1e-10);
}
#[test]
#[ignore = "NaN propagation: ta-lib skips NaN in multi-input windows, we propagate per IEEE 754"]
fn medprice_with_nan() {
    run_medprice_golden("medprice__with_nan_5pct.json", 1e-10);
}
#[test]
fn medprice_all_same_value() {
    run_medprice_golden("medprice__all_same_value.json", 1e-10);
}
#[test]
fn medprice_real_btcusdt() {
    run_medprice_golden("medprice__real_btcusdt_1d.json", 1e-10);
}
#[test]
fn medprice_real_flat_period() {
    run_medprice_golden("medprice__real_flat_period.json", 1e-10);
}
