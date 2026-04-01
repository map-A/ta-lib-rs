//! Benchmark: polars-ta-core vs naive Rust implementations.
//!
//! Tests three data scales (Small=100, Medium=10,000, Large=1,000,000)
//! and outputs throughput in elements/second for comparison with ta-lib C.
//!
//! Run with: `cargo bench --package polars-ta-verify`
//!
//! To compare with ta-lib C, run: `scripts/compare_all.sh`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use polars_ta_core::math_ops::{add, div, max, maxindex, min, minindex, minmax, minmaxindex, mult, sub, sum};
use polars_ta_core::math_transform::{acos, asin, atan, ceil, cos, cosh, exp, floor, ln, log10, sin, sinh, sqrt, tan, tanh};
use polars_ta_core::oscillator::{adxr, apo, aroon, aroonosc, bop, cci, cmo, dx, mfi, minus_di, minus_dm, mom, plus_di, plus_dm, ppo, roc, rocp, rocr, rocr100, rsi, stoch, stochf, stochrsi, trix, ultosc, willr};
use polars_ta_core::price_transform::{avgprice, medprice, typprice, wclprice};
use polars_ta_core::statistic::{beta, correl, linearreg, linearreg_angle, linearreg_intercept, linearreg_slope, stddev, tsf, var};
use polars_ta_core::trend::{adx, bbands, dema, ema, kama, ma, macd, macdext, macdfix, midpoint, midprice, sar, sarext, sma, t3, tema, trima, wma};
use polars_ta_core::volatility::{atr, natr, trange};
use polars_ta_core::volume::{ad, adosc, obv};

const SIZES: [usize; 3] = [100, 10_000, 1_000_000];

fn make_data(size: usize) -> Vec<f64> {
    // Sinusoidal price-like data — avoids monotonic fast-paths in sliding-window algorithms.
    (0..size)
        .map(|i| 100.0 + (i as f64 * 0.01).sin() * 10.0 + (i as f64 * 0.003).sin() * 5.0)
        .collect()
}

/// 生成模拟 OHLCV 数据（high > close > low，volume 递增）
fn make_ohlcv(size: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let close: Vec<f64> = (0..size)
        .map(|i| 100.0 + (i as f64 * 0.01).sin() * 10.0 + (i as f64 * 0.003).sin() * 5.0)
        .collect();
    let high: Vec<f64> = close.iter().map(|&c| c * 1.01).collect();
    let low: Vec<f64> = close.iter().map(|&c| c * 0.99).collect();
    let volume: Vec<f64> = (0..size).map(|i| 1_000_000.0 + i as f64 * 10.0).collect();
    (high, low, close, volume)
}

// ─── Trend / Moving Averages ──────────────────────────────────────────────────

fn bench_sma(c: &mut Criterion) {
    let mut group = c.benchmark_group("sma");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| sma(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_ema(c: &mut Criterion) {
    let mut group = c.benchmark_group("ema");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| ema(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_wma(c: &mut Criterion) {
    let mut group = c.benchmark_group("wma");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| wma(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_dema(c: &mut Criterion) {
    let mut group = c.benchmark_group("dema");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| dema(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_tema(c: &mut Criterion) {
    let mut group = c.benchmark_group("tema");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| tema(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_macd(c: &mut Criterion) {
    let mut group = c.benchmark_group("macd");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| macd(black_box(&data), black_box(12), black_box(26), black_box(9)))
        });
    }
    group.finish();
}

fn bench_bbands(c: &mut Criterion) {
    let mut group = c.benchmark_group("bbands");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| bbands(black_box(&data), black_box(20), black_box(2.0), black_box(2.0)))
        });
    }
    group.finish();
}

fn bench_sar(c: &mut Criterion) {
    let mut group = c.benchmark_group("sar");
    for size in SIZES {
        let (high, low, _, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| sar(black_box(&high), black_box(&low), black_box(0.02), black_box(0.2)))
        });
    }
    group.finish();
}

