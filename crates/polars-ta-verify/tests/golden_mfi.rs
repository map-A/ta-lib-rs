//! Golden Tests for MFI (Money Flow Index).
use polars_ta_core::oscillator::mfi;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_mfi_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let high = golden.get_input("high").unwrap();
    let low = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let volume = golden.get_input("volume").unwrap();
    let actual = mfi(&high, &low, &close, &volume, period);
    let label = format!("mfi(period={})/{}", period, golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn mfi_period14_normal_1000() { run_mfi_golden("mfi_period14_normal_1000.json", 14, 1e-10); }
#[test]
fn mfi_period14_boundary_exact() { run_mfi_golden("mfi_period14_boundary_exact.json", 14, 1e-10); }
#[test]
fn mfi_period14_boundary_short() {
    let data = vec![1.0f64; 14];
    assert!(mfi(&data, &data, &data, &data, 14).is_empty());
}
#[test]
fn mfi_period14_with_nan() { run_mfi_golden("mfi_period14_with_nan_5pct.json", 14, 1e-10); }
#[test]
fn mfi_period14_all_same_value() { run_mfi_golden("mfi_period14_all_same_value.json", 14, 1e-10); }
#[test]
fn mfi_period14_real_btcusdt() { run_mfi_golden("mfi_period14_real_btcusdt_1d.json", 14, 1e-7); }
#[test]
fn mfi_period14_real_flat_period() { run_mfi_golden("mfi_period14_real_flat_period.json", 14, 1e-10); }
