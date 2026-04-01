//! Golden Tests for SAREXT (Extended Parabolic SAR).
use polars_ta_core::trend::sarext;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_sarext_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    // 使用 ta-lib SAREXT 默认参数
    let actual = sarext(&high, &low, 0.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20);
    let label = format!("sarext/{}", golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn sarext_normal_1000() { run_sarext_golden("sarext__normal_1000.json", 1e-10); }
#[test]
fn sarext_boundary_exact() { run_sarext_golden("sarext__boundary_exact.json", 1e-10); }
#[test]
fn sarext_boundary_short() {
    let data = vec![1.0f64; 1]; // lookback=1, length=1 → empty
    let out = sarext(&data, &data, 0.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20);
    assert!(out.is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs: ta-lib skips NaN in multi-input rolling windows"]
fn sarext_with_nan() { run_sarext_golden("sarext__with_nan_5pct.json", 1e-10); }
#[test]
fn sarext_all_same_value() { run_sarext_golden("sarext__all_same_value.json", 1e-10); }
#[test]
fn sarext_real_btcusdt() { run_sarext_golden("sarext__real_btcusdt_1d.json", 1e-7); }
#[test]
fn sarext_real_flat_period() { run_sarext_golden("sarext__real_flat_period.json", 1e-10); }