fn bench_adx(c: &mut Criterion) {
    let mut group = c.benchmark_group("adx");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| adx(black_box(&high), black_box(&low), black_box(&close), black_box(14)))
        });
    }
    group.finish();
}

// ─── Oscillators ──────────────────────────────────────────────────────────────

fn bench_rsi(c: &mut Criterion) {
    let mut group = c.benchmark_group("rsi");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| rsi(black_box(&data), black_box(14)))
        });
    }
    group.finish();
}

fn bench_cci(c: &mut Criterion) {
    let mut group = c.benchmark_group("cci");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| cci(black_box(&high), black_box(&low), black_box(&close), black_box(14)))
        });
    }
    group.finish();
}

fn bench_willr(c: &mut Criterion) {
    let mut group = c.benchmark_group("willr");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| willr(black_box(&high), black_box(&low), black_box(&close), black_box(14)))
        });
    }
    group.finish();
}

fn bench_stoch(c: &mut Criterion) {
    let mut group = c.benchmark_group("stoch");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| {
                stoch(
                    black_box(&high),
                    black_box(&low),
                    black_box(&close),
                    black_box(5),
                    black_box(3),
                    black_box(3),
                )
            })
        });
    }
    group.finish();
}

fn bench_stochrsi(c: &mut Criterion) {
    let mut group = c.benchmark_group("stochrsi");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| stochrsi(black_box(&data), black_box(14), black_box(5), black_box(3)))
        });
    }
    group.finish();
}

fn bench_ultosc(c: &mut Criterion) {
    let mut group = c.benchmark_group("ultosc");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| {
                ultosc(
                    black_box(&high),
                    black_box(&low),
                    black_box(&close),
                    black_box(7),
                    black_box(14),
                    black_box(28),
                )
            })
        });
    }
    group.finish();
}

// ─── Volume ───────────────────────────────────────────────────────────────────

fn bench_obv(c: &mut Criterion) {
    let mut group = c.benchmark_group("obv");
    for size in SIZES {
        let (_, _, close, volume) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| obv(black_box(&close), black_box(&volume)))
        });
    }
    group.finish();
}

fn bench_ad(c: &mut Criterion) {
    let mut group = c.benchmark_group("ad");
    for size in SIZES {
        let (high, low, close, volume) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| {
                ad(
                    black_box(&high),
                    black_box(&low),
                    black_box(&close),
                    black_box(&volume),
                )
            })
        });
    }
    group.finish();
}

fn bench_adosc(c: &mut Criterion) {
    let mut group = c.benchmark_group("adosc");
    for size in SIZES {
        let (high, low, close, volume) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| {
                adosc(
                    black_box(&high),
                    black_box(&low),
                    black_box(&close),
                    black_box(&volume),
                    black_box(3),
                    black_box(10),
                )
            })
        });
    }
    group.finish();
}

// ─── Volatility ───────────────────────────────────────────────────────────────

fn bench_trange(c: &mut Criterion) {
    let mut group = c.benchmark_group("trange");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| trange(black_box(&high), black_box(&low), black_box(&close)))
        });
    }
    group.finish();
}

fn bench_atr(c: &mut Criterion) {
    let mut group = c.benchmark_group("atr");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| atr(black_box(&high), black_box(&low), black_box(&close), black_box(14)))
        });
    }
    group.finish();
}

fn bench_natr(c: &mut Criterion) {
    let mut group = c.benchmark_group("natr");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| natr(black_box(&high), black_box(&low), black_box(&close), black_box(14)))
        });
    }
    group.finish();
}

// ─── Phase 2 Oscillators ──────────────────────────────────────────────────────

fn bench_mom(c: &mut Criterion) {
    let mut group = c.benchmark_group("mom");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| mom(black_box(&data), black_box(10)))
        });
    }
    group.finish();
}

fn bench_roc(c: &mut Criterion) {
    let mut group = c.benchmark_group("roc");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| roc(black_box(&data), black_box(10)))
        });
    }
    group.finish();
}

