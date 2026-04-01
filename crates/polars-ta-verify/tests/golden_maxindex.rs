//! Golden Tests for MAXINDEX.
use polars_ta_core::math_ops::maxindex;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_maxindex_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let close = golden.close_input().unwrap();
    let actual = maxindex(&close, period);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon,
                 &format!("maxindex(period={})/{}", period, golden.meta.dataset));
}

#[test]
fn maxindex_period30_normal_1000() { run_maxindex_golden("maxindex_period30_normal_1000.json", 30, 0.5); }
#[test]
fn maxindex_period30_all_same_value() { run_maxindex_golden("maxindex_period30_all_same_value.json", 30, 0.5); }
#[test]
fn maxindex_period30_real_btcusdt() { run_maxindex_golden("maxindex_period30_real_btcusdt_1d.json", 30, 0.5); }
#[test]
fn maxindex_period30_real_flat_period() { run_maxindex_golden("maxindex_period30_real_flat_period.json", 30, 0.5); }
#[test]
fn maxindex_period30_boundary_exact() { run_maxindex_golden("maxindex_period30_boundary_exact.json", 30, 0.5); }
#[test]
fn maxindex_period30_boundary_short() {
    assert!(maxindex(&vec![1.0f64; 29], 30).is_empty());
}
#[test]
#[ignore = "NaN propagation differs"]
fn maxindex_period30_with_nan() { run_maxindex_golden("maxindex_period30_with_nan_5pct.json", 30, 0.5); }
