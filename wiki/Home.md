# polars-ta Wiki

Welcome to the **polars-ta** wiki — a high-performance Rust technical analysis library with 91+ indicators, numerically aligned with ta-lib C.

## Navigation

| Page | Description |
|------|-------------|
| [Quick Start](Quick-Start) | Installation and your first indicator |
| [Indicator Reference](Indicator-Reference) | Complete list of all 91+ implemented indicators |
| [Performance](Performance) | Benchmark methodology and results |
| [Contributing](Contributing) | How to add new indicators |

## Overview

**polars-ta** provides:
- **91+ indicators** across 8 categories (trend, oscillator, volume, volatility, statistics, price transform, math transform, math operators)
- **Numerical parity** with ta-lib C — golden tests verify < 1e-10 error on all indicators
- **Native Polars Series API** — no manual array management
- **Zero-dependency core** (`polars-ta-core`) — suitable for embedded/WASM contexts
- **AI-friendly architecture** — see [docs/AI_GUIDE.md](../docs/AI_GUIDE.md)

## Repository Structure

```
ta-lib-rs/
├── crates/
│   ├── polars-ta-core/     # Pure algorithm layer (&[f64], zero deps)
│   ├── polars-ta/          # Polars Series wrappers
│   ├── polars-ta-plugin/   # Python Polars plugin (pyo3)
│   └── polars-ta-verify/   # Golden tests + Criterion benchmarks
├── tests/golden/           # Golden JSON test fixtures
├── scripts/                # Benchmark and test helper scripts
├── docs/                   # AI_GUIDE.md, CUSTOM_INDICATOR.md
└── wiki/                   # This wiki content (static reference)
```

## Quick Links

- [Crates.io](https://crates.io/crates/polars-ta)
- [docs.rs](https://docs.rs/polars-ta)
- [GitHub Actions CI](https://github.com/map-A/ta-lib-rs/actions/workflows/ci.yml)