fn bench_rocp(c: &mut Criterion) {
    let mut group = c.benchmark_group("rocp");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| rocp(black_box(&data), black_box(10)))
        });
    }
    group.finish();
}

fn bench_rocr(c: &mut Criterion) {
    let mut group = c.benchmark_group("rocr");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| rocr(black_box(&data), black_box(10)))
        });
    }
    group.finish();
}

fn bench_rocr100(c: &mut Criterion) {
    let mut group = c.benchmark_group("rocr100");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| rocr100(black_box(&data), black_box(10)))
        });
    }
    group.finish();
}

fn bench_cmo(c: &mut Criterion) {
    let mut group = c.benchmark_group("cmo");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| cmo(black_box(&data), black_box(14)))
        });
    }
    group.finish();
}

fn bench_apo(c: &mut Criterion) {
    let mut group = c.benchmark_group("apo");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| apo(black_box(&data), black_box(12), black_box(26)))
        });
    }
    group.finish();
}

fn bench_ppo(c: &mut Criterion) {
    let mut group = c.benchmark_group("ppo");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| ppo(black_box(&data), black_box(12), black_box(26)))
        });
    }
    group.finish();
}

fn bench_trix(c: &mut Criterion) {
    let mut group = c.benchmark_group("trix");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| trix(black_box(&data), black_box(14)))
        });
    }
    group.finish();
}

fn bench_bop(c: &mut Criterion) {
    let mut group = c.benchmark_group("bop");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        let open: Vec<f64> = close.iter().map(|&c| c * 1.005).collect();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| bop(black_box(&open), black_box(&high), black_box(&low), black_box(&close)))
        });
    }
    group.finish();
}

fn bench_minus_dm(c: &mut Criterion) {
    let mut group = c.benchmark_group("minus_dm");
    for size in SIZES {
        let (high, low, _, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| minus_dm(black_box(&high), black_box(&low), black_box(14)))
        });
    }
    group.finish();
}

fn bench_plus_dm(c: &mut Criterion) {
    let mut group = c.benchmark_group("plus_dm");
    for size in SIZES {
        let (high, low, _, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| plus_dm(black_box(&high), black_box(&low), black_box(14)))
        });
    }
    group.finish();
}

fn bench_minus_di(c: &mut Criterion) {
    let mut group = c.benchmark_group("minus_di");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| minus_di(black_box(&high), black_box(&low), black_box(&close), black_box(14)))
        });
    }
    group.finish();
}

fn bench_plus_di(c: &mut Criterion) {
    let mut group = c.benchmark_group("plus_di");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| plus_di(black_box(&high), black_box(&low), black_box(&close), black_box(14)))
        });
    }
    group.finish();
}

fn bench_dx(c: &mut Criterion) {
    let mut group = c.benchmark_group("dx");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| dx(black_box(&high), black_box(&low), black_box(&close), black_box(14)))
        });
    }
    group.finish();
}

fn bench_adxr(c: &mut Criterion) {
    let mut group = c.benchmark_group("adxr");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| adxr(black_box(&high), black_box(&low), black_box(&close), black_box(14)))
        });
    }
    group.finish();
}

// ─── Phase 2 Statistics ───────────────────────────────────────────────────────

fn bench_beta(c: &mut Criterion) {
    let mut group = c.benchmark_group("beta");
    for size in SIZES {
        let data = make_data(size);
        let data2: Vec<f64> = data.iter().map(|&x| x + 1.0).collect();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| beta(black_box(&data), black_box(&data2), black_box(5)))
        });
    }
    group.finish();
}

fn bench_correl(c: &mut Criterion) {
    let mut group = c.benchmark_group("correl");
    for size in SIZES {
        let data = make_data(size);
        let data2: Vec<f64> = data.iter().map(|&x| x + 1.0).collect();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| correl(black_box(&data), black_box(&data2), black_box(30)))
        });
    }
    group.finish();
}

fn bench_linearreg(c: &mut Criterion) {
    let mut group = c.benchmark_group("linearreg");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| linearreg(black_box(&data), black_box(14)))
        });
    }
    group.finish();
}

