//! Golden tests for CDLMORNINGDOJISTAR.
use polars_ta_core::pattern::cdlmorningdojistar;
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_cdlmorningdojistar_golden(filename: &str, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() { println!("SKIP: {}", filename); return; }
    let golden = load_golden_file(&path).unwrap_or_else(|e| panic!("{}: {}", filename, e));
    let open  = golden.get_input("open").unwrap();
    let high  = golden.get_input("high").unwrap();
    let low   = golden.get_input("low").unwrap();
    let close = golden.get_input("close").unwrap();
    let actual = cdlmorningdojistar(&open, &high, &low, &close);
    let label = format!("cdlmorningdojistar/{}", golden.meta.dataset);
    assert_close(&actual, golden.get_output_values("values").unwrap(), epsilon, &label);
}

#[test]
fn cdlmorningdojistar_normal_1000() { run_cdlmorningdojistar_golden("cdlmorningdojistar__normal_1000.json", 1.0); }
