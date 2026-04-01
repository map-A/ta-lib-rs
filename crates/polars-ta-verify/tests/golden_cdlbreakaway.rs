//! Golden tests for CDLBREAKAWAY.
use polars_ta_core::pattern::cdlbreakaway;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_cdlbreakaway_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let open  = golden.get_input("open").unwrap();
    let high  = golden.get_input("high").unwrap();
    let low   = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = cdlbreakaway(&open, &high, &low, &close);
    let label = format!("cdlbreakaway/{}", golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn cdlbreakaway_normal_1000() { run_cdlbreakaway_golden("cdlbreakaway__normal_1000.json", 1.0); }