fn bench_linearreg_angle(c: &mut Criterion) {
    let mut group = c.benchmark_group("linearreg_angle");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| linearreg_angle(black_box(&data), black_box(14)))
        });
    }
    group.finish();
}

fn bench_linearreg_intercept(c: &mut Criterion) {
    let mut group = c.benchmark_group("linearreg_intercept");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| linearreg_intercept(black_box(&data), black_box(14)))
        });
    }
    group.finish();
}

fn bench_linearreg_slope(c: &mut Criterion) {
    let mut group = c.benchmark_group("linearreg_slope");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| linearreg_slope(black_box(&data), black_box(14)))
        });
    }
    group.finish();
}

fn bench_stddev(c: &mut Criterion) {
    let mut group = c.benchmark_group("stddev");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| stddev(black_box(&data), black_box(5), black_box(1.0)))
        });
    }
    group.finish();
}

fn bench_tsf(c: &mut Criterion) {
    let mut group = c.benchmark_group("tsf");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| tsf(black_box(&data), black_box(14)))
        });
    }
    group.finish();
}

fn bench_var(c: &mut Criterion) {
    let mut group = c.benchmark_group("var");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| var(black_box(&data), black_box(5), black_box(1.0)))
        });
    }
    group.finish();
}

// ─── Phase 2 Trend ────────────────────────────────────────────────────────────

fn bench_kama(c: &mut Criterion) {
    let mut group = c.benchmark_group("kama");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| kama(black_box(&data), black_box(30)))
        });
    }
    group.finish();
}

fn bench_trima(c: &mut Criterion) {
    let mut group = c.benchmark_group("trima");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| trima(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_t3(c: &mut Criterion) {
    let mut group = c.benchmark_group("t3");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| t3(black_box(&data), black_box(5), black_box(0.7)))
        });
    }
    group.finish();
}

fn bench_midpoint(c: &mut Criterion) {
    let mut group = c.benchmark_group("midpoint");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| midpoint(black_box(&data), black_box(14)))
        });
    }
    group.finish();
}

fn bench_midprice(c: &mut Criterion) {
    let mut group = c.benchmark_group("midprice");
    for size in SIZES {
        let (high, low, _, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| midprice(black_box(&high), black_box(&low), black_box(14)))
        });
    }
    group.finish();
}

// ─── Phase 1 missing: aroon, mfi ─────────────────────────────────────────────

fn bench_aroon(c: &mut Criterion) {
    let mut group = c.benchmark_group("aroon");
    for size in SIZES {
        let (high, low, _, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| aroon(black_box(&high), black_box(&low), black_box(14)))
        });
    }
    group.finish();
}

fn bench_mfi(c: &mut Criterion) {
    let mut group = c.benchmark_group("mfi");
    for size in SIZES {
        let (high, low, close, volume) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| mfi(black_box(&high), black_box(&low), black_box(&close), black_box(&volume), black_box(14)))
        });
    }
    group.finish();
}

// ─── Phase 3: Math Operators ──────────────────────────────────────────────────

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");
    for size in SIZES {
        let data = make_data(size);
        let data2 = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| add(black_box(&data), black_box(&data2)))
        });
    }
    group.finish();
}

fn bench_div(c: &mut Criterion) {
    let mut group = c.benchmark_group("div");
    for size in SIZES {
        let data = make_data(size);
        let data2 = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| div(black_box(&data), black_box(&data2)))
        });
    }
    group.finish();
}

fn bench_mult(c: &mut Criterion) {
    let mut group = c.benchmark_group("mult");
    for size in SIZES {
        let data = make_data(size);
        let data2 = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| mult(black_box(&data), black_box(&data2)))
        });
    }
    group.finish();
}

fn bench_sub(c: &mut Criterion) {
    let mut group = c.benchmark_group("sub");
    for size in SIZES {
        let data = make_data(size);
        let data2 = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| sub(black_box(&data), black_box(&data2)))
        });
    }
    group.finish();
}

