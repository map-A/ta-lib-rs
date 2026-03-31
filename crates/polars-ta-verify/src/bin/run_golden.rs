//! Golden Test runner binary.
//!
//! Discovers all golden JSON files under `tests/golden/`, runs the corresponding
//! indicator, and prints a pass/fail report.
//!
//! Usage: `cargo run --package polars-ta-verify --bin run-golden`

use polars_ta_core::oscillator::{aroon, cci, mfi, rsi, stoch, stochrsi, ultosc, willr};
use polars_ta_core::trend::{adx, bbands, dema, ema, macd, sar, sma, tema, wma};
use polars_ta_core::volatility::{atr, natr, trange};
use polars_ta_core::volume::{ad, adosc, obv};
use polars_ta_verify::golden::{check_close, load_golden_file, GoldenTestResult};
use std::path::Path;

/// 合并多个输出的结果，取最差情况（任意一个失败则整体失败）。
fn merge_results(label: &str, results: &[GoldenTestResult]) -> GoldenTestResult {
    let passed = results.iter().all(|r| r.passed);
    let max_diff = results.iter().map(|r| r.max_diff).fold(0.0f64, f64::max);
    let failure_count = results.iter().map(|r| r.failure_count).sum();
    let actual_len = results.first().map(|r| r.actual_len).unwrap_or(0);
    let expected_len = results.first().map(|r| r.expected_len).unwrap_or(0);
    GoldenTestResult { label: label.to_string(), passed, max_diff, failure_count, actual_len, expected_len }
}

