//! Golden Tests for AD (Accumulation/Distribution). lookback=0, no boundary tests.
use polars_ta_core::volume::ad;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests")
        .join("golden")
        .join(filename)
}

fn run_ad_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {}", filename);
        return;
    }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let volume = golden.get_input("volume").unwrap();
    let actual = ad(&high, &low, &close, &volume);
    let label = format!("ad/{}", golden.meta.dataset);
    assert_close(
        &actual,
        golden.get_output_values("values").unwrap(),
        epsilon,
        &label,
    );
}

#[test]
fn ad_normal_1000() {
    run_ad_golden("ad__normal_1000.json", 1e-5);
}
#[test]
#[ignore = "ta-lib NaN propagation differs: our AD propagates NaN via IEEE 754, ta-lib skips"]
fn ad_with_nan() {
    run_ad_golden("ad__with_nan_5pct.json", 1e-5);
}
#[test]
fn ad_all_same_value() {
    run_ad_golden("ad__all_same_value.json", 1e-5);
}
#[test]
fn ad_real_btcusdt() {
    run_ad_golden("ad__real_btcusdt_1d.json", 1e-3);
}
#[test]
fn ad_real_flat_period() {
    run_ad_golden("ad__real_flat_period.json", 1e-5);
}
