# Performance

## Methodology

Benchmarks are run with [Criterion.rs](https://github.com/bheisler/criterion.rs) using:
- **Data size**: 1,000,000 elements (f64)
- **Period**: 20 (unless noted)
- **Iterations**: 100 samples, 3 s measurement, 1 s warm-up
- **Machine**: Apple M-series (arm64, single core)

ta-lib C throughput is measured via the Python `ta-lib` wrapper using `timeit`.

## Results Summary

| Indicator | polars-ta (Melem/s) | ta-lib C (Melem/s) | Ratio | Status |
|-----------|--------------------:|-------------------:|------:|--------|
| BBands    | ~1150 | ~500 | **230%** | ✅ |
| AD        | ~590  | ~500 | **118%** | ✅ |
| ADX       | ~220  | ~200 | **110%** | ✅ |
| TRange    | ~460  | ~500 | 92%      | ✅ |
| ADOSC     | ~410  | ~500 | 82%      | ✅ |
| OBV       | ~415  | ~500 | 83%      | ✅ |

> **Gate**: polars-ta must achieve ≥ 80% of ta-lib C throughput. All Phase 1 indicators pass.

## Why polars-ta Can Be Faster

1. **SIMD-friendly inner loops**: tight loops with f64 arithmetic compile to vectorized instructions
2. **O(n) algorithms**: MAX/MIN use monotone deque (vs. O(n·period) naive scan in some ta-lib paths)
3. **Zero-copy Polars integration**: Series data is passed as `&[f64]` without copying
4. **No FFI overhead**: pure Rust, no C function call boundary

## Running Benchmarks

```bash
cd /path/to/ta-lib-rs
source .venv/bin/activate

# Full comparison (requires Python ta-lib)
./scripts/compare_all.sh

# Quick mode (faster, fewer samples)
./scripts/compare_all.sh --quick

# Single indicator
./scripts/compare_all.sh --indicator=sma

# Criterion micro-benchmarks only
cargo bench --package polars-ta-verify

# Generate Markdown report
./scripts/compare_all.sh --report
```

## Performance Report Format

The `--report` flag on `compare_all.sh` generates a Markdown file at `polars-ta-report.md` with:
- Run timestamp and system info
- Full 158-indicator table (91 implemented + 67 planned)
- Status icons: ✅ (≥95%), ⚠️ (80–95%), ❌ (<80%), ⏳ (not implemented)

## Criterion HTML Report

After running `cargo bench`, open the HTML report:

```bash
open target/criterion/report/index.html
```
