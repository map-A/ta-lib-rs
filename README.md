# polars-ta: Technical Analysis Library in Pure Rust

[![CI](https://github.com/map-A/ta-lib-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/map-A/ta-lib-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/polars-ta-core.svg)](https://crates.io/crates/polars-ta-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

[中文文档](README.zh.md) | [API Docs](https://docs.rs/polars-ta-core) | [Wiki](https://github.com/map-A/ta-lib-rs/wiki)

A complete Rust implementation of all 158 ta-lib technical indicators, with:
- 🦀 **Pure Rust** — no C dependencies, safe code
- ⚡ **High Performance** — competitive with or faster than ta-lib on most indicators
- 🐼 **Polars Integration** — first-class Series/DataFrame API
- 🤖 **AI-Friendly** — comprehensive documentation for LLM-assisted development
- ✅ **Golden Tests** — 684/688 test cases pass vs ta-lib reference implementation

## Quick Start

### Install

```toml
[dependencies]
polars-ta-core = "0.1"
```

### Basic Usage

```rust
use polars_ta_core::trend::sma;
use polars_ta_core::oscillator::rsi;

let prices: Vec<f64> = vec![...];
let sma_values = sma(&prices, 20);
let rsi_values = rsi(&prices, 14);
```

### With Polars

```rust
use polars::prelude::*;
use polars_ta::trend::sma_series;

let series = Series::new("close".into(), &prices);
let result = sma_series(&series, 20)?;
```

## Run Comparison Benchmarks

```bash
git clone https://github.com/map-A/ta-lib-rs.git
cd ta-lib-rs
./scripts/run_golden.sh          # run golden tests vs ta-lib
./scripts/bench_all.sh           # run full performance comparison
```

## Performance Comparison

All benchmarks run on n=10,000 data points (µs = microseconds, median latency).  
Ratio = polars-ta / ta-lib (lower is better; `N/A` means ta-lib doesn't expose a Python benchmark for this indicator).

| # | Indicator | Description | Category | ta-lib (µs) | polars-ta (µs) | Ratio | Status |
|---|-----------|-------------|----------|-------------|----------------|-------|--------|
| 1 | `BBANDS` | Bollinger Bands | Overlap Studies | 37.7 | 44.3 | 1.17x | ✅ ≥67% |
| 2 | `DEMA` | Double Exponential Moving Average | Overlap Studies | 30.2 | 37.8 | 1.25x | ✅ ≥67% |
| 3 | `EMA` | Exponential Moving Average | Overlap Studies | 13.8 | 16.8 | 1.22x | ✅ ≥67% |
| 4 | `HT_TRENDLINE` | Hilbert Transform - Instantaneous Trendline | Overlap Studies | N/A | 2323.5 | N/A | ✅ Implemented |
| 5 | `KAMA` | Kaufman Adaptive Moving Average | Overlap Studies | 15.3 | 20.3 | 1.33x | ✅ ≥67% |
| 6 | `MA` | Moving Average | Overlap Studies | 13.8 | 14.4 | 1.05x | ✅ ≥67% |
| 7 | `MAMA` | MESA Adaptive Moving Average | Overlap Studies | N/A | 328.5 | N/A | ✅ Implemented |
| 8 | `MAVP` | Moving Average with Variable Period | Overlap Studies | N/A | 12.1 | N/A | ✅ Implemented |
| 9 | `MIDPOINT` | MidPoint over period | Overlap Studies | 55.4 | 29.3 | 0.53x | 🚀 Faster |
| 10 | `MIDPRICE` | Midpoint Price over period | Overlap Studies | 54.8 | 38.6 | 0.70x | 🚀 Faster |
| 11 | `SAR` | Parabolic SAR | Overlap Studies | 30.7 | 27.1 | 0.89x | 🚀 Faster |
| 12 | `SAREXT` | Parabolic SAR - Extended | Overlap Studies | 30.5 | 28.5 | 0.93x | 🚀 Faster |
| 13 | `SMA` | Simple Moving Average | Overlap Studies | 10.7 | 7.5 | 0.70x | 🚀 Faster |
| 14 | `T3` | Triple Exponential Moving Average (T3) | Overlap Studies | 18.4 | 20.4 | 1.11x | ✅ ≥67% |
| 15 | `TEMA` | Triple Exponential Moving Average | Overlap Studies | 44.5 | 44.7 | 1.00x | ✅ ≥67% |
| 16 | `TRIMA` | Triangular Moving Average | Overlap Studies | 19.4 | 12.9 | 0.66x | 🚀 Faster |
| 17 | `WMA` | Weighted Moving Average | Overlap Studies | 13.5 | 13.2 | 0.98x | 🚀 Faster |
| 18 | `ADX` | Average Directional Movement Index | Momentum | 41.3 | 15.0 | 0.36x | 🚀 Faster |
| 19 | `ADXR` | Average Directional Movement Index Rating | Momentum | 47.7 | 16.9 | 0.35x | 🚀 Faster |
| 20 | `APO` | Absolute Price Oscillator | Momentum | 23.2 | 11.1 | 0.48x | 🚀 Faster |
| 21 | `AROON` | Aroon | Momentum | 48.7 | 39.9 | 0.82x | 🚀 Faster |
| 22 | `AROONOSC` | Aroon Oscillator | Momentum | 46.7 | 43.0 | 0.92x | 🚀 Faster |
| 23 | `BOP` | Balance Of Power | Momentum | 6.0 | 3.6 | 0.60x | 🚀 Faster |
| 24 | `CCI` | Commodity Channel Index | Momentum | 67.8 | 43.7 | 0.64x | 🚀 Faster |
| 25 | `CMO` | Chande Momentum Oscillator | Momentum | 43.6 | 14.6 | 0.34x | 🚀 Faster |
| 26 | `DX` | Directional Movement Index | Momentum | 42.0 | 21.8 | 0.52x | 🚀 Faster |
| 27 | `MACD` | Moving Average Convergence/Divergence | Momentum | 47.8 | 46.2 | 0.97x | 🚀 Faster |
| 28 | `MACDEXT` | MACD with controllable MA type | Momentum | 47.3 | 28.4 | 0.60x | 🚀 Faster |
| 29 | `MACDFIX` | Moving Average Convergence/Divergence Fix 12/26 | Momentum | 47.8 | 36.4 | 0.76x | 🚀 Faster |
| 30 | `MFI` | Money Flow Index | Momentum | 15.1 | 14.9 | 0.98x | 🚀 Faster |
| 31 | `MINUS_DI` | Minus Directional Indicator | Momentum | 38.5 | 23.6 | 0.61x | 🚀 Faster |
| 32 | `MINUS_DM` | Minus Directional Movement | Momentum | 38.3 | 17.0 | 0.45x | 🚀 Faster |
| 33 | `MOM` | Momentum | Momentum | 1.8 | 1.3 | 0.73x | 🚀 Faster |
| 34 | `PLUS_DI` | Plus Directional Indicator | Momentum | 38.4 | 23.6 | 0.61x | 🚀 Faster |
| 35 | `PLUS_DM` | Plus Directional Movement | Momentum | 38.4 | 17.1 | 0.45x | 🚀 Faster |
| 36 | `PPO` | Percentage Price Oscillator | Momentum | 26.2 | 9.7 | 0.37x | 🚀 Faster |
| 37 | `ROC` | Rate of change | Momentum | 5.5 | 3.2 | 0.59x | 🚀 Faster |
| 38 | `ROCP` | Rate of change Percentage | Momentum | 5.6 | 3.0 | 0.53x | 🚀 Faster |
| 39 | `ROCR` | Rate of change ratio | Momentum | 5.6 | 2.2 | 0.38x | 🚀 Faster |
| 40 | `ROCR100` | Rate of change ratio 100 scale | Momentum | 5.3 | 2.4 | 0.46x | 🚀 Faster |
| 41 | `RSI` | Relative Strength Index | Momentum | 38.2 | 37.3 | 0.98x | 🚀 Faster |
| 42 | `STOCH` | Stochastic | Momentum | 39.8 | 60.3 | 1.52x | ✅ ≥50% |
| 43 | `STOCHF` | Stochastic Fast | Momentum | 31.8 | 29.8 | 0.94x | 🚀 Faster |
| 44 | `STOCHRSI` | Stochastic Relative Strength Index | Momentum | 67.1 | 78.0 | 1.16x | ✅ ≥67% |
| 45 | `TRIX` | 1-day Rate-Of-Change (ROC) of a Triple Smooth EMA | Momentum | 46.4 | 46.3 | 1.00x | 🚀 Faster |
| 46 | `ULTOSC` | Ultimate Oscillator | Momentum | 39.0 | 31.7 | 0.81x | 🚀 Faster |
| 47 | `WILLR` | Williams' %R | Momentum | 49.8 | 67.4 | 1.35x | ✅ ≥67% |
| 48 | `AD` | Chaikin A/D Line | Volume | 9.8 | 7.9 | 0.81x | 🚀 Faster |
| 49 | `ADOSC` | Chaikin A/D Oscillator | Volume | 19.2 | 16.0 | 0.83x | 🚀 Faster |
| 50 | `OBV` | On Balance Volume | Volume | 7.5 | 7.8 | 1.04x | ✅ ≥67% |
| 51 | `ATR` | Average True Range | Volatility | 44.5 | 18.9 | 0.43x | 🚀 Faster |
| 52 | `NATR` | Normalized Average True Range | Volatility | 44.5 | 20.5 | 0.46x | 🚀 Faster |
| 53 | `TRANGE` | True Range | Volatility | 3.5 | 4.6 | 1.30x | ✅ ≥67% |
| 54 | `AVGPRICE` | Average Price | Price Transform | 4.1 | 3.5 | 0.87x | 🚀 Faster |
| 55 | `MEDPRICE` | Median Price | Price Transform | 2.5 | 2.0 | 0.78x | 🚀 Faster |
| 56 | `TYPPRICE` | Typical Price | Price Transform | 3.7 | 3.0 | 0.82x | 🚀 Faster |
| 57 | `WCLPRICE` | Weighted Close Price | Price Transform | 3.4 | 3.0 | 0.89x | 🚀 Faster |
| 58 | `HT_DCPERIOD` | Hilbert Transform - Dominant Cycle Period | Cycle | N/A | 2319.1 | N/A | ✅ Implemented |
| 59 | `HT_DCPHASE` | Hilbert Transform - Dominant Cycle Phase | Cycle | N/A | 2332.3 | N/A | ✅ Implemented |
| 60 | `HT_PHASOR` | Hilbert Transform - Phasor Components | Cycle | N/A | 2349.2 | N/A | ✅ Implemented |
| 61 | `HT_SINE` | Hilbert Transform - SineWave | Cycle | N/A | 2311.2 | N/A | ✅ Implemented |
| 62 | `HT_TRENDMODE` | Hilbert Transform - Trend vs Cycle Mode | Cycle | N/A | 2356.5 | N/A | ✅ Implemented |
| 63 | `CDL2CROWS` | Two Crows | Pattern | N/A | 12.8 | N/A | ✅ Implemented |
| 64 | `CDL3BLACKCROWS` | Three Black Crows | Pattern | N/A | 16.6 | N/A | ✅ Implemented |
| 65 | `CDL3INSIDE` | Three Inside Up/Down | Pattern | N/A | 14.8 | N/A | ✅ Implemented |
| 66 | `CDL3LINESTRIKE` | Three-Line Strike | Pattern | N/A | 13.2 | N/A | ✅ Implemented |
| 67 | `CDL3STARSINSOUTH` | Three Stars In The South | Pattern | N/A | 15.3 | N/A | ✅ Implemented |
| 68 | `CDL3WHITESOLDIERS` | Three Advancing White Soldiers | Pattern | N/A | 13.5 | N/A | ✅ Implemented |
| 69 | `CDLABANDONEDBABY` | Abandoned Baby | Pattern | N/A | 16.2 | N/A | ✅ Implemented |
| 70 | `CDLADVANCEBLOCK` | Advance Block | Pattern | N/A | 32.4 | N/A | ✅ Implemented |
| 71 | `CDLBELTHOLD` | Belt-hold | Pattern | N/A | 14.3 | N/A | ✅ Implemented |
| 72 | `CDLBREAKAWAY` | Breakaway | Pattern | N/A | 13.3 | N/A | ✅ Implemented |
| 73 | `CDLCLOSINGMARUBOZU` | Closing Marubozu | Pattern | N/A | 14.2 | N/A | ✅ Implemented |
| 74 | `CDLCONCEALBABYSWALL` | Concealing Baby Swallow | Pattern | N/A | 14.9 | N/A | ✅ Implemented |
| 75 | `CDLCOUNTERATTACK` | Counterattack | Pattern | N/A | 14.9 | N/A | ✅ Implemented |
| 76 | `CDLDARKCLOUDCOVER` | Dark Cloud Cover | Pattern | N/A | 13.1 | N/A | ✅ Implemented |
| 77 | `CDLDOJI` | Doji | Pattern | N/A | 12.6 | N/A | ✅ Implemented |
| 78 | `CDLDOJISTAR` | Doji Star | Pattern | N/A | 14.4 | N/A | ✅ Implemented |
| 79 | `CDLDRAGONFLYDOJI` | Dragonfly Doji | Pattern | N/A | 12.7 | N/A | ✅ Implemented |
| 80 | `CDLENGULFING` | Engulfing Pattern | Pattern | N/A | 5.9 | N/A | ✅ Implemented |
| 81 | `CDLEVENINGDOJISTAR` | Evening Doji Star | Pattern | N/A | 14.5 | N/A | ✅ Implemented |
| 82 | `CDLEVENINGSTAR` | Evening Star | Pattern | N/A | 14.1 | N/A | ✅ Implemented |
| 83 | `CDLGAPSIDESIDEWHITE` | Up/Down-gap side-by-side white lines | Pattern | N/A | 16.7 | N/A | ✅ Implemented |
| 84 | `CDLGRAVESTONEDOJI` | Gravestone Doji | Pattern | N/A | 12.8 | N/A | ✅ Implemented |
| 85 | `CDLHAMMER` | Hammer | Pattern | N/A | 15.1 | N/A | ✅ Implemented |
| 86 | `CDLHANGINGMAN` | Hanging Man | Pattern | N/A | 15.2 | N/A | ✅ Implemented |
| 87 | `CDLHARAMI` | Harami Pattern | Pattern | N/A | 18.8 | N/A | ✅ Implemented |
| 88 | `CDLHARAMICROSS` | Harami Cross Pattern | Pattern | N/A | 10.2 | N/A | ✅ Implemented |
| 89 | `CDLHIGHWAVE` | High-Wave Candle | Pattern | N/A | 9.5 | N/A | ✅ Implemented |
| 90 | `CDLHIKKAKE` | Hikkake Pattern | Pattern | N/A | 6.3 | N/A | ✅ Implemented |
| 91 | `CDLHIKKAKEMOD` | Modified Hikkake Pattern | Pattern | N/A | 8.7 | N/A | ✅ Implemented |
| 92 | `CDLHOMINGPIGEON` | Homing Pigeon | Pattern | N/A | 14.4 | N/A | ✅ Implemented |
| 93 | `CDLIDENTICAL3CROWS` | Identical Three Crows | Pattern | N/A | 12.8 | N/A | ✅ Implemented |
| 94 | `CDLINNECK` | In-Neck Pattern | Pattern | N/A | 14.6 | N/A | ✅ Implemented |
| 95 | `CDLINVERTEDHAMMER` | Inverted Hammer | Pattern | N/A | 12.7 | N/A | ✅ Implemented |
| 96 | `CDLKICKING` | Kicking | Pattern | N/A | 20.2 | N/A | ✅ Implemented |
| 97 | `CDLKICKINGBYLENGTH` | Kicking - bull/bear determined by the longer marubozu | Pattern | N/A | 20.9 | N/A | ✅ Implemented |
| 98 | `CDLLADDERBOTTOM` | Ladder Bottom | Pattern | N/A | 15.3 | N/A | ✅ Implemented |
| 99 | `CDLLONGLEGGEDDOJI` | Long Legged Doji | Pattern | N/A | 12.8 | N/A | ✅ Implemented |
| 100 | `CDLLONGLINE` | Long Line Candle | Pattern | N/A | 15.2 | N/A | ✅ Implemented |
| 101 | `CDLMARUBOZU` | Marubozu | Pattern | N/A | 14.1 | N/A | ✅ Implemented |
| 102 | `CDLMATCHINGLOW` | Matching Low | Pattern | N/A | 13.9 | N/A | ✅ Implemented |
| 103 | `CDLMATHOLD` | Mat Hold | Pattern | N/A | 14.7 | N/A | ✅ Implemented |
| 104 | `CDLMORNINGDOJISTAR` | Morning Doji Star | Pattern | N/A | 14.5 | N/A | ✅ Implemented |
| 105 | `CDLMORNINGSTAR` | Morning Star | Pattern | N/A | 14.0 | N/A | ✅ Implemented |
| 106 | `CDLONNECK` | On-Neck Pattern | Pattern | N/A | 14.5 | N/A | ✅ Implemented |
| 107 | `CDLPIERCING` | Piercing Pattern | Pattern | N/A | 12.0 | N/A | ✅ Implemented |
| 108 | `CDLRICKSHAWMAN` | Rickshaw Man | Pattern | N/A | 17.6 | N/A | ✅ Implemented |
| 109 | `CDLRISEFALL3METHODS` | Rising/Falling Three Methods | Pattern | N/A | 13.1 | N/A | ✅ Implemented |
| 110 | `CDLSEPARATINGLINES` | Separating Lines | Pattern | N/A | 13.2 | N/A | ✅ Implemented |
| 111 | `CDLSHOOTINGSTAR` | Shooting Star | Pattern | N/A | 14.5 | N/A | ✅ Implemented |
| 112 | `CDLSHORTLINE` | Short Line Candle | Pattern | N/A | 15.0 | N/A | ✅ Implemented |
| 113 | `CDLSPINNINGTOP` | Spinning Top | Pattern | N/A | 13.6 | N/A | ✅ Implemented |
| 114 | `CDLSTALLEDPATTERN` | Stalled Pattern | Pattern | N/A | 13.5 | N/A | ✅ Implemented |
| 115 | `CDLSTICKSANDWICH` | Stick Sandwich | Pattern | N/A | 13.2 | N/A | ✅ Implemented |
| 116 | `CDLTAKURI` | Takuri (Dragonfly Doji with very long lower shadow) | Pattern | N/A | 13.0 | N/A | ✅ Implemented |
| 117 | `CDLTASUKIGAP` | Tasuki Gap | Pattern | N/A | 15.8 | N/A | ✅ Implemented |
| 118 | `CDLTHRUSTING` | Thrusting Pattern | Pattern | N/A | 14.6 | N/A | ✅ Implemented |
| 119 | `CDLTRISTAR` | Tristar Pattern | Pattern | N/A | 13.2 | N/A | ✅ Implemented |
| 120 | `CDLUNIQUE3RIVER` | Unique 3 River | Pattern | N/A | 12.8 | N/A | ✅ Implemented |
| 121 | `CDLUPSIDEGAP2CROWS` | Upside Gap Two Crows | Pattern | N/A | 12.8 | N/A | ✅ Implemented |
| 122 | `CDLXSIDEGAP3METHODS` | Upside/Downside Gap Three Methods | Pattern | N/A | 10.2 | N/A | ✅ Implemented |
| 123 | `BETA` | Beta | Statistics | 34.6 | 32.2 | 0.93x | 🚀 Faster |
| 124 | `CORREL` | Pearson's Correlation Coefficient (r) | Statistics | 23.3 | 31.4 | 1.34x | ✅ ≥67% |
| 125 | `LINEARREG` | Linear Regression | Statistics | 43.7 | 14.0 | 0.32x | 🚀 Faster |
| 126 | `LINEARREG_ANGLE` | Linear Regression Angle | Statistics | N/A | 30.2 | N/A | ✅ Implemented |
| 127 | `LINEARREG_INTERCEPT` | Linear Regression Intercept | Statistics | N/A | 13.2 | N/A | ✅ Implemented |
| 128 | `LINEARREG_SLOPE` | Linear Regression Slope | Statistics | N/A | 13.3 | N/A | ✅ Implemented |
| 129 | `STDDEV` | Standard Deviation | Statistics | 16.1 | 10.3 | 0.64x | 🚀 Faster |
| 130 | `TSF` | Time Series Forecast | Statistics | 43.2 | 13.6 | 0.31x | 🚀 Faster |
| 131 | `VAR` | Variance | Statistics | 13.5 | 9.1 | 0.68x | 🚀 Faster |
| 132 | `ACOS` | Vector Trigonometric ACos | Math Transform | 25.0 | 16.4 | 0.65x | 🚀 Faster |
| 133 | `ASIN` | Vector Trigonometric ASin | Math Transform | 27.1 | 18.8 | 0.69x | 🚀 Faster |
| 134 | `ATAN` | Vector Trigonometric ATan | Math Transform | 27.6 | 26.8 | 0.97x | 🚀 Faster |
| 135 | `CEIL` | Vector Ceil | Math Transform | 1.3 | 1.4 | 1.06x | ✅ ≥67% |
| 136 | `COS` | Vector Trigonometric Cos | Math Transform | 27.0 | 26.8 | 0.99x | 🚀 Faster |
| 137 | `COSH` | Vector Cosh | Math Transform | 17.9 | 17.8 | 0.99x | 🚀 Faster |
| 138 | `EXP` | Vector Arithmetic Exp | Math Transform | 15.6 | 16.8 | 1.08x | ✅ ≥67% |
| 139 | `FLOOR` | Vector Floor | Math Transform | 1.3 | 1.4 | 1.09x | ✅ ≥67% |
| 140 | `LN` | Vector Log Natural | Math Transform | 18.0 | 17.9 | 0.99x | 🚀 Faster |
| 141 | `LOG10` | Vector Log10 | Math Transform | 19.2 | 19.1 | 0.99x | 🚀 Faster |
| 142 | `SIN` | Vector Trigonometric Sin | Math Transform | 26.4 | 26.3 | 1.00x | 🚀 Faster |
| 143 | `SINH` | Vector Sinh | Math Transform | 17.9 | 18.0 | 1.01x | ✅ ≥67% |
| 144 | `SQRT` | Vector Square Root | Math Transform | 3.0 | 2.8 | 0.93x | 🚀 Faster |
| 145 | `TAN` | Vector Trigonometric Tan | Math Transform | 27.2 | 26.8 | 0.98x | 🚀 Faster |
| 146 | `TANH` | Vector Tanh | Math Transform | 13.8 | 12.9 | 0.93x | 🚀 Faster |
| 147 | `ADD` | Vector Arithmetic Add | Math Operators | 2.5 | 2.1 | 0.81x | 🚀 Faster |
| 148 | `DIV` | Vector Arithmetic Div | Math Operators | 2.5 | 2.1 | 0.82x | 🚀 Faster |
| 149 | `MAX` | Highest value over a specified period | Math Operators | 30.6 | 26.8 | 0.88x | 🚀 Faster |
| 150 | `MAXINDEX` | Index of highest value over a specified period | Math Operators | 37.1 | 28.4 | 0.77x | 🚀 Faster |
| 151 | `MIN` | Lowest value over a specified period | Math Operators | 34.8 | 26.6 | 0.76x | 🚀 Faster |
| 152 | `MININDEX` | Index of lowest value over a specified period | Math Operators | 39.1 | 26.9 | 0.69x | 🚀 Faster |
| 153 | `MINMAX` | Lowest and highest values over a specified period | Math Operators | 59.4 | 95.2 | 1.60x | ✅ ≥50% |
| 154 | `MINMAXINDEX` | Indexes of lowest and highest values over a specified period | Math Operators | 69.6 | 95.1 | 1.37x | ✅ ≥67% |
| 155 | `MULT` | Vector Arithmetic Mult | Math Operators | 2.7 | 2.0 | 0.74x | 🚀 Faster |
| 156 | `SUB` | Vector Arithmetic Subtraction | Math Operators | 2.4 | 2.0 | 0.82x | 🚀 Faster |
| 157 | `SUM` | Summation | Math Operators | 11.4 | 7.1 | 0.62x | 🚀 Faster |
## Documentation

- [Indicator Reference (EN)](docs/INDICATORS.md)
- [Indicator Reference (ZH)](docs/INDICATORS.zh.md)
- [AI Usage Guide (EN)](docs/AI_GUIDE.md)
- [Custom Indicator Guide (EN)](docs/CUSTOM_INDICATOR.md)
- [GitHub Wiki](https://github.com/map-A/ta-lib-rs/wiki)

## Project Structure

```
ta-lib-rs/
├── crates/
│   ├── polars-ta-core/     # Core Rust implementation (no Polars dependency)
│   ├── polars-ta/          # Polars Series/DataFrame wrappers
│   ├── polars-ta-plugin/   # PyO3/Polars Python plugin
│   └── polars-ta-verify/   # Golden tests and benchmarks
├── scripts/                # Utility scripts
├── docs/                   # Documentation
└── golden/                 # Golden test cases (JSON)
```

## Contributing

See [docs/CUSTOM_INDICATOR.md](docs/CUSTOM_INDICATOR.md) for how to add new indicators.
