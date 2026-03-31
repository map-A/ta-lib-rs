//! Golden Tests for EMA (Exponential Moving Average).
use polars_ta_core::trend::ema;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_ema_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: golden file not found: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("Failed to load {}: {}", filename, e));
    let input = golden.close_input().unwrap_or_else(|e| panic!("Failed to parse input: {}", e));
    let actual = ema(&input, period);
    let label = format!("ema(period={})/{}", period, golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn ema_period20_normal_1000() { run_ema_golden("ema_period20_normal_1000.json", 20, 1e-10); }
#[test]
fn ema_period20_boundary_exact() { run_ema_golden("ema_period20_boundary_exact.json", 20, 1e-10); }
#[test]
fn ema_period20_boundary_short() {
    let data = vec![1.0f64; 19];
    let result = ema(&data, 20);
    assert!(result.is_empty(), "输入不足时应返回空 Vec，got len={}", result.len());
}
#[test]
fn ema_period20_with_nan() { run_ema_golden("ema_period20_with_nan_5pct.json", 20, 1e-10); }
#[test]
fn ema_period20_all_same_value() { run_ema_golden("ema_period20_all_same_value.json", 20, 1e-10); }
#[test]
fn ema_period20_real_btcusdt() { run_ema_golden("ema_period20_real_btcusdt_1d.json", 20, 1e-7); }
#[test]
fn ema_period20_real_flat_period() { run_ema_golden("ema_period20_real_flat_period.json", 20, 1e-10); }
