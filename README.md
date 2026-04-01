# polars-ta

> **A Rust technical indicator library with a native Polars Series API, numerically aligned with ta-lib C.**

[![Crates.io](https://img.shields.io/crates/v/polars-ta.svg)](https://crates.io/crates/polars-ta)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-807%20passing-brightgreen)]()

## Features

- 🚀 **Native Polars interface** — operate directly on `Series`, zero-copy
- 🎯 **Numerically aligned** — Golden Tests guarantee < 1e-10 error vs ta-lib C for all 91 indicators
- ⚡ **High performance** — key indicators exceed ta-lib C throughput (BBands 230%, AD 118%, ADX 110%)
- 🔌 **Zero-dependency core** — `polars-ta-core` has no external deps; usable in embedded/WASM contexts
- 🤖 **AI-friendly** — complete documentation and architecture for AI-assisted extension

## Quick Start

```toml
# Cargo.toml
[dependencies]
polars-ta = "0.1"
polars = "0.46"
```

### Single-output indicator (SMA)

```rust
use polars::prelude::*;
use polars_ta::trend::sma_series;

fn main() -> PolarsResult<()> {
    let close = Series::new("close".into(), &[1.0f64, 2.0, 3.0, 4.0, 5.0]);

    // SMA(period=3): output length = 5 - (3-1) = 3
    let sma = sma_series(&close, 3)?;
    println!("{sma}");  // [2.0, 3.0, 4.0]

    Ok(())
}
```

### Multi-output indicator (MACD)

```rust
use polars::prelude::*;
use polars_ta::trend::macd_series;

fn main() -> PolarsResult<()> {
    let close = Series::new("close".into(), vec![/* price data */]);

    // Returns (macd_line, signal_line, histogram) — three Series of equal length
    let (macd_line, signal, hist) = macd_series(&close, 12, 26, 9)?;
    println!("MACD:   {macd_line}");
    println!("Signal: {signal}");
    println!("Hist:   {hist}");

    Ok(())
}
```

## Output Convention

All indicators follow the ta-lib C API convention: **output is shorter than input**.

```
output_length = input_length - lookback

Example — SMA(period=3), lookback=2:
  input:  [v0, v1, v2, v3, v4]   len=5
  output:       [v2', v3', v4']  len=3
```

The caller is responsible for index-aligning the output with the original DataFrame. The `lookback` value for each indicator is documented in its function signature.

## Complete Indicator Reference

### Trend (14) ✅

| Function | Name |
|----------|------|
| `sma` | Simple Moving Average |
| `ema` | Exponential Moving Average |
| `wma` | Weighted Moving Average |
| `dema` | Double EMA |
| `tema` | Triple EMA |
| `kama` | Kaufman Adaptive MA |
| `trima` | Triangular MA |
| `t3` | Triple Exponential MA (T3) |
| `ma` | Moving Average (adaptive, selects by type) |
| `macd` | MACD |
| `macdext` | MACD with Controllable MA Type |
| `macdfix` | MACD with Fixed 12/26 Periods |
| `bbands` | Bollinger Bands |
| `midpoint` | Midpoint over Period |
| `midprice` | Midpoint Price over Period |
| `sar` | Parabolic SAR |
| `sarext` | Parabolic SAR — Extended |
| `adx` | Average Directional Index |
| `adxr` | ADX Rating |
| `minus_di` | Minus Directional Indicator |
| `plus_di` | Plus Directional Indicator |
| `minus_dm` | Minus Directional Movement |
| `plus_dm` | Plus Directional Movement |
| `dx` | Directional Movement Index |

### Oscillators (22) ✅

| Function | Name |
|----------|------|
| `rsi` | Relative Strength Index |
| `stoch` | Stochastic |
| `stochf` | Stochastic Fast |
| `stochrsi` | Stochastic RSI |
| `cci` | Commodity Channel Index |
| `willr` | Williams %R |
| `ultosc` | Ultimate Oscillator |
| `aroon` | Aroon |
| `aroonosc` | Aroon Oscillator |
| `mfi` | Money Flow Index |
| `mom` | Momentum |
| `roc` | Rate of Change |
| `rocp` | Rate of Change Percentage |
| `rocr` | Rate of Change Ratio |
| `rocr100` | Rate of Change Ratio ×100 |
| `cmo` | Chande Momentum Oscillator |
| `apo` | Absolute Price Oscillator |
| `ppo` | Percentage Price Oscillator |
| `trix` | 1-day Rate-of-Change of Triple-Smooth EMA |
| `bop` | Balance of Power |
| `adxr` | ADX Rating |
| `dx` | Directional Movement Index |

### Volume (3) ✅

| Function | Name |
|----------|------|
| `obv` | On Balance Volume |
| `ad` | Chaikin A/D Line |
| `adosc` | Chaikin A/D Oscillator |

### Volatility (3) ✅

| Function | Name |
|----------|------|
| `trange` | True Range |
| `atr` | Average True Range |
| `natr` | Normalized ATR |
| `beta` | Beta |

### Statistics (9) ✅

| Function | Name |
|----------|------|
| `beta` | Beta |
| `correl` | Pearson's Correlation Coefficient |
| `linearreg` | Linear Regression |
| `linearreg_angle` | Linear Regression Angle |
| `linearreg_intercept` | Linear Regression Intercept |
| `linearreg_slope` | Linear Regression Slope |
| `stddev` | Standard Deviation |
| `tsf` | Time Series Forecast |
| `var` | Variance |

### Price Transform (4) ✅

| Function | Name |
|----------|------|
| `avgprice` | Average Price `(O+H+L+C)/4` |
| `medprice` | Median Price `(H+L)/2` |
| `typprice` | Typical Price `(H+L+C)/3` |
| `wclprice` | Weighted Close Price `(H+L+2C)/4` |

### Math Transform (15) ✅

All element-wise (lookback = 0, output length = input length):

| Function | Name |
|----------|------|
| `acos` | Arc Cosine |
| `asin` | Arc Sine |
| `atan` | Arc Tangent |
| `ceil` | Vector Ceiling |
| `cos` | Cosine |
| `cosh` | Hyperbolic Cosine |
| `exp` | Exponential |
| `floor` | Vector Floor |
| `ln` | Natural Logarithm |
| `log10` | Base-10 Logarithm |
| `sin` | Sine |
| `sinh` | Hyperbolic Sine |
| `sqrt` | Square Root |
| `tan` | Tangent |
| `tanh` | Hyperbolic Tangent |

### Math Operators (11) ✅

| Function | Name | Notes |
|----------|------|-------|
| `add` | Element-wise Addition | lookback=0 |
| `div` | Element-wise Division | lookback=0 |
| `mult` | Element-wise Multiplication | lookback=0 |
| `sub` | Element-wise Subtraction | lookback=0 |
| `max` | Highest value over period | O(n) monotone deque |
| `min` | Lowest value over period | O(n) monotone deque |
| `sum` | Summation over period | O(n) sliding window |
| `maxindex` | Index of highest value | O(n) monotone deque |
| `minindex` | Index of lowest value | O(n) monotone deque |
| `minmax` | Lowest and highest over period | O(n) |
| `minmaxindex` | Indexes of lowest and highest | O(n) |

## Performance

Benchmarked on Apple M-series (1,000,000 elements, period=20 unless noted). Ratio = polars-ta / ta-lib C throughput.

| Indicator | polars-ta (Melems/s) | ta-lib C (Melems/s) | Ratio |
|-----------|--------------------:|--------------------:|------:|
| BBands    | ~1150               | ~500                | **230%** |
| AD        | ~590                | ~500                | **118%** |
| ADX       | ~220                | ~200                | **110%** |
| TRange    | ~460                | ~500                | 92% |
| ADOSC     | ~410                | ~500                | 82% |
| OBV       | ~415                | ~500                | 83% |

> All Phase 1 indicators exceed the 80% performance gate. Run benchmarks with `cargo bench --package polars-ta-verify`.

## Architecture

```
ta-lib-rs/
├── crates/
│   ├── polars-ta-core/        # Pure algorithm layer (&[f64], zero deps, no_std-compatible)
│   │   └── src/
│   │       ├── trend/         # SMA, EMA, WMA, DEMA, TEMA, KAMA, TRIMA, T3, MA,
│   │       │                  # MACD, MACDEXT, MACDFIX, BBands, MidPoint, MidPrice,
│   │       │                  # SAR, SAREXT, ADX, ADXR, ±DI, ±DM, DX
│   │       ├── oscillator/    # RSI, Stoch, StochF, StochRSI, CCI, WillR, UltOsc,
│   │       │                  # Aroon, AroonOsc, MFI, MOM, ROC/ROCP/ROCR/ROCR100,
│   │       │                  # CMO, APO, PPO, TRIX, BOP
│   │       ├── volume/        # OBV, AD, ADOSC
│   │       ├── volatility/    # TRange, ATR, NATR
│   │       ├── statistic/     # Beta, Correl, LinearReg (×4), StdDev, TSF, Var
│   │       ├── price_transform/ # AvgPrice, MedPrice, TyptPrice, WclPrice
│   │       ├── math_transform/  # ACOS, ASIN, ATAN, CEIL, COS, COSH, EXP, FLOOR,
│   │       │                    # LN, LOG10, SIN, SINH, SQRT, TAN, TANH
│   │       └── math_ops/        # ADD, DIV, MULT, SUB, MAX, MIN, SUM,
│   │                            # MAXINDEX, MININDEX, MINMAX, MINMAXINDEX
│   ├── polars-ta/             # Polars Series wrappers (main user-facing API)
│   │   └── src/
│   │       ├── trend/
│   │       ├── oscillator/
│   │       ├── volume/
│   │       └── volatility/
│   ├── polars-ta-plugin/      # Python Polars plugin (pyo3)
│   └── polars-ta-verify/      # Golden test framework + Criterion benchmarks
├── tests/golden/              # Golden JSON files (version-controlled)
├── scripts/                   # generate_golden.py, compare_all.sh, run_golden.sh
└── docs/                      # AI_GUIDE.md, CUSTOM_INDICATOR.md
```

**Data flow**: raw `&[f64]` arrays → `polars-ta-core` → `Vec<f64>` results → `polars-ta` wraps in `Series`.

## Validation & Benchmarks

### Run golden tests

```bash
# Generate golden JSON files (requires Python + ta-lib)
python scripts/generate_golden.py

# Run all 807 golden tests
cargo test --package polars-ta-verify

# One-liner
./scripts/run_golden.sh
```

### Compare throughput vs ta-lib C

```bash
# Requires Python + ta-lib installed
./scripts/compare_all.sh
```

### Criterion micro-benchmarks

```bash
cargo bench --package polars-ta-verify
```

## Extending with Custom Indicators

See [docs/CUSTOM_INDICATOR.md](docs/CUSTOM_INDICATOR.md) for a complete step-by-step guide including a full working VWAP example.

## Development Guide

See [docs/AI_GUIDE.md](docs/AI_GUIDE.md) — designed for AI-assisted development. Contains:
- New indicator implementation template (7 steps)
- Golden test failure debugging flow
- Precision standards and performance checklist

## License

MIT
