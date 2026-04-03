//! Quick benchmark: measure all indicators, output CSV for README table generation.
//! Usage: cargo run --release --package polars-ta-verify --bin quick-bench
use polars_ta_core::hilbert::{
    ht_dcperiod, ht_dcphase, ht_phasor, ht_sine, ht_trendline, ht_trendmode,
};
use polars_ta_core::math_ops::{
    add, div, max, maxindex, min, minindex, minmax, minmaxindex, mult, sub, sum,
};
use polars_ta_core::math_transform::{
    acos, asin, atan, ceil, cos, cosh, exp, floor, ln, log10, sin, sinh, sqrt, tan, tanh,
};
use polars_ta_core::oscillator::{
    adxr, apo, aroon, aroonosc, bop, cci, cmo, dx, mfi, minus_di, minus_dm, mom, plus_di, plus_dm,
    ppo, roc, rocp, rocr, rocr100, rsi, stoch, stochf, stochrsi, trix, ultosc, willr,
};
use polars_ta_core::pattern::*;
use polars_ta_core::price_transform::{avgprice, medprice, typprice, wclprice};
use polars_ta_core::statistic::{
    beta, correl, linearreg, linearreg_angle, linearreg_intercept, linearreg_slope, stddev, tsf,
    var,
};
use polars_ta_core::trend::{
    adx, bbands, dema, ema, kama, ma, macd, macdext, macdfix, mama, mavp, midpoint, midprice, sar,
    sarext, sma, t3, tema, trima, wma,
};
use polars_ta_core::volatility::{atr, natr, trange};
use polars_ta_core::volume::{ad, adosc, obv};
use std::time::Instant;

const N: usize = 10_000;
const REPS: usize = 300;

fn make_data() -> Vec<f64> {
    (0..N)
        .map(|i| 100.0 + (i as f64 * 0.01).sin() * 10.0 + (i as f64 * 0.003).sin() * 5.0)
        .collect()
}

fn make_ohlcv() -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let c = make_data();
    let h: Vec<f64> = c.iter().map(|&x| x * 1.01).collect();
    let l: Vec<f64> = c.iter().map(|&x| x * 0.99).collect();
    let vol: Vec<f64> = (0..N).map(|i| 1_000_000.0 + i as f64).collect();
    (h, l, c, vol)
}

fn time_us<F: Fn()>(f: F) -> f64 {
    for _ in 0..10 {
        f();
    }
    let t = Instant::now();
    for _ in 0..REPS {
        f();
    }
    t.elapsed().as_micros() as f64 / REPS as f64
}

macro_rules! b {
    ($name:expr, $expr:expr) => {
        println!(
            "{},{:.3}",
            $name,
            time_us(|| {
                let _ = $expr;
            })
        );
    };
}

