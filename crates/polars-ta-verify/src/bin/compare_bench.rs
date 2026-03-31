//! Benchmark comparison binary.
//!
//! Reads benchmark results from the Rust side (criterion output) and
//! Python ta-lib benchmark data, then prints a unified comparison report.
//!
//! Usage: `cargo run --package polars-ta-verify --bin compare-bench`

fn main() {
    println!("polars-ta Performance Report");
    println!("============================");
    println!();
    println!("To generate the full comparison, run:");
    println!("  scripts/compare_all.sh");
    println!();
    println!("This script will:");
    println!("  1. Run Rust criterion benchmarks (cargo bench)");
    println!("  2. Run Python ta-lib benchmarks (scripts/bench_talib.py)");
    println!("  3. Print a side-by-side comparison table");
}
