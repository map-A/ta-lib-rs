# Contributing

## Adding a New Indicator

The full step-by-step guide lives in [docs/CUSTOM_INDICATOR.md](../docs/CUSTOM_INDICATOR.md). Here is the quick overview:

### 7-Step Process

1. **Add the core function** in `crates/polars-ta-core/src/<category>/<name>.rs`
2. **Re-export** from the category `mod.rs`
3. **Add the Series wrapper** in `crates/polars-ta/src/<category>/mod.rs`
4. **Generate golden tests** with `python scripts/generate_golden.py --indicator <name>`
5. **Add the golden test runner** in `crates/polars-ta-verify/src/indicators/<name>.rs`
6. **Add a Criterion benchmark** in `crates/polars-ta-verify/benches/bench_vs_talib.rs`
7. **Update README** indicator table

### Documentation Requirements

Each new indicator module (`//!` doc comment) must include:

```rust
//! Indicator Name (ABBREV)
//!
//! English description. Chinese description (中文说明).
//!
//! # Algorithm
//! ```text
//! formula here
//! ```
//!
//! # Parameters
//! - `param1` — description
//!
//! # Output
//! - Length = `n - lookback`
//!
//! # Example
//! ```rust
//! use polars_ta_core::category::function_name;
//! let result = function_name(&data, period);
//! assert_eq!(result.len(), expected_len);
//! ```
```

### Precision Standard

Golden tests require all outputs to be within **1e-10** of ta-lib C output. Use `assert_approx_eq!(result[i], expected[i], 1e-10)`.

### Performance Gate

New indicators must achieve ≥ **80%** of ta-lib C throughput at 1,000,000 elements. Check with `./scripts/compare_all.sh --indicator=<name>`.

## Code Style

- No `unsafe` unless profiling shows a measurable speedup
- No `TODO` comments — either implement or remove
- No `any` types (N/A for Rust, but avoid `dyn Any` patterns)
- Comments in Chinese are fine for algorithm explanations
- Public API docs in English

## Testing

```bash
# Run all golden tests
cargo test --workspace --release -q

# Run tests for a specific package
cargo test --package polars-ta-core

# Run with output
cargo test -- --nocapture
```

## PR Checklist

- [ ] Core function implemented and exported
- [ ] Module-level `//!` doc comment with all required sections
- [ ] Golden test fixtures generated and committed
- [ ] Golden test runner added
- [ ] Criterion benchmark added
- [ ] README indicator table updated
- [ ] `cargo test --workspace --release -q` passes
- [ ] Performance ≥ 80% of ta-lib C
