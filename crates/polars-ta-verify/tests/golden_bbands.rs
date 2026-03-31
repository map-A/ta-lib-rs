//! Golden Tests for Bollinger Bands.
use polars_ta_core::trend::bbands;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_bbands_golden(filename: &str, period: usize, nbdev_up: f64, nbdev_dn: f64, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let input = golden.close_input().unwrap();
    let actual = bbands(&input, period, nbdev_up, nbdev_dn);
    let label = format!("bbands/{}", golden.meta.dataset);
    assert_close(&actual.upper, golden.get_output_values("upper").unwrap(), epsilon, &format!("{}/upper", label));
    assert_close(&actual.middle, golden.get_output_values("middle").unwrap(), epsilon, &format!("{}/middle", label));
    assert_close(&actual.lower, golden.get_output_values("lower").unwrap(), epsilon, &format!("{}/lower", label));
}

#[test]
fn bbands_normal_1000() { run_bbands_golden("bbands_period20_nbdevup2.0_nbdevdn2.0_normal_1000.json", 20, 2.0, 2.0, 1e-10); }
#[test]
fn bbands_boundary_exact() { run_bbands_golden("bbands_period20_nbdevup2.0_nbdevdn2.0_boundary_exact.json", 20, 2.0, 2.0, 1e-10); }
#[test]
fn bbands_boundary_short() {
    let data = vec![1.0f64; 19];
    let out = bbands(&data, 20, 2.0, 2.0);
    assert!(out.upper.is_empty());
}
#[test]
fn bbands_with_nan() { run_bbands_golden("bbands_period20_nbdevup2.0_nbdevdn2.0_with_nan_5pct.json", 20, 2.0, 2.0, 1e-10); }
#[test]
fn bbands_all_same_value() { run_bbands_golden("bbands_period20_nbdevup2.0_nbdevdn2.0_all_same_value.json", 20, 2.0, 2.0, 1e-10); }
#[test]
fn bbands_real_btcusdt() { run_bbands_golden("bbands_period20_nbdevup2.0_nbdevdn2.0_real_btcusdt_1d.json", 20, 2.0, 2.0, 1e-7); }
#[test]
fn bbands_real_flat_period() { run_bbands_golden("bbands_period20_nbdevup2.0_nbdevdn2.0_real_flat_period.json", 20, 2.0, 2.0, 1e-6); }
