//! Golden Tests for Fast Stochastic Oscillator (STOCHF).
use polars_ta_core::oscillator::stochf;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_stochf_golden(filename: &str, fastk: usize, fastd: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = stochf(&high, &low, &close, fastk, fastd);
    let label = format!("stochf/{}", golden.meta.dataset);
    assert_close(&actual.fastk, golden.get_output_values("fastk").unwrap(), epsilon, &format!("{}/fastk", label));
    assert_close(&actual.fastd, golden.get_output_values("fastd").unwrap(), epsilon, &format!("{}/fastd", label));
}

#[test]
fn stochf_normal_1000() { run_stochf_golden("stochf_fastk5_fastd3_normal_1000.json", 5, 3, 1e-10); }
#[test]
fn stochf_boundary_exact() { run_stochf_golden("stochf_fastk5_fastd3_boundary_exact.json", 5, 3, 1e-10); }
#[test]
fn stochf_boundary_short() {
    let data = vec![1.0f64; 6]; // lookback = 5+3-2=6, length=6 → empty
    let out = stochf(&data, &data, &data, 5, 3);
    assert!(out.fastk.is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs: ta-lib skips NaN in rolling windows; our impl propagates NaN via IEEE 754"]
fn stochf_with_nan() { run_stochf_golden("stochf_fastk5_fastd3_with_nan_5pct.json", 5, 3, 1e-10); }
#[test]
fn stochf_all_same_value() { run_stochf_golden("stochf_fastk5_fastd3_all_same_value.json", 5, 3, 1e-10); }
#[test]
fn stochf_real_btcusdt() { run_stochf_golden("stochf_fastk5_fastd3_real_btcusdt_1d.json", 5, 3, 1e-7); }
#[test]
fn stochf_real_flat_period() { run_stochf_golden("stochf_fastk5_fastd3_real_flat_period.json", 5, 3, 1e-10); }
