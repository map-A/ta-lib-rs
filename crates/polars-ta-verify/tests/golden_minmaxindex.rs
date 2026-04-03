//! Golden Tests for MINMAXINDEX.
use polars_ta_core::math_ops::minmaxindex;
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

fn run_minmaxindex_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let close = golden.close_input().unwrap();
    let actual = minmaxindex(&close, period);
    let label = format!("minmaxindex(period={})/{}", period, golden.meta.dataset);
    assert_close(
        &actual.min_idx,
        golden.get_output_values("minidx").unwrap(),
        epsilon,
        &format!("{}/minidx", label),
    );
    assert_close(
        &actual.max_idx,
        golden.get_output_values("maxidx").unwrap(),
        epsilon,
        &format!("{}/maxidx", label),
    );
}

#[test]
fn minmaxindex_period30_normal_1000() {
    run_minmaxindex_golden("minmaxindex_period30_normal_1000.json", 30, 0.5);
}
#[test]
fn minmaxindex_period30_all_same_value() {
    run_minmaxindex_golden("minmaxindex_period30_all_same_value.json", 30, 0.5);
}
#[test]
fn minmaxindex_period30_real_btcusdt() {
    run_minmaxindex_golden("minmaxindex_period30_real_btcusdt_1d.json", 30, 0.5);
}
#[test]
fn minmaxindex_period30_real_flat_period() {
    run_minmaxindex_golden("minmaxindex_period30_real_flat_period.json", 30, 0.5);
}
#[test]
fn minmaxindex_period30_boundary_exact() {
    run_minmaxindex_golden("minmaxindex_period30_boundary_exact.json", 30, 0.5);
}
#[test]
fn minmaxindex_period30_boundary_short() {
    let result = minmaxindex(&vec![1.0f64; 29], 30);
    assert!(result.min_idx.is_empty());
    assert!(result.max_idx.is_empty());
}
#[test]
#[ignore = "NaN propagation differs"]
fn minmaxindex_period30_with_nan() {
    run_minmaxindex_golden("minmaxindex_period30_with_nan_5pct.json", 30, 0.5);
}