fn bench_max(c: &mut Criterion) {
    let mut group = c.benchmark_group("max");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| max(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_min(c: &mut Criterion) {
    let mut group = c.benchmark_group("min");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| min(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| sum(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_maxindex(c: &mut Criterion) {
    let mut group = c.benchmark_group("maxindex");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| maxindex(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_minindex(c: &mut Criterion) {
    let mut group = c.benchmark_group("minindex");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| minindex(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_minmax(c: &mut Criterion) {
    let mut group = c.benchmark_group("minmax");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| minmax(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

fn bench_minmaxindex(c: &mut Criterion) {
    let mut group = c.benchmark_group("minmaxindex");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| minmaxindex(black_box(&data), black_box(20)))
        });
    }
    group.finish();
}

// ─── Phase 3: Math Transform ──────────────────────────────────────────────────

fn bench_acos(c: &mut Criterion) {
    let mut group = c.benchmark_group("acos");
    for size in SIZES {
        // 归一化到 [-1, 1] 以产生合法输出
        let data: Vec<f64> = (0..size).map(|i| (i as f64 / size as f64) * 2.0 - 1.0).collect();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| acos(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_asin(c: &mut Criterion) {
    let mut group = c.benchmark_group("asin");
    for size in SIZES {
        let data: Vec<f64> = (0..size).map(|i| (i as f64 / size as f64) * 2.0 - 1.0).collect();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| asin(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_atan(c: &mut Criterion) {
    let mut group = c.benchmark_group("atan");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| atan(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_ceil(c: &mut Criterion) {
    let mut group = c.benchmark_group("ceil");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| ceil(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_cos(c: &mut Criterion) {
    let mut group = c.benchmark_group("cos");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| cos(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_cosh(c: &mut Criterion) {
    let mut group = c.benchmark_group("cosh");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| cosh(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_exp(c: &mut Criterion) {
    let mut group = c.benchmark_group("exp");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| exp(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_floor(c: &mut Criterion) {
    let mut group = c.benchmark_group("floor");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| floor(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_ln(c: &mut Criterion) {
    let mut group = c.benchmark_group("ln");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| ln(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_log10(c: &mut Criterion) {
    let mut group = c.benchmark_group("log10");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| log10(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_sin(c: &mut Criterion) {
    let mut group = c.benchmark_group("sin");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| sin(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_sinh(c: &mut Criterion) {
    let mut group = c.benchmark_group("sinh");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| sinh(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_sqrt(c: &mut Criterion) {
    let mut group = c.benchmark_group("sqrt");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| sqrt(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_tan(c: &mut Criterion) {
    let mut group = c.benchmark_group("tan");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| tan(black_box(&data)))
        });
    }
    group.finish();
}

fn bench_tanh(c: &mut Criterion) {
    let mut group = c.benchmark_group("tanh");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| tanh(black_box(&data)))
        });
    }
    group.finish();
}

// ─── Phase 3: Price Transform ─────────────────────────────────────────────────

fn bench_avgprice(c: &mut Criterion) {
    let mut group = c.benchmark_group("avgprice");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        let open: Vec<f64> = close.iter().map(|&c| c * 0.995).collect();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| avgprice(black_box(&open), black_box(&high), black_box(&low), black_box(&close)))
        });
    }
    group.finish();
}

fn bench_medprice(c: &mut Criterion) {
    let mut group = c.benchmark_group("medprice");
    for size in SIZES {
        let (high, low, _, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| medprice(black_box(&high), black_box(&low)))
        });
    }
    group.finish();
}

fn bench_typprice(c: &mut Criterion) {
    let mut group = c.benchmark_group("typprice");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| typprice(black_box(&high), black_box(&low), black_box(&close)))
        });
    }
    group.finish();
}

fn bench_wclprice(c: &mut Criterion) {
    let mut group = c.benchmark_group("wclprice");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| wclprice(black_box(&high), black_box(&low), black_box(&close)))
        });
    }
    group.finish();
}

// ─── Phase 3: 振荡器 / 趋势 ────────────────────────────────────────────────────

fn bench_aroonosc(c: &mut Criterion) {
    let mut group = c.benchmark_group("aroonosc");
    for size in SIZES {
        let (high, low, _, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| aroonosc(black_box(&high), black_box(&low), black_box(14)))
        });
    }
    group.finish();
}

fn bench_stochf(c: &mut Criterion) {
    let mut group = c.benchmark_group("stochf");
    for size in SIZES {
        let (high, low, close, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| stochf(black_box(&high), black_box(&low), black_box(&close), black_box(5), black_box(3)))
        });
    }
    group.finish();
}

fn bench_ma(c: &mut Criterion) {
    let mut group = c.benchmark_group("ma");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| ma(black_box(&data), black_box(20), black_box(1)))
        });
    }
    group.finish();
}

fn bench_macdext(c: &mut Criterion) {
    let mut group = c.benchmark_group("macdext");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| macdext(black_box(&data), black_box(12), black_box(1), black_box(26), black_box(1), black_box(9), black_box(1)))
        });
    }
    group.finish();
}

fn bench_macdfix(c: &mut Criterion) {
    let mut group = c.benchmark_group("macdfix");
    for size in SIZES {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| macdfix(black_box(&data), black_box(9)))
        });
    }
    group.finish();
}

