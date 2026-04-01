//! Golden Tests for MA (Generic Moving Average dispatcher).
use polars_ta_core::trend::ma;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_ma_golden(filename: &str, period: usize, matype: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.close_input().unwrap();
    let actual = ma(&input, period, matype);
    let label = format!("ma(period={},matype={})/{}", period, matype, golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn ma_period30_matype1_normal_1000() { run_ma_golden("ma_period30_matype1_normal_1000.json", 30, 1, 1e-10); }
#[test]
fn ma_period30_matype1_boundary_exact() { run_ma_golden("ma_period30_matype1_boundary_exact.json", 30, 1, 1e-10); }
#[test]
fn ma_period30_matype1_boundary_short() {
    let data = vec![1.0f64; 29]; // lookback = 30-1=29, length=29 → empty
    let out = ma(&data, 30, 1);
    assert!(out.is_empty());
}
#[test]
fn ma_period30_matype1_with_nan() { run_ma_golden("ma_period30_matype1_with_nan_5pct.json", 30, 1, 1e-10); }
#[test]
fn ma_period30_matype1_all_same_value() { run_ma_golden("ma_period30_matype1_all_same_value.json", 30, 1, 1e-10); }
#[test]
fn ma_period30_matype1_real_btcusdt() { run_ma_golden("ma_period30_matype1_real_btcusdt_1d.json", 30, 1, 1e-7); }
#[test]
fn ma_period30_matype1_real_flat_period() { run_ma_golden("ma_period30_matype1_real_flat_period.json", 30, 1, 1e-10); }
