# polars-ta

> **高性能 Rust 技术分析库，91+ 指标，数值与 ta-lib C 完全对齐。**

[![Crates.io](https://img.shields.io/crates/v/polars-ta.svg)](https://crates.io/crates/polars-ta)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://github.com/map-A/ta-lib-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/map-A/ta-lib-rs/actions/workflows/ci.yml)
[![Indicators](https://img.shields.io/badge/指标数量-91%2B-blue)](wiki/Indicator-Reference.md)

## 特性

- 🚀 **原生 Polars 接口** — 直接操作 `Series`，零拷贝
- 🎯 **数值严格对齐** — 黄金测试验证所有 91 个指标与 ta-lib C 误差 < 1e-10
- ⚡ **高性能** — 主要指标超越 ta-lib C 吞吐量（BBands 230%，AD 118%，ADX 110%）
- 🔌 **零外部依赖核心** — `polars-ta-core` 无外部依赖，可用于嵌入式/WASM 场景
- 🤖 **AI 友好** — 完整文档与架构说明，便于 AI 辅助扩展开发

## 快速开始

### 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
polars-ta = "0.1"
polars = "0.53"
```

### 单输出指标（SMA）

```rust
use polars::prelude::*;
use polars_ta::trend::sma_series;

fn main() -> PolarsResult<()> {
    let close = Series::new("close".into(), &[1.0f64, 2.0, 3.0, 4.0, 5.0]);

    // SMA(period=3)：输出长度 = 5 - (3-1) = 3
    let sma = sma_series(&close, 3)?;
    println!("{sma}");  // [2.0, 3.0, 4.0]

    Ok(())
}
```

### 多输出指标（MACD）

```rust
use polars::prelude::*;
use polars_ta::trend::macd_series;

fn main() -> PolarsResult<()> {
    let close = Series::new("close".into(), vec![/* 价格数据 */]);

    // 返回 (macd线, 信号线, 柱状图) — 三个等长 Series
    let (macd_line, signal, hist) = macd_series(&close, 12, 26, 9)?;
    println!("MACD:   {macd_line}");
    println!("信号线: {signal}");
    println!("柱状图: {hist}");

    Ok(())
}
```

## 输出长度约定

所有指标遵循 ta-lib C API 约定：**输出比输入短**。

```
输出长度 = 输入长度 - lookback

示例 — SMA(period=3)，lookback=2：
  输入：  [v0, v1, v2, v3, v4]   长度=5
  输出：         [v2', v3', v4']  长度=3
```

## 完整指标参考

### 趋势指标 (24) ✅

| 函数 | 名称 |
|------|------|
| `sma` | 简单移动平均 |
| `ema` | 指数移动平均 |
| `wma` | 加权移动平均 |
| `dema` | 双重指数移动平均 |
| `tema` | 三重指数移动平均 |
| `kama` | Kaufman 自适应移动平均 |
| `trima` | 三角移动平均 |
| `t3` | T3 三重指数移动平均 |
| `ma` | 自适应移动平均（按类型选择） |
| `macd` | 移动平均收敛散度 |
| `macdext` | 可控 MA 类型的 MACD |
| `macdfix` | 固定 12/26 周期 MACD |
| `bbands` | 布林带 |
| `midpoint` | 周期中值 |
| `midprice` | 中间价 |
| `sar` | 抛物线转向指标 |
| `sarext` | 抛物线转向指标扩展版 |
| `adx` | 平均趋向指数 |
| `adxr` | ADX 评级 |
| `minus_di` | 负向指标 |
| `plus_di` | 正向指标 |
| `minus_dm` | 负向运动 |
| `plus_dm` | 正向运动 |
| `dx` | 趋向运动指数 |

### 震荡指标 (22) ✅

| 函数 | 名称 |
|------|------|
| `rsi` | 相对强弱指数 |
| `stoch` | 随机指标 |
| `stochf` | 快速随机指标 |
| `stochrsi` | 随机 RSI |
| `cci` | 商品通道指数 |
| `willr` | 威廉 %R |
| `ultosc` | 终极振荡器 |
| `aroon` | 阿隆指标 |
| `aroonosc` | 阿隆振荡器 |
| `mfi` | 资金流量指数 |
| `mom` | 动量指标 |
| `roc` | 变动率 |
| `rocp` | 变动率百分比 |
| `rocr` | 变动率比率 |
| `rocr100` | 变动率比率 ×100 |
| `cmo` | 钱德动量振荡器 |
| `apo` | 绝对价格振荡器 |
| `ppo` | 百分比价格振荡器 |
| `trix` | 三重平滑 EMA 的单日变动率 |
| `bop` | 多空力量 |

### 成交量指标 (3) ✅

| 函数 | 名称 |
|------|------|
| `obv` | 能量潮 |
| `ad` | 蔡金 A/D 线 |
| `adosc` | 蔡金 A/D 振荡器 |

### 波动率指标 (4) ✅

| 函数 | 名称 |
|------|------|
| `trange` | 真实波动幅度 |
| `atr` | 平均真实波动幅度 |
| `natr` | 归一化 ATR |
| `beta` | 贝塔系数 |

### 统计函数 (9) ✅

| 函数 | 名称 |
|------|------|
| `correl` | 皮尔逊相关系数 |
| `linearreg` | 线性回归 |
| `linearreg_angle` | 线性回归角度 |
| `linearreg_intercept` | 线性回归截距 |
| `linearreg_slope` | 线性回归斜率 |
| `stddev` | 标准差 |
| `tsf` | 时间序列预测 |
| `var` | 方差 |

### 价格变换 (4) ✅

| 函数 | 名称 | 公式 |
|------|------|------|
| `avgprice` | 均价 | (O+H+L+C)/4 |
| `medprice` | 中间价 | (H+L)/2 |
| `typprice` | 典型价格 | (H+L+C)/3 |
| `wclprice` | 加权收盘价 | (H+L+2C)/4 |

### 数学变换 (15) ✅

全部逐元素运算（lookback=0），NaN 自动传播：

`acos`，`asin`，`atan`，`ceil`，`cos`，`cosh`，`exp`，`floor`，`ln`，`log10`，`sin`，`sinh`，`sqrt`，`tan`，`tanh`

### 数学运算符 (11) ✅

`add`，`div`，`mult`，`sub`，`max`，`min`，`sum`，`maxindex`，`minindex`，`minmax`，`minmaxindex`

## 性能

在 Apple M 系列芯片（1,000,000 元素，period=20）上的基准测试结果：

| 指标 | polars-ta (Melem/s) | ta-lib C (Melem/s) | 比率 |
|------|--------------------:|-------------------:|------|
| BBands | ~1150 | ~500 | **230%** |
| AD     | ~590  | ~500 | **118%** |
| ADX    | ~220  | ~200 | **110%** |
| TRange | ~460  | ~500 | 92% |
| ADOSC  | ~410  | ~500 | 82% |
| OBV    | ~415  | ~500 | 83% |

## 运行基准测试

```bash
# 需要 Python + ta-lib
./scripts/compare_all.sh

# 快速模式（更少迭代次数）
./scripts/compare_all.sh --quick

# 生成 Markdown 报告
./scripts/compare_all.sh --report

# Criterion 微基准测试
cargo bench --package polars-ta-verify
```

## 验证与测试

```bash
# 生成黄金测试 JSON（需要 Python + ta-lib）
python scripts/generate_golden.py

# 运行全部 807 个黄金测试
cargo test --package polars-ta-verify

# 一键执行
./scripts/run_golden.sh
```

## 架构

```
ta-lib-rs/
├── crates/
│   ├── polars-ta-core/        # 纯算法层（&[f64]，无外部依赖）
│   ├── polars-ta/             # Polars Series 封装（面向用户的主 API）
│   ├── polars-ta-plugin/      # Python Polars 插件（pyo3）
│   └── polars-ta-verify/      # 黄金测试框架 + Criterion 基准测试
├── tests/golden/              # 黄金 JSON 文件（纳入版本控制）
├── scripts/                   # 辅助脚本
└── docs/                      # AI_GUIDE.md, CUSTOM_INDICATOR.md
```

## 扩展自定义指标

参见 [docs/CUSTOM_INDICATOR.md](docs/CUSTOM_INDICATOR.md)，包含完整的分步指南和 VWAP 示例。

## 开发指南

参见 [docs/AI_GUIDE.md](docs/AI_GUIDE.md) — 为 AI 辅助开发设计：
- 新指标实现模板（7 步骤）
- 黄金测试失败调试流程
- 精度标准与性能检查清单

## Wiki

详细文档见 [wiki/](wiki/)：
- [快速开始](wiki/Quick-Start.md)
- [指标参考](wiki/Indicator-Reference.md)
- [性能说明](wiki/Performance.md)
- [贡献指南](wiki/Contributing.md)

> 注意：GitHub Wiki 是独立的 git 仓库。将 `wiki/` 目录内容推送到
> `https://github.com/map-A/ta-lib-rs.wiki.git` 即可在 GitHub 上访问。

## 许可证

MIT