fn main() {
    let data = make_data();
    let (h, l, c, vol) = make_ohlcv();
    let open: Vec<f64> = h.iter().map(|&x| x * 0.995).collect();
    let period_data = vec![14.0f64; N];

    println!("indicator,p50_us");

    // ── Overlap Studies ──────────────────────────────────────────────────────
    b!("bbands", bbands(&data, 20, 2.0, 2.0));
    b!("dema", dema(&data, 14));
    b!("ema", ema(&data, 14));
    b!("ht_trendline", ht_trendline(&data));
    b!("kama", kama(&data, 10));
    b!("ma", ma(&data, 30, 1));
    b!("mama", mama(&data, 0.5, 0.05));
    b!("mavp", mavp(&data, &period_data, 2, 30));
    b!("midpoint", midpoint(&data, 14));
    b!("midprice", midprice(&h, &l, 14));
    b!("sar", sar(&h, &l, 0.02, 0.2));
    b!(
        "sarext",
        sarext(&h, &l, 0.0, 0.0, 0.02, 0.02, 0.2, 0.02, 0.02, 0.2)
    );
    b!("sma", sma(&data, 14));
    b!("t3", t3(&data, 5, 0.7));
    b!("tema", tema(&data, 14));
    b!("trima", trima(&data, 14));
    b!("wma", wma(&data, 14));

    // ── Momentum Indicators ──────────────────────────────────────────────────
    b!("adx", adx(&h, &l, &c, 14));
    b!("adxr", adxr(&h, &l, &c, 14));
    b!("apo", apo(&data, 12, 26));
    b!("aroon", aroon(&h, &l, 14));
    b!("aroonosc", aroonosc(&h, &l, 14));
    b!("bop", bop(&open, &h, &l, &c));
    b!("cci", cci(&h, &l, &c, 14));
    b!("cmo", cmo(&data, 14));
    b!("dx", dx(&h, &l, &c, 14));
    b!("macd", macd(&data, 12, 26, 9));
    b!("macdext", macdext(&data, 12, 0, 26, 0, 9, 0));
    b!("macdfix", macdfix(&data, 9));
    b!("mfi", mfi(&h, &l, &c, &vol, 14));
    b!("minus_di", minus_di(&h, &l, &c, 14));
    b!("minus_dm", minus_dm(&h, &l, 14));
    b!("mom", mom(&data, 10));
    b!("plus_di", plus_di(&h, &l, &c, 14));
    b!("plus_dm", plus_dm(&h, &l, 14));
    b!("ppo", ppo(&data, 12, 26));
    b!("roc", roc(&data, 10));
    b!("rocp", rocp(&data, 10));
    b!("rocr", rocr(&data, 10));
    b!("rocr100", rocr100(&data, 10));
    b!("rsi", rsi(&data, 14));
    b!("stoch", stoch(&h, &l, &c, 5, 3, 3));
    b!("stochf", stochf(&h, &l, &c, 5, 3));
    b!("stochrsi", stochrsi(&data, 14, 14, 3));
    b!("trix", trix(&data, 5));
    b!("ultosc", ultosc(&h, &l, &c, 7, 14, 28));
    b!("willr", willr(&h, &l, &c, 14));

    // ── Volume Indicators ────────────────────────────────────────────────────
    b!("ad", ad(&h, &l, &c, &vol));
    b!("adosc", adosc(&h, &l, &c, &vol, 3, 10));
    b!("obv", obv(&c, &vol));

    // ── Volatility Indicators ────────────────────────────────────────────────
    b!("atr", atr(&h, &l, &c, 14));
    b!("natr", natr(&h, &l, &c, 14));
    b!("trange", trange(&h, &l, &c));

    // ── Price Transform ──────────────────────────────────────────────────────
    b!("avgprice", avgprice(&open, &h, &l, &c));
    b!("medprice", medprice(&h, &l));
    b!("typprice", typprice(&h, &l, &c));
    b!("wclprice", wclprice(&h, &l, &c));

    // ── Cycle Indicators ─────────────────────────────────────────────────────
    b!("ht_dcperiod", ht_dcperiod(&data));
    b!("ht_dcphase", ht_dcphase(&data));
    b!("ht_phasor", ht_phasor(&data));
    b!("ht_sine", ht_sine(&data));
    b!("ht_trendmode", ht_trendmode(&data));

    // ── Statistics ───────────────────────────────────────────────────────────
    b!("beta", beta(&h, &l, 5));
    b!("correl", correl(&h, &l, 30));
    b!("linearreg", linearreg(&data, 14));
    b!("linearreg_angle", linearreg_angle(&data, 14));
    b!("linearreg_intercept", linearreg_intercept(&data, 14));
    b!("linearreg_slope", linearreg_slope(&data, 14));
    b!("stddev", stddev(&data, 5, 1.0));
    b!("tsf", tsf(&data, 14));
    b!("var", var(&data, 5, 1.0));

    // ── Math Transform ───────────────────────────────────────────────────────
    b!("acos", acos(&data));
    b!("asin", asin(&data));
    b!("atan", atan(&data));
    b!("ceil", ceil(&data));
    b!("cos", cos(&data));
    b!("cosh", cosh(&data));
    b!("exp", exp(&data));
    b!("floor", floor(&data));
    b!("ln", ln(&data));
    b!("log10", log10(&data));
    b!("sin", sin(&data));
    b!("sinh", sinh(&data));
    b!("sqrt", sqrt(&data));
    b!("tan", tan(&data));
    b!("tanh", tanh(&data));

    // ── Math Operators ───────────────────────────────────────────────────────
    b!("add", add(&data, &h));
    b!("div", div(&data, &h));
    b!("max", max(&data, 30));
    b!("maxindex", maxindex(&data, 30));
    b!("min", min(&data, 30));
    b!("minindex", minindex(&data, 30));
    b!("minmax", minmax(&data, 30));
    b!("minmaxindex", minmaxindex(&data, 30));
    b!("mult", mult(&data, &h));
    b!("sub", sub(&data, &h));
    b!("sum", sum(&data, 30));

    // ── Pattern Recognition (CDL) — sample for throughput ───────────────────
    b!("cdl2crows", cdl2crows(&open, &h, &l, &c));
    b!("cdl3blackcrows", cdl3blackcrows(&open, &h, &l, &c));
    b!("cdl3inside", cdl3inside(&open, &h, &l, &c));
    b!("cdl3linestrike", cdl3linestrike(&open, &h, &l, &c));
    b!("cdl3starsinsouth", cdl3starsinsouth(&open, &h, &l, &c));
    b!("cdl3whitesoldiers", cdl3whitesoldiers(&open, &h, &l, &c));
    b!("cdlabandonedbaby", cdlabandonedbaby(&open, &h, &l, &c));
    b!("cdladvanceblock", cdladvanceblock(&open, &h, &l, &c));
    b!("cdlbelthold", cdlbelthold(&open, &h, &l, &c));
    b!("cdlbreakaway", cdlbreakaway(&open, &h, &l, &c));
    b!("cdlclosingmarubozu", cdlclosingmarubozu(&open, &h, &l, &c));
    b!(
        "cdlconcealbabyswall",
        cdlconcealbabyswall(&open, &h, &l, &c)
    );
    b!("cdlcounterattack", cdlcounterattack(&open, &h, &l, &c));
    b!("cdldarkcloudcover", cdldarkcloudcover(&open, &h, &l, &c));
    b!("cdldoji", cdldoji(&open, &h, &l, &c));
    b!("cdldojistar", cdldojistar(&open, &h, &l, &c));
    b!("cdldragonflydoji", cdldragonflydoji(&open, &h, &l, &c));
    b!("cdlengulfing", cdlengulfing(&open, &h, &l, &c));
    b!("cdleveningdojistar", cdleveningdojistar(&open, &h, &l, &c));
    b!("cdleveningstar", cdleveningstar(&open, &h, &l, &c));
    b!(
        "cdlgapsidesidewhite",
        cdlgapsidesidewhite(&open, &h, &l, &c)
    );
    b!("cdlgravestonedoji", cdlgravestonedoji(&open, &h, &l, &c));
    b!("cdlhammer", cdlhammer(&open, &h, &l, &c));
    b!("cdlhangingman", cdlhangingman(&open, &h, &l, &c));
    b!("cdlharami", cdlharami(&open, &h, &l, &c));
    b!("cdlharamicross", cdlharamicross(&open, &h, &l, &c));
    b!("cdlhighwave", cdlhighwave(&open, &h, &l, &c));
    b!("cdlhikkake", cdlhikkake(&open, &h, &l, &c));
    b!("cdlhikkakemod", cdlhikkakemod(&open, &h, &l, &c));
    b!("cdlhomingpigeon", cdlhomingpigeon(&open, &h, &l, &c));
    b!("cdlidentical3crows", cdlidentical3crows(&open, &h, &l, &c));
    b!("cdlinneck", cdlinneck(&open, &h, &l, &c));
    b!("cdlinvertedhammer", cdlinvertedhammer(&open, &h, &l, &c));
    b!("cdlkicking", cdlkicking(&open, &h, &l, &c));
    b!("cdlkickingbylength", cdlkickingbylength(&open, &h, &l, &c));
    b!("cdlladderbottom", cdlladderbottom(&open, &h, &l, &c));
    b!("cdllongleggeddoji", cdllongleggeddoji(&open, &h, &l, &c));
    b!("cdllongline", cdllongline(&open, &h, &l, &c));
    b!("cdlmarubozu", cdlmarubozu(&open, &h, &l, &c));
    b!("cdlmatchinglow", cdlmatchinglow(&open, &h, &l, &c));
    b!("cdlmathold", cdlmathold(&open, &h, &l, &c));
    b!("cdlmorningdojistar", cdlmorningdojistar(&open, &h, &l, &c));
    b!("cdlmorningstar", cdlmorningstar(&open, &h, &l, &c));
    b!("cdlonneck", cdlonneck(&open, &h, &l, &c));
    b!("cdlpiercing", cdlpiercing(&open, &h, &l, &c));
    b!("cdlrickshawman", cdlrickshawman(&open, &h, &l, &c));
    b!(
        "cdlrisefall3methods",
        cdlrisefall3methods(&open, &h, &l, &c)
    );
    b!("cdlseparatinglines", cdlseparatinglines(&open, &h, &l, &c));
    b!("cdlshootingstar", cdlshootingstar(&open, &h, &l, &c));
    b!("cdlshortline", cdlshortline(&open, &h, &l, &c));
    b!("cdlspinningtop", cdlspinningtop(&open, &h, &l, &c));
    b!("cdlstalledpattern", cdlstalledpattern(&open, &h, &l, &c));
    b!("cdlsticksandwich", cdlsticksandwich(&open, &h, &l, &c));
    b!("cdltakuri", cdltakuri(&open, &h, &l, &c));
    b!("cdltasukigap", cdltasukigap(&open, &h, &l, &c));
    b!("cdlthrusting", cdlthrusting(&open, &h, &l, &c));
    b!("cdltristar", cdltristar(&open, &h, &l, &c));
    b!("cdlunique3river", cdlunique3river(&open, &h, &l, &c));
    b!("cdlupsidegap2crows", cdlupsidegap2crows(&open, &h, &l, &c));
    b!(
        "cdlxsidegap3methods",
        cdlxsidegap3methods(&open, &h, &l, &c)
    );
}
