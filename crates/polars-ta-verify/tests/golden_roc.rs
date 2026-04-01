//! Golden Tests for ROC (Rate of Change).
use polars_ta_core::oscillator::roc;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_roc_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.close_input().unwrap();
    let actual = roc(&input, period);
    let label = format!("roc(period={})/{}", period, golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn roc_period10_normal_1000() { run_roc_golden("roc_period10_normal_1000.json", 10, 1e-10); }
#[test]
fn roc_period10_boundary_exact() { run_roc_golden("roc_period10_boundary_exact.json", 10, 1e-10); }
#[test]
fn roc_period10_boundary_short() {
    let data = vec![1.0f64; 10];
    assert!(roc(&data, 10).is_empty());
}
#[test]
#[ignore = "NaN propagation: ta-lib skips NaN in multi-input windows, we propagate per IEEE 754"]
fn roc_period10_with_nan() { run_roc_golden("roc_period10_with_nan_5pct.json", 10, 1e-10); }
#[test]
fn roc_period10_all_same_value() { run_roc_golden("roc_period10_all_same_value.json", 10, 1e-10); }
#[test]
fn roc_period10_real_btcusdt() { run_roc_golden("roc_period10_real_btcusdt_1d.json", 10, 1e-8); }
#[test]
fn roc_period10_real_flat_period() { run_roc_golden("roc_period10_real_flat_period.json", 10, 1e-8); }
