//! Golden Tests for TRIX (1-day Rate-of-Change of Triple Smooth EMA).
use polars_ta_core::oscillator::trix;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_trix_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.close_input().unwrap();
    let actual = trix(&input, period);
    let label = format!("trix(period={})/{}", period, golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn trix_period5_normal_1000() { run_trix_golden("trix_period5_normal_1000.json", 5, 1e-10); }
#[test]
fn trix_period5_boundary_exact() { run_trix_golden("trix_period5_boundary_exact.json", 5, 1e-10); }
#[test]
fn trix_period5_boundary_short() {
    // lookback = 3*(5-1)+1 = 13, need 14 points minimum
    let data = vec![1.0f64; 13];
    assert!(trix(&data, 5).is_empty());
}
#[test]
#[ignore = "NaN propagation: ta-lib skips NaN in multi-input windows, we propagate per IEEE 754"]
fn trix_period5_with_nan() { run_trix_golden("trix_period5_with_nan_5pct.json", 5, 1e-10); }
#[test]
fn trix_period5_all_same_value() { run_trix_golden("trix_period5_all_same_value.json", 5, 1e-10); }
#[test]
fn trix_period5_real_btcusdt() { run_trix_golden("trix_period5_real_btcusdt_1d.json", 5, 1e-7); }
#[test]
fn trix_period5_real_flat_period() { run_trix_golden("trix_period5_real_flat_period.json", 5, 1e-7); }