fn bench_sarext(c: &mut Criterion) {
    let mut group = c.benchmark_group("sarext");
    for size in SIZES {
        let (high, low, _, _) = make_ohlcv(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size, |b, _| {
            b.iter(|| sarext(
                black_box(&high), black_box(&low),
                black_box(0.0), black_box(0.0),
                black_box(0.02), black_box(0.02), black_box(0.20),
                black_box(0.02), black_box(0.02), black_box(0.20),
            ))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_sma,
    bench_ema,
    bench_wma,
    bench_dema,
    bench_tema,
    bench_macd,
    bench_bbands,
    bench_sar,
    bench_adx,
    bench_rsi,
    bench_cci,
    bench_willr,
    bench_stoch,
    bench_stochrsi,
    bench_ultosc,
    bench_obv,
    bench_ad,
    bench_adosc,
    bench_trange,
    bench_atr,
    bench_natr,
    // Phase 2 振荡器
    bench_mom,
    bench_roc,
    bench_rocp,
    bench_rocr,
    bench_rocr100,
    bench_cmo,
    bench_apo,
    bench_ppo,
    bench_trix,
    bench_bop,
    bench_minus_dm,
    bench_plus_dm,
    bench_minus_di,
    bench_plus_di,
    bench_dx,
    bench_adxr,
    // Phase 2 统计
    bench_beta,
    bench_correl,
    bench_linearreg,
    bench_linearreg_angle,
    bench_linearreg_intercept,
    bench_linearreg_slope,
    bench_stddev,
    bench_tsf,
    bench_var,
    // Phase 2 趋势
    bench_kama,
    bench_trima,
    bench_t3,
    bench_midpoint,
    bench_midprice,
    // Phase 1 missing
    bench_aroon,
    bench_mfi,
    // Phase 3 数学运算符
    bench_add,
    bench_div,
    bench_mult,
    bench_sub,
    bench_max,
    bench_min,
    bench_sum,
    bench_maxindex,
    bench_minindex,
    bench_minmax,
    bench_minmaxindex,
    // Phase 3 数学变换
    bench_acos,
    bench_asin,
    bench_atan,
    bench_ceil,
    bench_cos,
    bench_cosh,
    bench_exp,
    bench_floor,
    bench_ln,
    bench_log10,
    bench_sin,
    bench_sinh,
    bench_sqrt,
    bench_tan,
    bench_tanh,
    // Phase 3 价格变换
    bench_avgprice,
    bench_medprice,
    bench_typprice,
    bench_wclprice,
    // Phase 3 振荡器与趋势
    bench_aroonosc,
    bench_stochf,
    bench_ma,
    bench_macdext,
    bench_macdfix,
    bench_sarext,
);
criterion_main!(benches);
