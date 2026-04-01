use std::time::Instant;
fn bench_fn<F: Fn() -> Vec<f64>>(name: &str, f: F, n: usize) {
    let reps = 9;
    let mut times: Vec<u128> = (0..reps).map(|_| { let s = Instant::now(); std::hint::black_box(f()); s.elapsed().as_micros() }).collect();
    times.sort();
    let med = times[reps/2] as f64;
    println!("{:<20} {:>8.0} µs  {:>8.1} M/s", name, med, n as f64 / med);
}
fn main() {
    let n = 1_000_000usize;
    // Same data as bench_talib.py: linear ramp matching make_bench_data/make_ohlcv
    let close: Vec<f64> = (0..n).map(|i| i as f64 * 0.01 + 100.0).collect();
    let high:  Vec<f64> = close.iter().map(|&c| c * 1.01).collect();
    let low:   Vec<f64> = close.iter().map(|&c| c * 0.99).collect();
    let volume: Vec<f64> = (0..n).map(|i| i as f64 * 10.0 + 1_000_000.0).collect();
    
    bench_fn("bbands(20)", || { let out = polars_ta_core::trend::bbands::bbands(&close, 20, 2.0, 2.0); vec![out.upper[0],out.middle[0],out.lower[0]] }, n);
    bench_fn("adx(14)", || polars_ta_core::trend::adx::adx(&high, &low, &close, 14), n);
    bench_fn("obv", || polars_ta_core::volume::obv::obv(&close, &volume), n);
    bench_fn("ad", || polars_ta_core::volume::ad::ad(&high, &low, &close, &volume), n);
}
