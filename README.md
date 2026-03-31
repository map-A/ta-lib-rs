# polars-ta

> **Rust 技术指标库，以 Polars Series 为原生接口，数值精度与 ta-lib 完全对齐。**

[![Crates.io](https://img.shields.io/crates/v/polars-ta.svg)](https://crates.io/crates/polars-ta)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## 特性

- 🚀 **原生 Polars 接口** — 直接操作 `Series`，零拷贝
- 🎯 **数值完全对齐** — 通过 Golden Test 保证与 ta-lib C 误差 < 1e-10
- ⚡ **高性能** — 关键指标吞吐量 ≥ ta-lib C 的 90%
- 🔌 **零依赖核心层** — `polars-ta-core` 无外部依赖，可用于嵌入式/WASM
- 🤖 **AI 友好** — 完整的文档体系，AI 可直接读取并扩展新指标

## 快速开始

```toml
# Cargo.toml
[dependencies]
polars-ta = "0.1"
polars = "0.46"
```

```rust
use polars::prelude::*;
use polars_ta::trend::sma_series;

fn main() -> PolarsResult<()> {
    let close = Series::new("close".into(), &[1.0f64, 2.0, 3.0, 4.0, 5.0]);
    
    // SMA(period=3): 输出长度 = 5 - (3-1) = 3
    let sma = sma_series(&close, 3)?;
    println!("{sma}");  // [2.0, 3.0, 4.0]
    
    Ok(())
}
```

## 已实现指标

### Phase 1（核心 30 个指标）

| 类别 | 指标 | 函数 | 状态 |
|------|------|------|------|
| 趋势 | Simple Moving Average | `sma` | ✅ |
| 趋势 | Exponential Moving Average | `ema` | 🔄 |
| 趋势 | Weighted Moving Average | `wma` | 🔄 |
| 趋势 | Double EMA | `dema` | 🔄 |
| 趋势 | Triple EMA | `tema` | 🔄 |
| 趋势 | MACD | `macd` | 🔄 |
| 趋势 | Bollinger Bands | `bbands` | 🔄 |
| 趋势 | Parabolic SAR | `sar` | 🔄 |
| 趋势 | ADX | `adx` | 🔄 |
| 震荡 | RSI | `rsi` | 🔄 |
| 震荡 | Stochastic | `stoch` | 🔄 |
| 震荡 | Stochastic RSI | `stochrsi` | 🔄 |
| 震荡 | CCI | `cci` | 🔄 |
| 震荡 | Williams %R | `willr` | 🔄 |
| 震荡 | Ultimate Oscillator | `ultosc` | 🔄 |
| 震荡 | Aroon | `aroon` | 🔄 |
| 震荡 | MFI | `mfi` | 🔄 |
| 成交量 | OBV | `obv` | 🔄 |
| 成交量 | Chaikin A/D | `ad` | 🔄 |
| 成交量 | Chaikin A/D Oscillator | `adosc` | 🔄 |
| 波动率 | ATR | `atr` | 🔄 |
| 波动率 | NATR | `natr` | 🔄 |
| 波动率 | True Range | `trange` | 🔄 |

✅ = 已完成（Golden Test 通过，性能达标）  
🔄 = 开发中

完整状态见 [TODO.md](TODO.md)。

## 输出约定

所有指标输出遵循 ta-lib C API 约定：

```text
输出长度 = 输入长度 - lookback

例（SMA period=3，lookback=2）：
输入:  [v0, v1, v2, v3, v4]  len=5
输出:        [v2', v3', v4']  len=3
```

**调用方负责处理 index 偏移对齐。**

## 架构

```
polars-ta/
├── crates/
│   ├── polars-ta-core/    # 纯算法层（&[f64]，零依赖，no_std）
│   ├── polars-ta/         # Polars Series 封装（主用户 API）
│   ├── polars-ta-plugin/  # Python Polars 插件（pyo3）
│   └── polars-ta-verify/  # Golden Test + 性能基准框架
├── tests/golden/          # Golden JSON 文件（版本控制）
├── scripts/               # 工具脚本
└── docs/                  # 文档
```

## 验证与基准

### 运行 Golden Tests

```bash
# 1. 生成 golden 文件（需要 Python + ta-lib）
python scripts/generate_golden.py

# 2. 运行所有 golden tests
cargo test --package polars-ta-verify

# 或一键运行
./scripts/run_golden.sh
```

### 与 ta-lib C 对比

```bash
# 一键运行性能对比（需要 Python + ta-lib）
./scripts/compare_all.sh
```

## 开发指南

参见 [docs/AI_GUIDE.md](docs/AI_GUIDE.md) — 专为 AI 辅助开发设计，包含：
- 新指标实现模板
- Golden Test 失败修复流程
- 标准化 Prompt 模板

## License

MIT