fn main() {
    let golden_dir = Path::new("tests/golden");
    if !golden_dir.exists() {
        eprintln!("ERROR: tests/golden/ directory not found.");
        eprintln!("Run `scripts/generate_golden.py` first to generate golden files.");
        std::process::exit(1);
    }

    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;

    // 遍历所有 golden JSON 文件
    let mut entries: Vec<_> = std::fs::read_dir(golden_dir)
        .expect("cannot read tests/golden/")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in &entries {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy();

        let golden = match load_golden_file(&path) {
            Ok(g) => g,
            Err(e) => {
                println!("  SKIP  {filename}: {e}");
                skipped += 1;
                continue;
            }
        };

        // マクロ: 単一入力列を取得し、失敗したら skip
        macro_rules! get_input {
            ($col:expr) => {
                match golden.get_input($col) {
                    Ok(v) => v,
                    Err(e) => { println!("  SKIP  {filename}: {e}"); skipped += 1; continue; }
                }
            };
        }
        // マクロ: 出力列を取得し、失敗したら skip
        macro_rules! get_output {
            ($key:expr) => {
                match golden.get_output_values($key) {
                    Ok(v) => v,
                    Err(e) => { println!("  SKIP  {filename}: {e}"); skipped += 1; continue; }
                }
            };
        }

        let result = match golden.meta.indicator.as_str() {
            "sma" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let input = get_input!("close");
                check_close(&sma(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "ema" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let input = get_input!("close");
                check_close(&ema(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "wma" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let input = get_input!("close");
                check_close(&wma(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "dema" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let input = get_input!("close");
                check_close(&dema(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "tema" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let input = get_input!("close");
                check_close(&tema(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "macd" => {
                let fast = golden.meta.params["fast"].as_u64().unwrap_or(12) as usize;
                let slow = golden.meta.params["slow"].as_u64().unwrap_or(26) as usize;
                let signal = golden.meta.params["signal"].as_u64().unwrap_or(9) as usize;
                let input = get_input!("close");
                let out = macd(&input, fast, slow, signal);
                let r_macd   = check_close(&out.macd,   get_output!("macd"),   1e-10, &filename);
                let r_signal = check_close(&out.signal, get_output!("signal"), 1e-10, &filename);
                let r_hist   = check_close(&out.hist,   get_output!("hist"),   1e-10, &filename);
                merge_results(&filename, &[r_macd, r_signal, r_hist])
            }
            "bbands" => {
                let period  = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let nbdevup = golden.meta.params["nbdevup"].as_f64().unwrap_or(2.0);
                let nbdevdn = golden.meta.params["nbdevdn"].as_f64().unwrap_or(2.0);
                let input = get_input!("close");
                let out = bbands(&input, period, nbdevup, nbdevdn);
                let r_upper  = check_close(&out.upper,  get_output!("upper"),  1e-10, &filename);
                let r_middle = check_close(&out.middle, get_output!("middle"), 1e-10, &filename);
                let r_lower  = check_close(&out.lower,  get_output!("lower"),  1e-10, &filename);
                merge_results(&filename, &[r_upper, r_middle, r_lower])
            }
            "sar" => {
                let acceleration = golden.meta.params["acceleration"].as_f64().unwrap_or(0.02);
                let maximum      = golden.meta.params["maximum"].as_f64().unwrap_or(0.2);
                let high  = get_input!("high");
                let low   = get_input!("low");
                check_close(&sar(&high, &low, acceleration, maximum), get_output!("values"), 1e-10, &filename)
            }
            "adx" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&adx(&high, &low, &close, period), get_output!("values"), 1e-10, &filename)
            }
            "rsi" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                check_close(&rsi(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "stoch" => {
                let fastk = golden.meta.params["fastk"].as_u64().unwrap_or(5) as usize;
                let slowk = golden.meta.params["slowk"].as_u64().unwrap_or(3) as usize;
                let slowd = golden.meta.params["slowd"].as_u64().unwrap_or(3) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                let out = stoch(&high, &low, &close, fastk, slowk, slowd);
                let r_slowk = check_close(&out.slowk, get_output!("slowk"), 1e-10, &filename);
                let r_slowd = check_close(&out.slowd, get_output!("slowd"), 1e-10, &filename);
                merge_results(&filename, &[r_slowk, r_slowd])
            }
            "stochrsi" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let fastk  = golden.meta.params["fastk"].as_u64().unwrap_or(5) as usize;
                let fastd  = golden.meta.params["fastd"].as_u64().unwrap_or(3) as usize;
                let input = get_input!("close");
                let out = stochrsi(&input, period, fastk, fastd);
                let r_fastk = check_close(&out.fastk, get_output!("fastk"), 1e-10, &filename);
                let r_fastd = check_close(&out.fastd, get_output!("fastd"), 1e-10, &filename);
                merge_results(&filename, &[r_fastk, r_fastd])
            }
            "cci" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cci(&high, &low, &close, period), get_output!("values"), 1e-10, &filename)
            }
            "willr" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&willr(&high, &low, &close, period), get_output!("values"), 1e-10, &filename)
            }
            "ultosc" => {
                let period1 = golden.meta.params["period1"].as_u64().unwrap_or(7)  as usize;
                let period2 = golden.meta.params["period2"].as_u64().unwrap_or(14) as usize;
                let period3 = golden.meta.params["period3"].as_u64().unwrap_or(28) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&ultosc(&high, &low, &close, period1, period2, period3), get_output!("values"), 1e-10, &filename)
            }
            "aroon" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high = get_input!("high");
                let low  = get_input!("low");
                let out = aroon(&high, &low, period);
                let r_down = check_close(&out.aroon_down, get_output!("aroon_down"), 1e-10, &filename);
                let r_up   = check_close(&out.aroon_up,   get_output!("aroon_up"),   1e-10, &filename);
                merge_results(&filename, &[r_down, r_up])
            }
            "mfi" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high   = get_input!("high");
                let low    = get_input!("low");
                let close  = get_input!("close");
                let volume = get_input!("volume");
                check_close(&mfi(&high, &low, &close, &volume, period), get_output!("values"), 1e-10, &filename)
            }
            "obv" => {
                let close  = get_input!("close");
                let volume = get_input!("volume");
                check_close(&obv(&close, &volume), get_output!("values"), 1e-10, &filename)
            }
            "ad" => {
                let high   = get_input!("high");
                let low    = get_input!("low");
                let close  = get_input!("close");
                let volume = get_input!("volume");
                check_close(&ad(&high, &low, &close, &volume), get_output!("values"), 1e-10, &filename)
            }
            "adosc" => {
                let fast = golden.meta.params["fast"].as_u64().unwrap_or(3)  as usize;
                let slow = golden.meta.params["slow"].as_u64().unwrap_or(10) as usize;
                let high   = get_input!("high");
                let low    = get_input!("low");
                let close  = get_input!("close");
                let volume = get_input!("volume");
                check_close(&adosc(&high, &low, &close, &volume, fast, slow), get_output!("values"), 1e-10, &filename)
            }
            "trange" => {
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&trange(&high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "atr" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&atr(&high, &low, &close, period), get_output!("values"), 1e-10, &filename)
            }
            "natr" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&natr(&high, &low, &close, period), get_output!("values"), 1e-10, &filename)
            }
            other => {
                println!("  SKIP  {filename}: indicator '{other}' not yet implemented in runner");
                skipped += 1;
                continue;
            }
        };

        if result.passed {
            println!("  PASS  {} (max_diff={:.2e})", filename, result.max_diff);
            passed += 1;
        } else {
            println!(
                "  FAIL  {} ({} failures, max_diff={:.2e}, len actual={} expected={})",
                filename, result.failure_count, result.max_diff,
                result.actual_len, result.expected_len
            );
            failed += 1;
        }
    }

    println!();
    println!("Golden Test Report");
    println!("==================");
    println!("  PASSED:  {passed}");
    println!("  FAILED:  {failed}");
    println!("  SKIPPED: {skipped}");
    println!("  TOTAL:   {}", passed + failed + skipped);

    if failed > 0 {
        std::process::exit(1);
    }
}
