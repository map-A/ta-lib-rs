//! Golden tests for CDL3WHITESOLDIERS.
use polars_ta_core::pattern::cdl3whitesoldiers;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_cdl3whitesoldiers_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let open  = golden.get_input("open").unwrap();
    let high  = golden.get_input("high").unwrap();
    let low   = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = cdl3whitesoldiers(&open, &high, &low, &close);
    let label = format!("cdl3whitesoldiers/{}", golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn cdl3whitesoldiers_normal_1000() { run_cdl3whitesoldiers_golden("cdl3whitesoldiers__normal_1000.json", 1.0); }
