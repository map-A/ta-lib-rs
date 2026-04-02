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
            // ── Trend ─────────────────────────────────────────────────────────
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
            "kama" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                check_close(&kama(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "trima" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                check_close(&trima(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "t3" => {
                let period  = golden.meta.params["period"].as_u64().unwrap_or(5) as usize;
                let vfactor = golden.meta.params["vfactor"].as_f64().unwrap_or(0.7);
                let input = get_input!("close");
                check_close(&t3(&input, period, vfactor), get_output!("values"), 1e-10, &filename)
            }
            "ma" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let matype = golden.meta.params["matype"].as_u64().unwrap_or(1) as usize;
                let input = get_input!("close");
                check_close(&ma(&input, period, matype), get_output!("values"), 1e-10, &filename)
            }
            "midpoint" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                check_close(&midpoint(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "midprice" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high = get_input!("high");
                let low  = get_input!("low");
                check_close(&midprice(&high, &low, period), get_output!("values"), 1e-10, &filename)
            }
            "macd" => {
                let fast   = golden.meta.params["fast"].as_u64().unwrap_or(12) as usize;
                let slow   = golden.meta.params["slow"].as_u64().unwrap_or(26) as usize;
                let signal = golden.meta.params["signal"].as_u64().unwrap_or(9) as usize;
                let input = get_input!("close");
                let out = macd(&input, fast, slow, signal);
                let r_macd   = check_close(&out.macd,   get_output!("macd"),   1e-10, &filename);
                let r_signal = check_close(&out.signal, get_output!("signal"), 1e-10, &filename);
                let r_hist   = check_close(&out.hist,   get_output!("hist"),   1e-10, &filename);
                merge_results(&filename, &[r_macd, r_signal, r_hist])
            }
            "macdext" => {
                let fast   = golden.meta.params["fast"].as_u64().unwrap_or(12) as usize;
                let slow   = golden.meta.params["slow"].as_u64().unwrap_or(26) as usize;
                let signal = golden.meta.params["signal"].as_u64().unwrap_or(9) as usize;
                let matype = golden.meta.params["matype"].as_u64().unwrap_or(1) as usize;
                let input = get_input!("close");
                let out = macdext(&input, fast, matype, slow, matype, signal, matype);
                let r_macd   = check_close(&out.macd,   get_output!("macd"),   1e-10, &filename);
                let r_signal = check_close(&out.signal, get_output!("signal"), 1e-10, &filename);
                let r_hist   = check_close(&out.hist,   get_output!("hist"),   1e-10, &filename);
                merge_results(&filename, &[r_macd, r_signal, r_hist])
            }
            "macdfix" => {
                let signal = golden.meta.params["signal"].as_u64().unwrap_or(9) as usize;
                let input = get_input!("close");
                let out = macdfix(&input, signal);
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
                let high = get_input!("high");
                let low  = get_input!("low");
                check_close(&sar(&high, &low, acceleration, maximum), get_output!("values"), 1e-10, &filename)
            }
            "sarext" => {
                let high = get_input!("high");
                let low  = get_input!("low");
                check_close(
                    &sarext(&high, &low, 0.0, 0.0, 0.01, 0.01, 0.20, 0.01, 0.01, 0.20),
                    get_output!("values"), 1e-10, &filename,
                )
            }
            "adx" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&adx(&high, &low, &close, period), get_output!("values"), 1e-10, &filename)
            }
            "mama" => {
                let fast_limit = golden.meta.params["fastlimit"].as_f64().unwrap_or(0.5);
                let slow_limit = golden.meta.params["slowlimit"].as_f64().unwrap_or(0.05);
                let input = get_input!("close");
                let out = mama(&input, fast_limit, slow_limit);
                let r_mama = check_close(&out.mama, get_output!("mama"), 1e-10, &filename);
                let r_fama = check_close(&out.fama, get_output!("fama"), 1e-10, &filename);
                merge_results(&filename, &[r_mama, r_fama])
            }
            "mavp" => {
                let min_period = golden.meta.params["minperiod"].as_u64().unwrap_or(2) as usize;
                let max_period = golden.meta.params["maxperiod"].as_u64().unwrap_or(30) as usize;
                let input   = get_input!("close");
                let periods = get_input!("periods");
                check_close(&mavp(&input, &periods, min_period, max_period), get_output!("values"), 1e-10, &filename)
            }

            // ── Oscillator ────────────────────────────────────────────────────
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
            "stochf" => {
                let fastk = golden.meta.params["fastk"].as_u64().unwrap_or(5) as usize;
                let fastd = golden.meta.params["fastd"].as_u64().unwrap_or(3) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                let out = stochf(&high, &low, &close, fastk, fastd);
                let r_fastk = check_close(&out.fastk, get_output!("fastk"), 1e-10, &filename);
                let r_fastd = check_close(&out.fastd, get_output!("fastd"), 1e-10, &filename);
                merge_results(&filename, &[r_fastk, r_fastd])
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
            "aroonosc" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high = get_input!("high");
                let low  = get_input!("low");
                check_close(&aroonosc(&high, &low, period), get_output!("values"), 1e-10, &filename)
            }
            "adxr" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&adxr(&high, &low, &close, period), get_output!("values"), 1e-10, &filename)
            }
            "apo" => {
                let fast = golden.meta.params["fast"].as_u64().unwrap_or(12) as usize;
                let slow = golden.meta.params["slow"].as_u64().unwrap_or(26) as usize;
                let input = get_input!("close");
                check_close(&apo(&input, fast, slow), get_output!("values"), 1e-10, &filename)
            }
            "ppo" => {
                let fast = golden.meta.params["fast"].as_u64().unwrap_or(12) as usize;
                let slow = golden.meta.params["slow"].as_u64().unwrap_or(26) as usize;
                let input = get_input!("close");
                check_close(&ppo(&input, fast, slow), get_output!("values"), 1e-10, &filename)
            }
            "bop" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&bop(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cmo" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                check_close(&cmo(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "dx" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&dx(&high, &low, &close, period), get_output!("values"), 1e-10, &filename)
            }
            "minus_di" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&minus_di(&high, &low, &close, period), get_output!("values"), 1e-10, &filename)
            }
            "minus_dm" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high = get_input!("high");
                let low  = get_input!("low");
                check_close(&minus_dm(&high, &low, period), get_output!("values"), 1e-10, &filename)
            }
            "plus_di" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&plus_di(&high, &low, &close, period), get_output!("values"), 1e-10, &filename)
            }
            "plus_dm" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high = get_input!("high");
                let low  = get_input!("low");
                check_close(&plus_dm(&high, &low, period), get_output!("values"), 1e-10, &filename)
            }
            "mfi" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let high   = get_input!("high");
                let low    = get_input!("low");
                let close  = get_input!("close");
                let volume = get_input!("volume");
                check_close(&mfi(&high, &low, &close, &volume, period), get_output!("values"), 1e-10, &filename)
            }
            "mom" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                check_close(&mom(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "roc" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                check_close(&roc(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "rocp" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                check_close(&rocp(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "rocr" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                check_close(&rocr(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "rocr100" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(10) as usize;
                let input = get_input!("close");
                check_close(&rocr100(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "trix" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(5) as usize;
                let input = get_input!("close");
                check_close(&trix(&input, period), get_output!("values"), 1e-10, &filename)
            }

            // ── Volatility ────────────────────────────────────────────────────
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

            // ── Volume ────────────────────────────────────────────────────────
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

            // ── Statistic ─────────────────────────────────────────────────────
            "beta" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(5) as usize;
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                check_close(&beta(&real0, &real1, period), get_output!("values"), 1e-10, &filename)
            }
            "correl" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                check_close(&correl(&real0, &real1, period), get_output!("values"), 1e-10, &filename)
            }
            "linearreg" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                check_close(&linearreg(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "linearreg_angle" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                check_close(&linearreg_angle(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "linearreg_intercept" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                check_close(&linearreg_intercept(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "linearreg_slope" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                check_close(&linearreg_slope(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "stddev" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(5) as usize;
                let nbdev  = golden.meta.params["nbdev"].as_f64().unwrap_or(1.0);
                let input = get_input!("close");
                check_close(&stddev(&input, period, nbdev), get_output!("values"), 1e-10, &filename)
            }
            "var" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(5) as usize;
                let nbdev  = golden.meta.params["nbdev"].as_f64().unwrap_or(1.0);
                let input = get_input!("close");
                check_close(&var(&input, period, nbdev), get_output!("values"), 1e-10, &filename)
            }
            "tsf" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(14) as usize;
                let input = get_input!("close");
                check_close(&tsf(&input, period), get_output!("values"), 1e-10, &filename)
            }

            // ── Price Transform ───────────────────────────────────────────────
            "avgprice" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&avgprice(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "medprice" => {
                let high = get_input!("high");
                let low  = get_input!("low");
                check_close(&medprice(&high, &low), get_output!("values"), 1e-10, &filename)
            }
            "typprice" => {
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&typprice(&high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "wclprice" => {
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&wclprice(&high, &low, &close), get_output!("values"), 1e-10, &filename)
            }

            // ── Math Transform ────────────────────────────────────────────────
            "acos" => {
                let input = get_input!("close");
                check_close(&acos(&input), get_output!("values"), 1e-10, &filename)
            }
            "asin" => {
                let input = get_input!("close");
                check_close(&asin(&input), get_output!("values"), 1e-10, &filename)
            }
            "atan" => {
                let input = get_input!("close");
                check_close(&atan(&input), get_output!("values"), 1e-10, &filename)
            }
            "ceil" => {
                let input = get_input!("close");
                check_close(&ceil(&input), get_output!("values"), 1e-10, &filename)
            }
            "cos" => {
                let input = get_input!("close");
                check_close(&cos(&input), get_output!("values"), 1e-10, &filename)
            }
            "cosh" => {
                let input = get_input!("close");
                check_close(&cosh(&input), get_output!("values"), 1e-10, &filename)
            }
            "exp" => {
                let input = get_input!("close");
                check_close(&exp(&input), get_output!("values"), 1e-10, &filename)
            }
            "floor" => {
                let input = get_input!("close");
                check_close(&floor(&input), get_output!("values"), 1e-10, &filename)
            }
            "ln" => {
                let input = get_input!("close");
                check_close(&ln(&input), get_output!("values"), 1e-10, &filename)
            }
            "log10" => {
                let input = get_input!("close");
                check_close(&log10(&input), get_output!("values"), 1e-10, &filename)
            }
            "sin" => {
                let input = get_input!("close");
                check_close(&sin(&input), get_output!("values"), 1e-10, &filename)
            }
            "sinh" => {
                let input = get_input!("close");
                check_close(&sinh(&input), get_output!("values"), 1e-10, &filename)
            }
            "sqrt" => {
                let input = get_input!("close");
                check_close(&sqrt(&input), get_output!("values"), 1e-10, &filename)
            }
            "tan" => {
                let input = get_input!("close");
                check_close(&tan(&input), get_output!("values"), 1e-10, &filename)
            }
            "tanh" => {
                let input = get_input!("close");
                check_close(&tanh(&input), get_output!("values"), 1e-10, &filename)
            }

            // ── Math Ops ──────────────────────────────────────────────────────
            "add" => {
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                check_close(&add(&real0, &real1), get_output!("values"), 1e-10, &filename)
            }
            "sub" => {
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                check_close(&sub(&real0, &real1), get_output!("values"), 1e-10, &filename)
            }
            "mult" => {
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                check_close(&mult(&real0, &real1), get_output!("values"), 1e-10, &filename)
            }
            "div" => {
                let real0 = get_input!("real0");
                let real1 = get_input!("real1");
                check_close(&div(&real0, &real1), get_output!("values"), 1e-10, &filename)
            }
            "max" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                check_close(&max(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "min" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                check_close(&min(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "sum" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                check_close(&sum(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "maxindex" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                check_close(&maxindex(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "minindex" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                check_close(&minindex(&input, period), get_output!("values"), 1e-10, &filename)
            }
            "minmax" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                let out = minmax(&input, period);
                let r_min = check_close(&out.min, get_output!("min"), 1e-10, &filename);
                let r_max = check_close(&out.max, get_output!("max"), 1e-10, &filename);
                merge_results(&filename, &[r_min, r_max])
            }
            "minmaxindex" => {
                let period = golden.meta.params["period"].as_u64().unwrap_or(30) as usize;
                let input = get_input!("close");
                let out = minmaxindex(&input, period);
                let r_min = check_close(&out.minidx, get_output!("minidx"), 1e-10, &filename);
                let r_max = check_close(&out.maxidx, get_output!("maxidx"), 1e-10, &filename);
                merge_results(&filename, &[r_min, r_max])
            }

            // ── Hilbert Transform ─────────────────────────────────────────────
            "ht_dcperiod" => {
                let input = get_input!("close");
                check_close(&ht_dcperiod(&input), get_output!("values"), 1e-10, &filename)
            }
            "ht_dcphase" => {
                let input = get_input!("close");
                check_close(&ht_dcphase(&input), get_output!("values"), 1e-10, &filename)
            }
            "ht_phasor" => {
                let input = get_input!("close");
                let (inphase, quadrature) = ht_phasor(&input);
                let r_ip = check_close(&inphase,    get_output!("inphase"),    1e-10, &filename);
                let r_q  = check_close(&quadrature, get_output!("quadrature"), 1e-10, &filename);
                merge_results(&filename, &[r_ip, r_q])
            }
            "ht_sine" => {
                let input = get_input!("close");
                let (sine, leadsine) = ht_sine(&input);
                let r_s  = check_close(&sine,     get_output!("sine"),     1e-10, &filename);
                let r_ls = check_close(&leadsine, get_output!("leadsine"), 1e-10, &filename);
                merge_results(&filename, &[r_s, r_ls])
            }
            "ht_trendline" => {
                let input = get_input!("close");
                check_close(&ht_trendline(&input), get_output!("values"), 1e-10, &filename)
            }
            "ht_trendmode" => {
                let input = get_input!("close");
                check_close(&ht_trendmode(&input), get_output!("values"), 1e-10, &filename)
            }

            // ── Candlestick Patterns ──────────────────────────────────────────
            "cdl2crows" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdl2crows(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdl3blackcrows" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdl3blackcrows(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdl3inside" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdl3inside(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdl3linestrike" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdl3linestrike(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdl3outside" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdl3outside(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdl3starsinsouth" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdl3starsinsouth(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdl3whitesoldiers" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdl3whitesoldiers(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlabandonedbaby" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlabandonedbaby(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdladvanceblock" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdladvanceblock(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlbelthold" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlbelthold(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlbreakaway" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlbreakaway(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlclosingmarubozu" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlclosingmarubozu(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlconcealbabyswall" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlconcealbabyswall(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlcounterattack" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlcounterattack(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdldarkcloudcover" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdldarkcloudcover(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdldoji" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdldoji(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdldojistar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdldojistar(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdldragonflydoji" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdldragonflydoji(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlengulfing" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlengulfing(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdleveningdojistar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdleveningdojistar(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdleveningstar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdleveningstar(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlgapsidesidewhite" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlgapsidesidewhite(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlgravestonedoji" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlgravestonedoji(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlhammer" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlhammer(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlhangingman" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlhangingman(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlharami" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlharami(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlharamicross" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlharamicross(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlhighwave" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlhighwave(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlhikkake" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlhikkake(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlhikkakemod" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlhikkakemod(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlhomingpigeon" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlhomingpigeon(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlidentical3crows" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlidentical3crows(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlinneck" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlinneck(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlinvertedhammer" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlinvertedhammer(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlkicking" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlkicking(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlkickingbylength" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlkickingbylength(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlladderbottom" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlladderbottom(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdllongleggeddoji" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdllongleggeddoji(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdllongline" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdllongline(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlmarubozu" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlmarubozu(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlmatchinglow" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlmatchinglow(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlmathold" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlmathold(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlmorningdojistar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlmorningdojistar(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlmorningstar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlmorningstar(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlonneck" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlonneck(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlpiercing" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlpiercing(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlrickshawman" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlrickshawman(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlrisefall3methods" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlrisefall3methods(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlseparatinglines" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlseparatinglines(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlshootingstar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlshootingstar(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlshortline" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlshortline(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlspinningtop" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlspinningtop(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlstalledpattern" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlstalledpattern(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlsticksandwich" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlsticksandwich(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdltakuri" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdltakuri(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdltasukigap" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdltasukigap(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlthrusting" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlthrusting(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdltristar" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdltristar(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlunique3river" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlunique3river(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlupsidegap2crows" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlupsidegap2crows(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
            }
            "cdlxsidegap3methods" => {
                let open  = get_input!("open");
                let high  = get_input!("high");
                let low   = get_input!("low");
                let close = get_input!("close");
                check_close(&cdlxsidegap3methods(&open, &high, &low, &close), get_output!("values"), 1e-10, &filename)
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
