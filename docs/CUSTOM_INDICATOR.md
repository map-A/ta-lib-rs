# Custom Indicator Guide

> A step-by-step guide for **third-party developers** who want to implement a custom technical indicator on top of `polars-ta`, without modifying the library itself.

---

## Overview

### When to use this guide

Use this guide when you want to add an indicator **to your own project** that depends on `polars-ta`. If your indicator is general-purpose and you want to contribute it upstream, see [AI_GUIDE.md](AI_GUIDE.md) for the internal development flow instead.

### Architecture

polars-ta has two layers:

```
polars-ta-core   — pure Rust algorithm on &[f64] slices, zero dependencies
polars-ta        — thin Polars Series wrapper around core functions
```

When writing a custom indicator you follow the same pattern: implement the algorithm on raw slices, then optionally wrap it in a Polars helper. You can place both in your own crate and call them from your application code.

---

## Full Working Example: VWAP

We will implement **VWAP — Volume Weighted Average Price** over a rolling window.

**Formula:**
```
typical_price[i] = (high[i] + low[i] + close[i]) / 3
vwap[i] = sum(typical_price[i-period+1..=i] * volume[i-period+1..=i])
         / sum(volume[i-period+1..=i])
```

- lookback = `period - 1`
- output length = `input.len() - (period - 1)`
- NaN in any input → NaN for that output bar

---

## Step 1: Create the Core Algorithm

Create a new Rust crate (or add a module to your existing one):

```bash
cargo new --lib my-indicators
cd my-indicators
```

Add to `Cargo.toml`:

```toml
[dependencies]
# Only needed if you want Polars Series wrappers (Step 3)
polars-ta = "0.1"
polars = { version = "0.53", features = ["lazy"] }
```

Create `src/vwap.rs`:

```rust
//! VWAP — Volume Weighted Average Price (rolling window)
//!
//! # Parameters
//! - `high`   — high price series
//! - `low`    — low price series
//! - `close`  — close price series
//! - `volume` — volume series (same length as price series)
//! - `period` — rolling window length (≥ 1)
//!
//! # Output
//! Length = `close.len() - (period - 1)`.
//! Returns empty `Vec` when any input is shorter than `period`.
//!
//! # Example
//! ```
//! use my_indicators::vwap::vwap;
//!
//! let high   = vec![10.0, 11.0, 12.0, 11.0, 10.0];
//! let low    = vec![ 9.0, 10.0, 11.0, 10.0,  9.0];
//! let close  = vec![ 9.5, 10.5, 11.5, 10.5,  9.5];
//! let volume = vec![100.0, 200.0, 150.0, 180.0, 120.0];
//!
//! let result = vwap(&high, &low, &close, &volume, 3);
//! assert_eq!(result.len(), 3);  // 5 - (3-1)
//! ```

