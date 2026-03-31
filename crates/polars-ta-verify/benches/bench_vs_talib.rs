//! Benchmark: polars-ta-core vs naive Rust implementations.
//!
//! Tests three data scales (Small=100, Medium=10,000, Large=1,000,000)
//! and outputs throughput in elements/second for comparison with ta-lib C.
//!
//! Run with: `cargo bench --package polars-ta-verify`
//!
//! To compare with ta-lib C, run: `scripts/compare_all.sh`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use polars_ta_core::oscillator::{cci, rsi, stoch, stochrsi, ultosc, willr};
use polars_ta_core::trend::{adx, bbands, dema, ema, kama, macd, midpoint, midprice, sar, sma, t3, tema, trima, wma};
use polars_ta_core::volatility::{atr, natr, trange};
use polars_ta_core::volume::{ad, adosc, obv};

const SIZES: [usize; 3] = [100, 10_000, 1_000_000];

fn make_data(size: usize) -> Vec<f64> {
    (0..size).map(|i| 100.0 + i as f64 * 0.01).collect()
}

/// 生成模拟 OHLCV 数据（high > close > low，volume 递增）
fn make_ohlcv(size: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let close: Vec<f64> = (0..size).map(|i| 100.0 + i as f64 * 0.01).collect();
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
);
criterion_main!(benches);
