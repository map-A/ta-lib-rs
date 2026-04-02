//! Golden Test runner binary.
//!
//! Discovers all golden JSON files under `tests/golden/`, runs the corresponding
//! indicator, and prints a pass/fail report.
//!
//! Usage: `cargo run --package polars-ta-verify --bin run-golden`

use polars_ta_core::hilbert::{
    ht_dcperiod, ht_dcphase, ht_phasor, ht_sine, ht_trendline, ht_trendmode,
};
use polars_ta_core::math_ops::{add, div, max, maxindex, min, minindex, minmax, minmaxindex, mult, sub, sum};
use polars_ta_core::math_transform::{
    acos, asin, atan, ceil, cos, cosh, exp, floor, ln, log10, sin, sinh, sqrt, tan, tanh,
};
use polars_ta_core::oscillator::{
    adxr, apo, aroon, aroonosc, bop, cci, cmo, dx, mfi, minus_di, minus_dm, mom,
    plus_di, plus_dm, ppo, roc, rocp, rocr, rocr100, rsi, stoch, stochf, stochrsi, trix,
    ultosc, willr,
};
use polars_ta_core::pattern::{
    cdl2crows, cdl3blackcrows, cdl3inside, cdl3linestrike, cdl3outside, cdl3starsinsouth,
    cdl3whitesoldiers, cdlabandonedbaby, cdladvanceblock, cdlbelthold, cdlbreakaway,
    cdlclosingmarubozu, cdlconcealbabyswall, cdlcounterattack, cdldarkcloudcover, cdldoji,
    cdldojistar, cdldragonflydoji, cdlengulfing, cdleveningdojistar, cdleveningstar,
    cdlgapsidesidewhite, cdlgravestonedoji, cdlhammer, cdlhangingman, cdlharami,
    cdlharamicross, cdlhighwave, cdlhikkake, cdlhikkakemod, cdlhomingpigeon,
    cdlidentical3crows, cdlinneck, cdlinvertedhammer, cdlkicking, cdlkickingbylength,
    cdlladderbottom, cdllongleggeddoji, cdllongline, cdlmarubozu, cdlmatchinglow, cdlmathold,
    cdlmorningdojistar, cdlmorningstar, cdlonneck, cdlpiercing, cdlrickshawman,
    cdlrisefall3methods, cdlseparatinglines, cdlshootingstar, cdlshortline, cdlspinningtop,
    cdlstalledpattern, cdlsticksandwich, cdltakuri, cdltasukigap, cdlthrusting, cdltristar,
    cdlunique3river, cdlupsidegap2crows, cdlxsidegap3methods,
};
use polars_ta_core::price_transform::{avgprice, medprice, typprice, wclprice};
use polars_ta_core::statistic::{
    beta, correl, linearreg, linearreg_angle, linearreg_intercept, linearreg_slope, stddev, tsf,
    var,
};
use polars_ta_core::trend::{
    adx, bbands, dema, ema, kama, ma, macd, macdext, macdfix, mama, mavp, midpoint, midprice,
    sar, sarext, sma, t3, tema, trima, wma,
};
use polars_ta_core::volatility::{atr, natr, trange};
use polars_ta_core::volume::{ad, adosc, obv};
use polars_ta_verify::golden::{check_close_relative, load_golden_file, GoldenTestResult};
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