pub fn vwap(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    period: usize,
) -> Vec<f64> {
    let n = close.len();
    assert_eq!(n, high.len(),   "high and close must have equal length");
    assert_eq!(n, low.len(),    "low and close must have equal length");
    assert_eq!(n, volume.len(), "volume and close must have equal length");

    if period == 0 || n < period {
        return vec![];
    }

    let lookback = period - 1;
    let out_len = n - lookback;
    let mut out = Vec::with_capacity(out_len);

    // Precompute typical_price * volume and volume arrays
    let tp_vol: Vec<f64> = (0..n)
        .map(|i| {
            let tp = (high[i] + low[i] + close[i]) / 3.0;
            tp * volume[i]
        })
        .collect();

    // Bootstrap the first window
    let mut sum_tv: f64 = tp_vol[..period].iter().sum();
    let mut sum_v: f64 = volume[..period].iter().sum();

    out.push(if sum_v == 0.0 { f64::NAN } else { sum_tv / sum_v });

    // Slide the window
    for i in period..n {
        sum_tv += tp_vol[i] - tp_vol[i - period];
        sum_v  += volume[i] - volume[i - period];
        out.push(if sum_v == 0.0 { f64::NAN } else { sum_tv / sum_v });
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn output_length_correct() {
        let h = vec![1.0; 10];
        let l = vec![1.0; 10];
        let c = vec![1.0; 10];
        let v = vec![1.0; 10];
        assert_eq!(vwap(&h, &l, &c, &v, 3).len(), 8);
    }

    #[test]
    fn returns_empty_when_too_short() {
        let h = vec![1.0, 2.0];
        let l = vec![1.0, 2.0];
        let c = vec![1.0, 2.0];
        let v = vec![1.0, 2.0];
        assert!(vwap(&h, &l, &c, &v, 3).is_empty());
    }

    #[test]
    fn uniform_price_equals_price() {
        // When all bars have the same typical price, VWAP = that price regardless of volume
        let price = 10.0;
        let h = vec![price + 0.5; 5];
        let l = vec![price - 0.5; 5];
        let c = vec![price; 5];
        let v = vec![100.0, 200.0, 50.0, 300.0, 150.0];
        let result = vwap(&h, &l, &c, &v, 3);
        for &r in &result {
            assert!(approx_eq(r, price, 1e-10), "expected {price}, got {r}");
        }
    }

    #[test]
    fn manual_calculation() {
        // Manual: period=2
        // bar0: tp=10, vol=100  →  tp_vol=1000
        // bar1: tp=11, vol=200  →  tp_vol=2200
        // bar2: tp=12, vol=150  →  tp_vol=1800
        //
        // window [0,1]: sum_tv=3200, sum_v=300 → vwap=10.6667
        // window [1,2]: sum_tv=4000, sum_v=350 → vwap=11.4286
        let h = vec![10.5, 11.5, 12.5];
        let l = vec![ 9.5, 10.5, 11.5];
        let c = vec![10.0, 11.0, 12.0];
        let v = vec![100.0, 200.0, 150.0];
        let result = vwap(&h, &l, &c, &v, 2);
        assert_eq!(result.len(), 2);
        assert!(approx_eq(result[0], 3200.0 / 300.0, 1e-10));
        assert!(approx_eq(result[1], 4000.0 / 350.0, 1e-10));
    }

    #[test]
    fn nan_volume_propagates() {
        let h = vec![10.5, 11.5, 12.5];
        let l = vec![ 9.5, 10.5, 11.5];
        let c = vec![10.0, 11.0, 12.0];
        let v = vec![100.0, f64::NAN, 150.0];
        let result = vwap(&h, &l, &c, &v, 2);
        assert!(result[0].is_nan() || result[1].is_nan());
    }
}
```

---

## Step 2: Register in mod.rs

In `src/lib.rs` (or wherever you declare modules):

```rust
pub mod vwap;
pub use vwap::vwap;
```

---

## Step 3: Create a Polars Series Wrapper (optional)

If your project uses Polars DataFrames, add `src/vwap_series.rs`:

```rust
use polars_core::prelude::*;
use crate::vwap::vwap as vwap_core;

/// Polars Series wrapper for VWAP.
///
/// Returns a `Series` of length `close.len() - (period - 1)`.
/// All inputs are cast to `Float64` before processing.
pub fn vwap_series(
    high: &Series,
    low: &Series,
    close: &Series,
    volume: &Series,
    period: usize,
) -> PolarsResult<Series> {
    let to_f64_vec = |s: &Series| -> PolarsResult<Vec<f64>> {
        let ca = s.cast(&DataType::Float64)?;
        let ca = ca.f64()?;
        Ok(ca.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect())
    };

    let h = to_f64_vec(high)?;
    let l = to_f64_vec(low)?;
    let c = to_f64_vec(close)?;
    let v = to_f64_vec(volume)?;

    let result = vwap_core(&h, &l, &c, &v, period);
    Ok(Series::new("vwap".into(), result))
}
```

Register it in `lib.rs`:

```rust
pub mod vwap_series;
pub use vwap_series::vwap_series;
```

Usage:

```rust
use my_indicators::vwap_series;

let vwap = vwap_series(&df["high"], &df["low"], &df["close"], &df["volume"], 14)?;
// vwap is a Series of length close.len() - 13
```

---

## Step 4: Generate Golden Test Data (Python)

Create `scripts/generate_vwap_golden.py`:

```python
#!/usr/bin/env python3
"""Generate VWAP golden test files for comparison."""

import json
import numpy as np
from pathlib import Path


def rolling_vwap(high, low, close, volume, period):
    """Pure Python VWAP — the reference implementation."""
    n = len(close)
    tp = (high + low + close) / 3.0
    tp_vol = tp * volume
    out = np.full(n - period + 1, np.nan)
    for i in range(period - 1, n):
        w = i - period + 1
        sv = np.sum(volume[w:i+1])
        out[i - period + 1] = np.sum(tp_vol[w:i+1]) / sv if sv != 0 else np.nan
    return out


def write_golden(path: Path, params: dict, inputs: dict, output: np.ndarray, lookback: int):
    obj = {
        "params": params,
        "lookback": lookback,
        "input": {k: v.tolist() for k, v in inputs.items()},
        "output": {"values": [None if np.isnan(x) else x for x in output]},
    }
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w") as f:
        json.dump(obj, f, indent=2)
    print(f"  wrote {path}")


def main():
    out_dir = Path("tests/golden")
    period = 14
    lookback = period - 1
    rng = np.random.default_rng(42)
    n = 1000

    # Synthetic dataset
    close  = 100.0 + np.cumsum(rng.normal(0, 1, n))
    high   = close + rng.uniform(0.1, 2.0, n)
    low    = close - rng.uniform(0.1, 2.0, n)
    volume = rng.uniform(1000, 5000, n)

    result = rolling_vwap(high, low, close, volume, period)
    write_golden(
        out_dir / f"vwap_period{period}_normal_1000.json",
        {"period": period},
        {"high": high, "low": low, "close": close, "volume": volume},
        result,
        lookback,
    )
    print("Done.")


if __name__ == "__main__":
    main()
```

Run it:

```bash
python scripts/generate_vwap_golden.py
```

---

## Step 5: Write the Golden Test

Create `tests/golden_vwap.rs` (or place inline under `src/vwap.rs` in a `#[cfg(test)]` block for smaller projects):

```rust
use my_indicators::vwap::vwap;
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests").join("golden").join(filename)
}

fn assert_close(actual: &[f64], expected: &[Option<f64>], eps: f64, label: &str) {
    assert_eq!(actual.len(), expected.len(),
        "{label}: length mismatch: actual={} expected={}", actual.len(), expected.len());
    for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
        match e {
            None => assert!(a.is_nan(), "{label}[{i}]: expected NaN, got {a}"),
            Some(ev) => assert!(
                (a - ev).abs() < eps,
                "{label}[{i}]: |{a} - {ev}| = {} >= {eps}", (a - ev).abs()
            ),
        }
    }
}

#[test]
fn vwap_normal_1000() {
    let path = golden_path("vwap_period14_normal_1000.json");
    if !path.exists() {
        println!("SKIP: golden file not found, run generate_vwap_golden.py first");
        return;
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let period = json["params"]["period"].as_u64().unwrap() as usize;

    let get_vec = |key: &str| -> Vec<f64> {
        json["input"][key].as_array().unwrap()
            .iter().map(|v| v.as_f64().unwrap_or(f64::NAN)).collect()
    };
    let expected: Vec<Option<f64>> = json["output"]["values"].as_array().unwrap()
        .iter().map(|v| v.as_f64()).collect();

    let result = vwap(&get_vec("high"), &get_vec("low"), &get_vec("close"), &get_vec("volume"), period);
    assert_close(&result, &expected, 1e-10, "vwap_normal_1000");
}
```

Add `serde_json` to `Cargo.toml` dev-dependencies:

```toml
[dev-dependencies]
serde_json = "1"
```

---

## Step 6: Add a Criterion Benchmark (optional)

Create `benches/bench_vwap.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use my_indicators::vwap::vwap;

fn make_data(n: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let close: Vec<f64>  = (0..n).map(|i| 100.0 + (i as f64 * 0.01).sin()).collect();
    let high: Vec<f64>   = close.iter().map(|&c| c + 0.5).collect();
    let low: Vec<f64>    = close.iter().map(|&c| c - 0.5).collect();
    let volume: Vec<f64> = (0..n).map(|i| 1000.0 + (i % 100) as f64).collect();
    (high, low, close, volume)
}

fn bench_vwap(c: &mut Criterion) {
    let period = 14;
    let mut group = c.benchmark_group("vwap");
    for &size in &[100usize, 10_000, 1_000_000] {
        let (h, l, cl, v) = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("rolling", size), &size, |b, _| {
            b.iter(|| vwap(black_box(&h), black_box(&l), black_box(&cl), black_box(&v), black_box(period)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_vwap);
criterion_main!(benches);
```

Add to `Cargo.toml`:

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "bench_vwap"
harness = false
```

Run:

```bash
cargo bench
```

---

## Checklist

Copy-paste this checklist when adding any new custom indicator:

```
[ ] src/{name}.rs — algorithm on &[f64], no panics, correct output length
[ ] src/lib.rs — pub mod {name}; pub use {name}::{name};
[ ] src/{name}_series.rs — Polars Series wrapper (if using Polars)
[ ] scripts/generate_{name}_golden.py — golden data generator
[ ] tests/golden/ — golden JSON files committed to the repo
[ ] tests/golden_{name}.rs — golden test (or inline #[cfg(test)])
[ ] benches/bench_{name}.rs — Criterion benchmark (optional)
[ ] Cargo.toml dev-deps — serde_json, criterion
```

---

## Tips and Common Pitfalls

### Output length vs input length

Every windowed indicator produces **fewer outputs than inputs**:

```
output.len() == input.len() - lookback
```

When aligning with a DataFrame, prepend `lookback` null values, or use Polars `Series::extend_constant` to pad the front. Never silently truncate the price series.

### EMA seeding

ta-lib seeds EMA with `SMA(data[0..period])`, not with `data[0]`. If you build on top of `polars-ta-core`'s `ema()` function, the seeding is already correct. If you implement your own EMA, replicate this seeding exactly or your results will diverge during the warm-up bars.

### NaN handling

- If any input value in the current window is `f64::NAN`, the output for that bar should also be `NaN`.
- Use `f64::NAN` (not 0.0) for missing data when converting Polars nulls: `v.unwrap_or(f64::NAN)`.
- Test NaN propagation explicitly — it is easy to accidentally mask NaNs with arithmetic.

### Zero-volume bars (VWAP-specific)

A window where all volume is zero produces `0 / 0 = NaN`. This is correct and expected. Do not substitute 0.0.

### Integer overflow in index tracking

When storing window indices (e.g. for MAXINDEX / monotone deques), use `usize`. On 32-bit platforms, `u32` can overflow with large inputs.

### Panic vs empty return

Prefer returning an empty `Vec` over panicking when inputs are too short. This matches the polars-ta convention and keeps composable pipelines working without `unwrap` chains.
