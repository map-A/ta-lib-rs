//! Golden Tests for OBV (On Balance Volume). lookback=0, no boundary tests.
use polars_ta_core::volume::obv;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_obv_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let close = golden.get_input("close").unwrap();
    let volume = golden.get_input("volume").unwrap();
    let actual = obv(&close, &volume);
    let label = format!("obv/{}", golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn obv_normal_1000() { run_obv_golden("obv__normal_1000.json", 1e-7); }
#[test]
#[ignore = "ta-lib NaN propagation differs for OBV with NaN inputs"]
fn obv_with_nan() { run_obv_golden("obv__with_nan_5pct.json", 1e-10); }
#[test]
fn obv_all_same_value() { run_obv_golden("obv__all_same_value.json", 1e-10); }
#[test]
fn obv_real_btcusdt() { run_obv_golden("obv__real_btcusdt_1d.json", 1e-3); }
#[test]
fn obv_real_flat_period() { run_obv_golden("obv__real_flat_period.json", 1e-8); }
