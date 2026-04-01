//! Golden Tests for MACDEXT (MACD with configurable MA types).
use polars_ta_core::trend::macdext;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_macdext_golden(filename: &str, fast: usize, slow: usize, signal: usize, matype: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.close_input().unwrap();
    let actual = macdext(&input, fast, matype, slow, matype, signal, matype);
    let label = format!("macdext/{}", golden.meta.dataset);
    assert_close(&actual.macd,   golden.get_output_values("macd").unwrap(),   epsilon, &format!("{}/macd", label));
    assert_close(&actual.signal, golden.get_output_values("signal").unwrap(), epsilon, &format!("{}/signal", label));
    assert_close(&actual.hist,   golden.get_output_values("hist").unwrap(),   epsilon, &format!("{}/hist", label));
}

#[test]
fn macdext_normal_1000() { run_macdext_golden("macdext_fast12_slow26_signal9_matype1_normal_1000.json", 12, 26, 9, 1, 1e-10); }
#[test]
fn macdext_boundary_exact() { run_macdext_golden("macdext_fast12_slow26_signal9_matype1_boundary_exact.json", 12, 26, 9, 1, 1e-10); }
#[test]
fn macdext_boundary_short() {
    let data = vec![1.0f64; 33]; // lookback=33
    let out = macdext(&data, 12, 1, 26, 1, 9, 1);
    assert!(out.macd.is_empty());
}
#[test]
fn macdext_with_nan() { run_macdext_golden("macdext_fast12_slow26_signal9_matype1_with_nan_5pct.json", 12, 26, 9, 1, 1e-10); }
#[test]
fn macdext_all_same_value() { run_macdext_golden("macdext_fast12_slow26_signal9_matype1_all_same_value.json", 12, 26, 9, 1, 1e-10); }
#[test]
fn macdext_real_btcusdt() { run_macdext_golden("macdext_fast12_slow26_signal9_matype1_real_btcusdt_1d.json", 12, 26, 9, 1, 1e-7); }
#[test]
fn macdext_real_flat_period() { run_macdext_golden("macdext_fast12_slow26_signal9_matype1_real_flat_period.json", 12, 26, 9, 1, 1e-10); }
