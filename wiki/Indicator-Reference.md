# Indicator Reference

Complete reference for all 91+ implemented indicators.

> **Status**: Phase 1–3 complete. CDL patterns and HT (Hilbert Transform) indicators are planned.

## Trend (24)

| Function | Name | Lookback | Inputs |
|----------|------|----------|--------|
| `sma` | Simple Moving Average | period-1 | close |
| `ema` | Exponential Moving Average | period-1 | close |
| `wma` | Weighted Moving Average | period-1 | close |
| `dema` | Double EMA | 2*(period-1) | close |
| `tema` | Triple EMA | 3*(period-1) | close |
| `kama` | Kaufman Adaptive MA | period-1 | close |
| `trima` | Triangular MA | period-1 | close |
| `t3` | Triple Exponential MA (T3) | 6*(period-1) | close |
| `ma` | Moving Average (type-adaptive) | varies | close |
| `macd` | MACD | slow_period + signal_period - 2 | close |
| `macdext` | MACD with Controllable MA Type | varies | close |
| `macdfix` | MACD Fixed 12/26 | 33 | close |
| `bbands` | Bollinger Bands | period-1 | close |
| `midpoint` | Midpoint over Period | period-1 | close |
| `midprice` | Midpoint Price over Period | period-1 | high, low |
| `sar` | Parabolic SAR | 1 | high, low |
| `sarext` | Parabolic SAR Extended | 1 | high, low |
| `adx` | Average Directional Index | 2*(period-1) | high, low, close |
| `adxr` | ADX Rating | 3*(period-1) | high, low, close |
| `minus_di` | Minus Directional Indicator | period | high, low, close |
| `plus_di` | Plus Directional Indicator | period | high, low, close |
| `minus_dm` | Minus Directional Movement | period-1 | high, low |
| `plus_dm` | Plus Directional Movement | period-1 | high, low |
| `dx` | Directional Movement Index | 2*(period-1) | high, low, close |

## Oscillators (22)

| Function | Name | Lookback | Inputs |
|----------|------|----------|--------|
| `rsi` | Relative Strength Index | period | close |
| `stoch` | Stochastic | fastk+slowk+slowd-3 | high, low, close |
| `stochf` | Stochastic Fast | fastk-1 | high, low, close |
| `stochrsi` | Stochastic RSI | period+rsi_period | close |
| `cci` | Commodity Channel Index | period-1 | high, low, close |
| `willr` | Williams %R | period-1 | high, low, close |
| `ultosc` | Ultimate Oscillator | max(p1,p2,p3)-1 | high, low, close |
| `aroon` | Aroon | period | high, low |
| `aroonosc` | Aroon Oscillator | period | high, low |
| `mfi` | Money Flow Index | period | high, low, close, vol |
| `mom` | Momentum | period | close |
| `roc` | Rate of Change | period | close |
| `rocp` | Rate of Change Percentage | period | close |
| `rocr` | Rate of Change Ratio | period | close |
| `rocr100` | Rate of Change Ratio ×100 | period | close |
| `cmo` | Chande Momentum Oscillator | period | close |
| `apo` | Absolute Price Oscillator | slow-1 | close |
| `ppo` | Percentage Price Oscillator | slow-1 | close |
| `trix` | 1-day ROC of Triple-Smooth EMA | 3*(period-1)+1 | close |
| `bop` | Balance of Power | 0 | open, high, low, close |
| `minus_di` | Minus Directional Indicator | period | high, low, close |
| `plus_di` | Plus Directional Indicator | period | high, low, close |

## Volume (3)

| Function | Name | Lookback | Inputs |
|----------|------|----------|--------|
| `obv` | On Balance Volume | 0 | close, volume |
| `ad` | Chaikin A/D Line | 0 | high, low, close, vol |
| `adosc` | Chaikin A/D Oscillator | slow-1 | high, low, close, vol |

## Volatility (4)

| Function | Name | Lookback | Inputs |
|----------|------|----------|--------|
| `trange` | True Range | 1 | high, low, close |
| `atr` | Average True Range | period | high, low, close |
| `natr` | Normalized ATR | period | high, low, close |
| `beta` | Beta | period-1 | real0, real1 |

## Statistics (9)

| Function | Name | Lookback | Inputs |
|----------|------|----------|--------|
| `correl` | Pearson's Correlation | period-1 | real0, real1 |
| `linearreg` | Linear Regression | period-1 | close |
| `linearreg_angle` | Linear Regression Angle | period-1 | close |
| `linearreg_intercept` | Linear Regression Intercept | period-1 | close |
| `linearreg_slope` | Linear Regression Slope | period-1 | close |
| `stddev` | Standard Deviation | period-1 | close |
| `tsf` | Time Series Forecast | period-1 | close |
| `var` | Variance | period-1 | close |

## Price Transform (4)

All have lookback=0, output length = input length.

| Function | Name | Formula |
|----------|------|---------|
| `avgprice` | Average Price | (O+H+L+C)/4 |
| `medprice` | Median Price | (H+L)/2 |
| `typprice` | Typical Price | (H+L+C)/3 |
| `wclprice` | Weighted Close | (H+L+2C)/4 |

## Math Transform (15)

All element-wise (lookback=0). NaN propagates via IEEE 754.

| Function | Name |
|----------|------|
| `acos` | Arc Cosine |
| `asin` | Arc Sine |
| `atan` | Arc Tangent |
| `ceil` | Ceiling |
| `cos` | Cosine |
| `cosh` | Hyperbolic Cosine |
| `exp` | Exponential (eˣ) |
| `floor` | Floor |
| `ln` | Natural Log |
| `log10` | Log Base-10 |
| `sin` | Sine |
| `sinh` | Hyperbolic Sine |
| `sqrt` | Square Root |
| `tan` | Tangent |
| `tanh` | Hyperbolic Tangent |

## Math Operators (11)

| Function | Name | Algorithm |
|----------|------|-----------|
| `add` | Addition | element-wise, O(n) |
| `div` | Division | element-wise, O(n) |
| `mult` | Multiplication | element-wise, O(n) |
| `sub` | Subtraction | element-wise, O(n) |
| `max` | Rolling Maximum | O(n) monotone deque |
| `min` | Rolling Minimum | O(n) monotone deque |
| `sum` | Rolling Sum | O(n) sliding window |
| `maxindex` | Index of Max | O(n) monotone deque |
| `minindex` | Index of Min | O(n) monotone deque |
| `minmax` | Min and Max | O(n) combined pass |
| `minmaxindex` | Indexes of Min/Max | O(n) combined pass |

## Planned (Not Yet Implemented)

| Category | Count | Notes |
|----------|-------|-------|
| CDL Patterns | 61 | Candlestick pattern recognition |
| HT Indicators | 6 | Hilbert Transform cycle analysis |