/// Use relative+absolute tolerance so BTC-scale floating-point drift doesn't fail tests.
/// Pass if |diff| <= 1e-5 (absolute, covers float cancellation) OR |diff|/|expected| <= 1e-8 (relative).
fn chk(actual: &[f64], golden: &[Option<f64>], label: &str) -> GoldenTestResult {
    check_close_relative(actual, golden, 1e-5, 1e-8, label)
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
            // ── Trend ─────────────────────────────────────────────────────────
            "sma" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let input = get_input!("close");
                chk(&sma(&input, period), get_output!("values"), &filename)
            }
            "ema" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let input = get_input!("close");
                chk(&ema(&input, period), get_output!("values"), &filename)
            }
            "wma" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let input = get_input!("close");
                chk(&wma(&input, period), get_output!("values"), &filename)
            }
            "dema" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let input = get_input!("close");
                chk(&dema(&input, period), get_output!("values"), &filename)
            }
            "tema" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let input = get_input!("close");
                chk(&tema(&input, period), get_output!("values"), &filename)
            }
            "kama" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                chk(&kama(&input, period), get_output!("values"), &filename)
            }
            "trima" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                chk(&trima(&input, period), get_output!("values"), &filename)
            }
            "t3" => {
                let period  = golden.meta.params["period"].as_u64().unwrap_or(5) as usize;
                let vfactor = golden.meta.params["vfactor"].as_f64().unwrap_or(0.7);
                let input = get_input!("close");
                chk(&t3(&input, period, vfactor), get_output!("values"), &filename)
            }
            "ma" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let matype = golden.meta.params["matype"].as_u64().unwrap_or(1) as usize;
                let input = get_input!("close");
                chk(&ma(&input, period, matype), get_output!("values"), &filename)
            }
            "midpoint" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                chk(&midpoint(&input, period), get_output!("values"), &filename)
            }
            "midprice" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high = get_input!("high");
                let low  = get_input!("low");
                chk(&midprice(&high, &low, period), get_output!("values"), &filename)
            }
            "macd" => {
                let fast   = golden.meta.params["fast"].as_u64().unwrap_or(12) as usize;
                let slow   = golden.meta.params["slow"].as_u64().unwrap_or(26) as usize;
                let signal = golden.meta.params["signal"].as_u64().unwrap_or(9) as usize;
                let input = get_input!("close");
                let out = macd(&input, fast, slow, signal);
                let r_macd   = chk(&out.macd, get_output!("macd"), &filename);
                let r_signal = chk(&out.signal, get_output!("signal"), &filename);
                let r_hist   = chk(&out.hist, get_output!("hist"), &filename);
                merge_results(&filename, &[r_macd, r_signal, r_hist])
            }
            "macdext" => {
                let fast   = golden.meta.params["fast"].as_u64().unwrap_or(12) as usize;
                let slow   = golden.meta.params["slow"].as_u64().unwrap_or(26) as usize;
                let signal = golden.meta.params["signal"].as_u64().unwrap_or(9) as usize;
                let matype = golden.meta.params["matype"].as_u64().unwrap_or(1) as usize;
                let input = get_input!("close");
                let out = macdext(&input, fast, matype, slow, matype, signal, matype);
                let r_macd   = chk(&out.macd, get_output!("macd"), &filename);
                let r_signal = chk(&out.signal, get_output!("signal"), &filename);
                let r_hist   = chk(&out.hist, get_output!("hist"), &filename);
                merge_results(&filename, &[r_macd, r_signal, r_hist])
            }
            "macdfix" => {
                let signal = golden.meta.params["signal"].as_u64().unwrap_or(9) as usize;
                let input = get_input!("close");
                let out = macdfix(&input, signal);
                let r_macd   = chk(&out.macd, get_output!("macd"), &filename);
                let r_signal = chk(&out.signal, get_output!("signal"), &filename);
                let r_hist   = chk(&out.hist, get_output!("hist"), &filename);
                merge_results(&filename, &[r_macd, r_signal, r_hist])
            }
            "bbands" => {
                let period  = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
                let nbdevup = golden.meta.params["nbdevup"].as_f64().unwrap_or(2.0);
                let nbdevdn = golden.meta.params["nbdevdn"].as_f64().unwrap_or(2.0);
                let input = get_input!("close");
                let out = bbands(&input, period, nbdevup, nbdevdn);
                let r_upper  = chk(&out.upper, get_output!("upper"), &filename);
                let r_middle = chk(&out.middle, get_output!("middle"), &filename);
                let r_lower  = chk(&out.lower, get_output!("lower"), &filename);
                merge_results(&filename, &[r_upper, r_middle, r_lower])
            }
            "sar" => {
                let acceleration = golden.meta.params["acceleration"].as_f64().unwrap_or(0.02);
                let maximum      = golden.meta.params["maximum"].as_f64().unwrap_or(0.2);
                let high = get_input!("high");
                let low  = get_input!("low");
                chk(&sar(&high, &low, acceleration, maximum), get_output!("values"), &filename)
            }
            "sarext" => {
                let high = get_input!("high");
                let low  = get_input!("low");
                chk(
                    &sarext(&high, &low, 0.0, 0.0, 0.01, 0.01, 0.20, 0.01, 0.01, 0.20),
                    get_output!("values"), &filename,
                )
            }
            "adx" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&adx(&high, &low, &close, period), get_output!("values"), &filename)
            }
            "mama" => {
                let fast_limit = golden.meta.params["fastlimit"].as_f64().unwrap_or(0.5);
                let slow_limit = golden.meta.params["slowlimit"].as_f64().unwrap_or(0.05);
                let input = get_input!("close");
                let out = mama(&input, fast_limit, slow_limit);
                let r_mama = chk(&out.mama, get_output!("mama"), &filename);
                let r_fama = chk(&out.fama, get_output!("fama"), &filename);
                merge_results(&filename, &[r_mama, r_fama])
            }
            "mavp" => {
                let min_period = golden.meta.params["minperiod"].as_u64().unwrap_or(2) as usize;
                let max_period = golden.meta.params["maxperiod"].as_u64().unwrap_or(30) as usize;
                let input   = get_input!("close");
                let periods = get_input!("periods");
                chk(&mavp(&input, &periods, min_period, max_period), get_output!("values"), &filename)
            }

            // ── Oscillator ────────────────────────────────────────────────────
            "rsi" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                chk(&rsi(&input, period), get_output!("values"), &filename)
            }
            "stoch" => {
                let fastk = golden.meta.params["fastk"].as_u64().unwrap_or(5) as usize;
                let slowk = golden.meta.params["slowk"].as_u64().unwrap_or(3) as usize;
                let slowd = golden.meta.params["slowd"].as_u64().unwrap_or(3) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                let out = stoch(&high, &low, &close, fastk, slowk, slowd);
                let r_slowk = chk(&out.slowk, get_output!("slowk"), &filename);
                let r_slowd = chk(&out.slowd, get_output!("slowd"), &filename);
                merge_results(&filename, &[r_slowk, r_slowd])
            }
            "stochf" => {
                let fastk = golden.meta.params["fastk"].as_u64().unwrap_or(5) as usize;
                let fastd = golden.meta.params["fastd"].as_u64().unwrap_or(3) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                let out = stochf(&high, &low, &close, fastk, fastd);
                let r_fastk = chk(&out.fastk, get_output!("fastk"), &filename);
                let r_fastd = chk(&out.fastd, get_output!("fastd"), &filename);
                merge_results(&filename, &[r_fastk, r_fastd])
            }
            "stochrsi" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let fastk  = golden.meta.params["fastk"].as_u64().unwrap_or(5) as usize;
                let fastd  = golden.meta.params["fastd"].as_u64().unwrap_or(3) as usize;
                let input = get_input!("close");
                let out = stochrsi(&input, period, fastk, fastd);
                let r_fastk = chk(&out.fastk, get_output!("fastk"), &filename);
                let r_fastd = chk(&out.fastd, get_output!("fastd"), &filename);
                merge_results(&filename, &[r_fastk, r_fastd])
            }
            "cci" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cci(&high, &low, &close, period), get_output!("values"), &filename)
            }
            "willr" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&willr(&high, &low, &close, period), get_output!("values"), &filename)
            }
            "ultosc" => {
                let period1 = golden.meta.params["period1"].as_u64().unwrap_or(7)  as usize;
                let period2 = golden.meta.params["period2"].as_u64().unwrap_or(14) as usize;
                let period3 = golden.meta.params["period3"].as_u64().unwrap_or(28) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&ultosc(&high, &low, &close, period1, period2, period3), get_output!("values"), &filename)
            }
            "aroon" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high = get_input!("high");
                let low  = get_input!("low");
                let out = aroon(&high, &low, period);
                let r_down = chk(&out.aroon_down, get_output!("aroon_down"), &filename);
                let r_up   = chk(&out.aroon_up, get_output!("aroon_up"), &filename);
                merge_results(&filename, &[r_down, r_up])
            }
            "aroonosc" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high = get_input!("high");
                let low  = get_input!("low");
                chk(&aroonosc(&high, &low, period), get_output!("values"), &filename)
            }
            "adxr" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&adxr(&high, &low, &close, period), get_output!("values"), &filename)
            }
            "apo" => {
                let fast = golden.meta.params["fast"].as_u64().unwrap_or(12) as usize;
                let slow = golden.meta.params["slow"].as_u64().unwrap_or(26) as usize;
                let input = get_input!("close");
                chk(&apo(&input, fast, slow), get_output!("values"), &filename)
            }
            "ppo" => {
                let fast = golden.meta.params["fast"].as_u64().unwrap_or(12) as usize;
                let slow = golden.meta.params["slow"].as_u64().unwrap_or(26) as usize;
                let input = get_input!("close");
                chk(&ppo(&input, fast, slow), get_output!("values"), &filename)
            }
            "bop" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&bop(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cmo" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                chk(&cmo(&input, period), get_output!("values"), &filename)
            }
            "dx" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&dx(&high, &low, &close, period), get_output!("values"), &filename)
            }
            "minus_di" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&minus_di(&high, &low, &close, period), get_output!("values"), &filename)
            }
            "minus_dm" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high = get_input!("high");
                let low  = get_input!("low");
                chk(&minus_dm(&high, &low, period), get_output!("values"), &filename)
            }
            "plus_di" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&plus_di(&high, &low, &close, period), get_output!("values"), &filename)
            }
            "plus_dm" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high = get_input!("high");
                let low  = get_input!("low");
                chk(&plus_dm(&high, &low, period), get_output!("values"), &filename)
            }
            "mfi" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high   = get_input!("high");
                let low    = get_input!("low");
                let close  = get_input!("close");
                let volume = get_input!("volume");
                chk(&mfi(&high, &low, &close, &volume, period), get_output!("values"), &filename)
            }
            "mom" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                chk(&mom(&input, period), get_output!("values"), &filename)
            }
            "roc" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                chk(&roc(&input, period), get_output!("values"), &filename)
            }
            "rocp" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                chk(&rocp(&input, period), get_output!("values"), &filename)
            }
            "rocr" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                chk(&rocr(&input, period), get_output!("values"), &filename)
            }
            "rocr100" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                chk(&rocr100(&input, period), get_output!("values"), &filename)
            }
            "trix" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(5) as usize;
                let input = get_input!("close");
                chk(&trix(&input, period), get_output!("values"), &filename)
            }

            // ── Volatility ────────────────────────────────────────────────────
            "trange" => {
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&trange(&high, &low, &close), get_output!("values"), &filename)
            }
            "atr" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&atr(&high, &low, &close, period), get_output!("values"), &filename)
            }
            "natr" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&natr(&high, &low, &close, period), get_output!("values"), &filename)
            }

            // ── Volume ────────────────────────────────────────────────────────
            "obv" => {
                let close  = get_input!("close");
                let volume = get_input!("volume");
                chk(&obv(&close, &volume), get_output!("values"), &filename)
            }
            "ad" => {
                let high   = get_input!("high");
                let low    = get_input!("low");
                let close  = get_input!("close");
                let volume = get_input!("volume");
                chk(&ad(&high, &low, &close, &volume), get_output!("values"), &filename)
            }
            "adosc" => {
                let fast = golden.meta.params["fast"].as_u64().unwrap_or(3)  as usize;
                let slow = golden.meta.params["slow"].as_u64().unwrap_or(10) as usize;
                let high   = get_input!("high");
                let low    = get_input!("low");
                let close  = get_input!("close");
                let volume = get_input!("volume");
                chk(&adosc(&high, &low, &close, &volume, fast, slow), get_output!("values"), &filename)
            }

            // ── Statistic ─────────────────────────────────────────────────────
            "beta" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(5) as usize;
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                chk(&beta(&real0, &real1, period), get_output!("values"), &filename)
            }
            "correl" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                chk(&correl(&real0, &real1, period), get_output!("values"), &filename)
            }
            "linearreg" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                chk(&linearreg(&input, period), get_output!("values"), &filename)
            }
            "linearreg_angle" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                chk(&linearreg_angle(&input, period), get_output!("values"), &filename)
            }
            "linearreg_intercept" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                chk(&linearreg_intercept(&input, period), get_output!("values"), &filename)
            }
            "linearreg_slope" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                chk(&linearreg_slope(&input, period), get_output!("values"), &filename)
            }
            "stddev" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(5) as usize;
                let nbdev  = golden.meta.params["nbdev"].as_f64().unwrap_or(1.0);
                let input = get_input!("close");
                chk(&stddev(&input, period, nbdev), get_output!("values"), &filename)
            }
            "var" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(5) as usize;
                let nbdev  = golden.meta.params["nbdev"].as_f64().unwrap_or(1.0);
                let input = get_input!("close");
                chk(&var(&input, period, nbdev), get_output!("values"), &filename)
            }
            "tsf" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                chk(&tsf(&input, period), get_output!("values"), &filename)
            }

            // ── Price Transform ───────────────────────────────────────────────
            "avgprice" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&avgprice(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "medprice" => {
                let high = get_input!("high");
                let low  = get_input!("low");
                chk(&medprice(&high, &low), get_output!("values"), &filename)
            }
            "typprice" => {
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&typprice(&high, &low, &close), get_output!("values"), &filename)
            }
            "wclprice" => {
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&wclprice(&high, &low, &close), get_output!("values"), &filename)
            }

            // ── Math Transform ────────────────────────────────────────────────
            "acos" => {
                let input = get_input!("close");
                chk(&acos(&input), get_output!("values"), &filename)
            }
            "asin" => {
                let input = get_input!("close");
                chk(&asin(&input), get_output!("values"), &filename)
            }
            "atan" => {
                let input = get_input!("close");
                chk(&atan(&input), get_output!("values"), &filename)
            }
            "ceil" => {
                let input = get_input!("close");
                chk(&ceil(&input), get_output!("values"), &filename)
            }
            "cos" => {
                let input = get_input!("close");
                chk(&cos(&input), get_output!("values"), &filename)
            }
            "cosh" => {
                let input = get_input!("close");
                chk(&cosh(&input), get_output!("values"), &filename)
            }
            "exp" => {
                let input = get_input!("close");
                chk(&exp(&input), get_output!("values"), &filename)
            }
            "floor" => {
                let input = get_input!("close");
                chk(&floor(&input), get_output!("values"), &filename)
            }
            "ln" => {
                let input = get_input!("close");
                chk(&ln(&input), get_output!("values"), &filename)
            }
            "log10" => {
                let input = get_input!("close");
                chk(&log10(&input), get_output!("values"), &filename)
            }
            "sin" => {
                let input = get_input!("close");
                chk(&sin(&input), get_output!("values"), &filename)
            }
            "sinh" => {
                let input = get_input!("close");
                chk(&sinh(&input), get_output!("values"), &filename)
            }
            "sqrt" => {
                let input = get_input!("close");
                chk(&sqrt(&input), get_output!("values"), &filename)
            }
            "tan" => {
                let input = get_input!("close");
                chk(&tan(&input), get_output!("values"), &filename)
            }
            "tanh" => {
                let input = get_input!("close");
                chk(&tanh(&input), get_output!("values"), &filename)
            }

            // ── Math Ops ──────────────────────────────────────────────────────
            "add" => {
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                chk(&add(&real0, &real1), get_output!("values"), &filename)
            }
            "sub" => {
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                chk(&sub(&real0, &real1), get_output!("values"), &filename)
            }
            "mult" => {
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                chk(&mult(&real0, &real1), get_output!("values"), &filename)
            }
            "div" => {
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                chk(&div(&real0, &real1), get_output!("values"), &filename)
            }
            "max" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                chk(&max(&input, period), get_output!("values"), &filename)
            }
            "min" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                chk(&min(&input, period), get_output!("values"), &filename)
            }
            "sum" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                chk(&sum(&input, period), get_output!("values"), &filename)
            }
            "maxindex" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                chk(&maxindex(&input, period), get_output!("values"), &filename)
            }
            "minindex" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                chk(&minindex(&input, period), get_output!("values"), &filename)
            }
            "minmax" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                let out = minmax(&input, period);
                let r_min = chk(&out.min, get_output!("min"), &filename);
                let r_max = chk(&out.max, get_output!("max"), &filename);
                merge_results(&filename, &[r_min, r_max])
            }
            "minmaxindex" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                let out = minmaxindex(&input, period);
                let r_min = chk(&out.minidx, get_output!("minidx"), &filename);
                let r_max = chk(&out.maxidx, get_output!("maxidx"), &filename);
                merge_results(&filename, &[r_min, r_max])
            }

            // ── Hilbert Transform ─────────────────────────────────────────────
            "ht_dcperiod" => {
                let input = get_input!("close");
                chk(&ht_dcperiod(&input), get_output!("values"), &filename)
            }
            "ht_dcphase" => {
                let input = get_input!("close");
                chk(&ht_dcphase(&input), get_output!("values"), &filename)
            }
            "ht_phasor" => {
                let input = get_input!("close");
                let (inphase, quadrature) = ht_phasor(&input);
                let r_ip = chk(&inphase,    get_output!("inphase"),    &filename);
                let r_q  = chk(&quadrature, get_output!("quadrature"), &filename);
                merge_results(&filename, &[r_ip, r_q])
            }
            "ht_sine" => {
                let input = get_input!("close");
                let (sine, leadsine) = ht_sine(&input);
                let r_s  = chk(&sine,     get_output!("sine"),     &filename);
                let r_ls = chk(&leadsine, get_output!("leadsine"), &filename);
                merge_results(&filename, &[r_s, r_ls])
            }
            "ht_trendline" => {
                let input = get_input!("close");
                chk(&ht_trendline(&input), get_output!("values"), &filename)
            }
            "ht_trendmode" => {
                let input = get_input!("close");
                chk(&ht_trendmode(&input), get_output!("values"), &filename)
            }

            // ── Candlestick Patterns ──────────────────────────────────────────
            "cdl2crows" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdl2crows(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdl3blackcrows" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdl3blackcrows(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdl3inside" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdl3inside(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdl3linestrike" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdl3linestrike(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdl3outside" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdl3outside(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdl3starsinsouth" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdl3starsinsouth(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdl3whitesoldiers" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdl3whitesoldiers(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlabandonedbaby" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlabandonedbaby(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdladvanceblock" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdladvanceblock(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlbelthold" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlbelthold(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlbreakaway" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlbreakaway(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlclosingmarubozu" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlclosingmarubozu(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlconcealbabyswall" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlconcealbabyswall(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlcounterattack" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlcounterattack(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdldarkcloudcover" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdldarkcloudcover(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdldoji" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdldoji(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdldojistar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdldojistar(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdldragonflydoji" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdldragonflydoji(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlengulfing" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlengulfing(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdleveningdojistar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdleveningdojistar(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdleveningstar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdleveningstar(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlgapsidesidewhite" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlgapsidesidewhite(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlgravestonedoji" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlgravestonedoji(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlhammer" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlhammer(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlhangingman" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlhangingman(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlharami" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlharami(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlharamicross" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlharamicross(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlhighwave" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlhighwave(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlhikkake" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlhikkake(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlhikkakemod" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlhikkakemod(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlhomingpigeon" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlhomingpigeon(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlidentical3crows" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlidentical3crows(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlinneck" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlinneck(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlinvertedhammer" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlinvertedhammer(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlkicking" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlkicking(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlkickingbylength" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlkickingbylength(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlladderbottom" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlladderbottom(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdllongleggeddoji" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdllongleggeddoji(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdllongline" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdllongline(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlmarubozu" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlmarubozu(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlmatchinglow" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlmatchinglow(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlmathold" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlmathold(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlmorningdojistar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlmorningdojistar(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlmorningstar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlmorningstar(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlonneck" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlonneck(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlpiercing" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlpiercing(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlrickshawman" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlrickshawman(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlrisefall3methods" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlrisefall3methods(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlseparatinglines" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlseparatinglines(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlshootingstar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlshootingstar(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlshortline" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlshortline(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlspinningtop" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlspinningtop(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlstalledpattern" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlstalledpattern(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlsticksandwich" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlsticksandwich(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdltakuri" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdltakuri(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdltasukigap" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdltasukigap(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlthrusting" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlthrusting(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdltristar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdltristar(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlunique3river" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlunique3river(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlupsidegap2crows" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlupsidegap2crows(&open, &high, &low, &close), get_output!("values"), &filename)
            }
            "cdlxsidegap3methods" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                chk(&cdlxsidegap3methods(&open, &high, &low, &close), get_output!("values"), &filename)
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
