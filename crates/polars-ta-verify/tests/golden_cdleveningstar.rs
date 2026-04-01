//! Golden tests for CDLEVENINGSTAR.
use polars_ta_core::pattern::cdleveningstar;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_cdleveningstar_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let open  = golden.get_input("open").unwrap();
    let high  = golden.get_input("high").unwrap();
    let low   = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = cdleveningstar(&open, &high, &low, &close);
    let label = format!("cdleveningstar/{}", golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn cdleveningstar_normal_1000() { run_cdleveningstar_golden("cdleveningstar__normal_1000.json", 1.0); }
