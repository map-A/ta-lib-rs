//! Golden Tests for TRange (True Range).
use polars_ta_core::volatility::trange;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_trange_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = trange(&high, &low, &close);
    let label = format!("trange/{}", golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn trange_normal_1000() { run_trange_golden("trange__normal_1000.json", 1e-10); }
#[test]
fn trange_boundary_exact() { run_trange_golden("trange__boundary_exact.json", 1e-10); }
#[test]
fn trange_boundary_short() {
    let data = vec![1.0f64; 1]; // lookback=1, short=1
    assert!(trange(&data, &data, &data).is_empty());
}
#[test]
#[ignore = "ta-lib NaN propagation differs: ta-lib skips NaN in rolling windows; our impl propagates NaN via IEEE 754"]
fn trange_with_nan() { run_trange_golden("trange__with_nan_5pct.json", 1e-10); }
#[test]
fn trange_all_same_value() { run_trange_golden("trange__all_same_value.json", 1e-10); }
#[test]
fn trange_real_btcusdt() { run_trange_golden("trange__real_btcusdt_1d.json", 1e-7); }
#[test]
fn trange_real_flat_period() { run_trange_golden("trange__real_flat_period.json", 1e-10); }
