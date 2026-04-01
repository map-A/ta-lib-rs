//! Golden Tests for MACDFIX (MACD with fixed 12/26 periods).
use polars_ta_core::trend::macdfix;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_macdfix_golden(filename: &str, signal: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.close_input().unwrap();
    let actual = macdfix(&input, signal);
    let label = format!("macdfix/{}", golden.meta.dataset);
    assert_close(&actual.macd,   golden.get_output_values("macd").unwrap(),   epsilon, &format!("{}/macd", label));
    assert_close(&actual.signal, golden.get_output_values("signal").unwrap(), epsilon, &format!("{}/signal", label));
    assert_close(&actual.hist,   golden.get_output_values("hist").unwrap(),   epsilon, &format!("{}/hist", label));
}

#[test]
fn macdfix_normal_1000() { run_macdfix_golden("macdfix_signal9_normal_1000.json", 9, 1e-10); }
#[test]
fn macdfix_boundary_exact() { run_macdfix_golden("macdfix_signal9_boundary_exact.json", 9, 1e-10); }
#[test]
fn macdfix_boundary_short() {
    let data = vec![1.0f64; 33]; // lookback=33
    let out = macdfix(&data, 9);
    assert!(out.macd.is_empty());
}
#[test]
fn macdfix_with_nan() { run_macdfix_golden("macdfix_signal9_with_nan_5pct.json", 9, 1e-10); }
#[test]
fn macdfix_all_same_value() { run_macdfix_golden("macdfix_signal9_all_same_value.json", 9, 1e-10); }
#[test]
fn macdfix_real_btcusdt() { run_macdfix_golden("macdfix_signal9_real_btcusdt_1d.json", 9, 1e-7); }
#[test]
fn macdfix_real_flat_period() { run_macdfix_golden("macdfix_signal9_real_flat_period.json", 9, 1e-10); }
