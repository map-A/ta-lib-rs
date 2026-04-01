# polars-ta — Product Requirements Document

**版本**：v0.1 · **状态**：Draft · **日期**：2025

---

## 目录

1. [项目概述](#1-项目概述)
2. [功能范围](#2-功能范围)
3. [技术架构](#3-技术架构)
4. [验证框架：Golden Test System](#4-验证框架golden-test-system)
5. [性能基准框架](#5-性能基准框架)
6. [开发路线图](#6-开发路线图)
7. [风险登记](#7-风险登记)
8. [AI 辅助开发规范](#8-ai-辅助开发规范)
9. [附录](#9-附录)

---

## 1. 项目概述

### 1.1 背景与动机

ta-lib 是量化交易领域使用最广泛的技术指标 C 库，提供超过 158 个标准金融技术指标。然而该项目自 2007 年后实质性维护停滞，核心算法已超过 15 年未经重大更新。

现有 Rust 生态替代方案（`ta-rs`、`traquer` 等）同样面临维护停滞，且均未以 Polars 生态为一等公民设计，导致 Rust 量化开发者面临生态断层。

> **项目定位**：polars-ta 是一个以 Polars Series 为原生接口、数值精度与 ta-lib 完全对齐的 Rust 技术指标库。目标是成为 Rust 量化生态的标准指标库，填补 ta-lib 停止维护留下的空白。

### 1.2 目标用户

| 用户类型 | 核心痛点 | polars-ta 解法 |
|---|---|---|
| Rust 量化策略开发者 | 指标库与 Polars DataFrame 不兼容，需手动转换 | 原生 Polars Series 接口，零拷贝 |
| 从 Python 迁移到 Rust 的团队 | ta-lib Python binding 行为不一致，难以复现结果 | 数值与 ta-lib 完全对齐，Golden Test 保证 |
| 高频/嵌入式场景开发者 | C 版本 FFI overhead 难以接受 | 纯 Rust 实现，zero-cost abstraction |
| 开源量化框架维护者 | 依赖 C 库导致编译和分发复杂 | 纯 Rust，无 C 依赖，跨平台编译 |

### 1.3 项目目标（定量）

| 目标 | 成功指标 | 时间节点 |
|---|---|---|
| Phase 1 发布 | 30 个核心指标，Golden Test 通过率 100% | Month 3 |
| Phase 2 发布 | 80 个指标，发布至 crates.io | Month 6 |
| Phase 3 发布 | 158 个指标，与 ta-lib 完整对齐 | Month 12 |
| 数值精度 | 所有指标与 ta-lib 误差 < 1e-10（默认） | 持续 |
| 性能基准 | 关键指标吞吐量 ≥ ta-lib C 版本 90% | Month 6 |
| 社区采用 | crates.io 月下载量 > 1,000 | Month 9 |

---

## 2. 功能范围

### 2.1 Phase 1 核心指标（Month 1–3）

#### 趋势类（Trend）

| 指标 | 函数名 | 参数 | Lookback 期 | 输入列 |
|---|---|---|---|---|
| Simple Moving Average | `sma` | period | period - 1 | close |
| Exponential Moving Average | `ema` | period | period - 1 | close |
| Weighted Moving Average | `wma` | period | period - 1 | close |
| Double EMA | `dema` | period | 2*(period-1) | close |
| Triple EMA | `tema` | period | 3*(period-1) | close |
| MACD | `macd` | fast, slow, signal | slow + signal - 2 | close |
| Bollinger Bands | `bbands` | period, nbdevup, nbdevdn | period - 1 | close |
| Parabolic SAR | `sar` | acceleration, maximum | 1 | high, low |
| Average Directional Index | `adx` | period | 2*period - 1 | high, low, close |

#### 震荡类（Oscillator）

| 指标 | 函数名 | 参数 | Lookback 期 | 输入列 |
|---|---|---|---|---|
| RSI | `rsi` | period | period | close |
| Stochastic | `stoch` | fastk, slowk, slowd | fastk + slowk - 1 | high, low, close |
| Stochastic RSI | `stochrsi` | period, fastk, fastd | period + fastk - 1 | close |
| CCI | `cci` | period | period - 1 | high, low, close |
| Williams %R | `willr` | period | period - 1 | high, low, close |
| Ultimate Oscillator | `ultosc` | period1, period2, period3 | max(periods) - 1 | high, low, close |
| Aroon | `aroon` | period | period | high, low |
| MFI | `mfi` | period | period | high, low, close, volume |

#### 成交量类（Volume）

| 指标 | 函数名 | 参数 | Lookback 期 | 输入列 |
|---|---|---|---|---|
| On Balance Volume | `obv` | — | 0 | close, volume |
| Chaikin A/D Line | `ad` | — | 0 | high, low, close, volume |
| Chaikin A/D Oscillator | `adosc` | fast, slow | slow - 1 | high, low, close, volume |

#### 波动率类（Volatility）

| 指标 | 函数名 | 参数 | Lookback 期 | 输入列 |
|---|---|---|---|---|
| Average True Range | `atr` | period | period | high, low, close |
| Normalized ATR | `natr` | period | period | high, low, close |
| True Range | `trange` | — | 1 | high, low, close |

### 2.2 输出约定（已确认设计决策）

**对齐 ta-lib 输出形状：**

- 输出数组长度 = 输入长度 - lookback 期
- 前 lookback 期的值**不输出**（与 ta-lib C API 保持一致）
- 调用方负责处理输出与输入的 index 偏移对齐

```
输入:  [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9]  长度=10
SMA(period=3), lookback=2
输出:        [v2', v3', v4', v5', v6', v7', v8', v9']  长度=8
```

**多输出指标返回 StructArray：**

```rust
// MACD 返回三列
struct MacdOutput {
    macd:   Vec<f64>,  // 长度 = n - lookback
    signal: Vec<f64>,
    hist:   Vec<f64>,
}
```

**NaN 处理规则：**

- 输入含 NaN：NaN 传播至对应输出位置
- 输入长度 < lookback + 1：返回空 Series，不 panic
- 全 NaN 输入：返回全 NaN 输出（长度符合公式）

### 2.3 Out of Scope（明确不做）

- Python bindings（由社区按需扩展）
- 蜡烛形态识别（Pattern Recognition）—— Phase 1/2 不做
- 希尔伯特变换类函数（HT_DCPERIOD 等）
- 有状态流式计算接口（stateful streaming）—— 未来版本

---

## 3. 技术架构

### 3.1 整体分层

```
┌─────────────────────────────────────────────────┐
│           Python / Polars 用户                   │
├─────────────────────────────────────────────────┤
│  polars-ta-plugin  (pyo3 + cdylib)              │  ← Python import
├─────────────────────────────────────────────────┤
│  polars-ta         (Series 接口层)               │  ← Rust 用户主入口
├─────────────────────────────────────────────────┤
│  polars-ta-core    (纯算法层, &[f64], no_std)    │  ← 零依赖，可嵌入
├─────────────────────────────────────────────────┤
│  polars-ta-verify  (Golden Test + Bench 框架)    │  ← 开发/CI 专用
└─────────────────────────────────────────────────┘
```

### 3.2 Crate 职责

| Crate | 职责 | 对外暴露 | 依赖 |
|---|---|---|---|
| `polars-ta-core` | 纯算法，`&[f64]` 接口，no_std 兼容 | `pub fn` per indicator | 无外部依赖 |
| `polars-ta` | Polars Series 封装，主用户 API | Series 接口 | polars-core |
| `polars-ta-plugin` | pyo3 Polars 插件 | cdylib | polars-ta + pyo3 |
| `polars-ta-verify` | Golden Test + 性能基准 | binary | polars-ta-core + criterion |

### 3.3 接口设计示例（以 SMA 为例）

```rust
// ── Layer 1: 纯算法层（polars-ta-core）──────────────────────────────
// 无任何框架依赖，可用于嵌入式/WASM
pub fn sma(data: &[f64], period: usize) -> Vec<f64> {
    if data.len() < period { return vec![]; }
    // 输出长度 = data.len() - (period - 1)
    let lookback = period - 1;
    let out_len = data.len() - lookback;
    let mut out = Vec::with_capacity(out_len);
    let mut sum: f64 = data[..period].iter().sum();
    out.push(sum / period as f64);
    for i in period..data.len() {
        sum += data[i] - data[i - period];
        out.push(sum / period as f64);
    }
    out
}

// ── Layer 2: Polars Series 接口层（polars-ta）───────────────────────
pub fn sma_series(series: &Series, period: usize) -> PolarsResult<Series> {
    let ca = series.f64()?;
    let data: Vec<f64> = ca.into_iter()
        .map(|v| v.unwrap_or(f64::NAN))
        .collect();
    let result = sma(&data, period);
    Ok(Series::new(series.name().clone(), result))
}

// ── Layer 3: Polars 插件（polars-ta-plugin）─────────────────────────
#[polars_expr(output_type=Float64)]
fn pl_sma(inputs: &[Series], kwargs: SmaKwargs) -> PolarsResult<Series> {
    sma_series(&inputs[0], kwargs.period)
}
```

### 3.4 目录结构

```
polars-ta/
├── Cargo.toml                  # workspace
├── crates/
│   ├── polars-ta-core/
│   │   ├── src/
│   │   │   ├── trend/          # sma.rs, ema.rs, macd.rs ...
│   │   │   ├── oscillator/     # rsi.rs, cci.rs ...
│   │   │   ├── volume/         # obv.rs, ad.rs ...
│   │   │   └── volatility/     # atr.rs, trange.rs ...
│   │   └── Cargo.toml
│   ├── polars-ta/
│   │   └── src/lib.rs
│   ├── polars-ta-plugin/
│   │   └── src/lib.rs
│   └── polars-ta-verify/
│       ├── src/
│       │   ├── golden/         # Golden Test runner
│       │   └── bench/          # Benchmark runner
│       └── Cargo.toml
├── tests/
│   └── golden/                 # Golden JSON 文件（版本控制）
│       ├── sma_period20.json
│       ├── ema_period20.json
│       └── ...
└── benches/
    └── vs_talib/               # 对比基准数据
```

---

## 4. 验证框架：Golden Test System

### 4.1 设计原则

> **核心规则**：每个指标合并入主干之前，必须通过 100% 的 Golden Test。Golden 数据由 ta-lib C 库权威生成，一次生成，版本化管理，人工不可手动修改。

验证的目标是**数值等价**，而非算法相似。我们不关心实现路径，只关心在所有测试数据集上与 ta-lib 的输出是否在精度范围内一致。

### 4.2 测试数据集规范

| 数据集名称 | 描述 | 长度 | 用途 |
|---|---|---|---|
| `normal_1000` | 随机 OHLCV，seed=42，固定 | 1000 | 主验证集，覆盖正常市场 |
| `boundary_exact` | 长度 = lookback + 1（刚好产生 1 个输出值） | 动态 | 边界正确性 |
| `boundary_short` | 长度 = lookback（应返回空 Series） | 动态 | 边界错误处理 |
| `with_nan_5pct` | normal_1000 随机插入 5% NaN | 1000 | NaN 传播行为 |
| `all_same_value` | 全部为 100.0 | 1000 | 除零/退化场景 |
| `real_btcusdt_1d` | BTC/USDT 真实日线数据 | 1000 | 极端行情，真实分布 |
| `real_flat_period` | 长时间横盘数据（振幅 < 0.1%） | 500 | 低波动场景 |

### 4.3 精度等级

| 等级 | epsilon | 适用情形 | 操作 |
|---|---|---|---|
| 严格（默认） | `1e-10` | 大多数指标 | 直接通过 |
| 宽松 | `1e-7` | 有浮点累积差异的指标 | 需在代码注释中说明原因 |
| 失败 | `> 1e-7` | 任何情形 | 必须修复，不可合并 |

### 4.4 Golden 文件格式

```json
{
  "meta": {
    "indicator": "sma",
    "params": { "period": 20 },
    "talib_version": "0.4.28",
    "generated_at": "2025-01-01T00:00:00Z",
    "dataset": "normal_1000"
  },
  "input": {
    "close": [102.5, 103.1, 101.8, "..."]
  },
  "output": {
    "values": [null, null, "...", 102.34, 103.12, "..."]
  },
  "lookback": 19,
  "output_length": 981
}
```

> **注意**：JSON 中 `null` 表示 lookback 期（前 19 个），实际输出数组长度为 981。Rust 侧读取时跳过 null，只对比有效值。

### 4.5 Golden 文件生成脚本

```python
# scripts/generate_golden.py
# 运行环境：Python 3.11+，ta-lib 0.4.x
import talib
import numpy as np
import json
from pathlib import Path

TALIB_VERSION = talib.__version__
OUTPUT_DIR = Path("tests/golden")

def generate_sma(data_close: np.ndarray, period: int, dataset_name: str):
    result = talib.SMA(data_close, timeperiod=period)
    payload = {
        "meta": {
            "indicator": "sma",
            "params": {"period": period},
            "talib_version": TALIB_VERSION,
            "dataset": dataset_name,
        },
        "input": {"close": data_close.tolist()},
        "output": {"values": [None if np.isnan(v) else v for v in result.tolist()]},
        "lookback": period - 1,
        "output_length": int((~np.isnan(result)).sum()),
    }
    fname = OUTPUT_DIR / f"sma_period{period}_{dataset_name}.json"
    fname.write_text(json.dumps(payload, indent=2))
    print(f"Generated: {fname}")

if __name__ == "__main__":
    rng = np.random.default_rng(seed=42)
    normal_1000 = 100.0 + rng.normal(0, 2, 1000).cumsum()
    generate_sma(normal_1000, period=20, dataset_name="normal_1000")
    # ... 其他数据集和指标
```

### 4.6 Rust Golden Test 实现

```rust
// crates/polars-ta-verify/src/golden/mod.rs

use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct GoldenFile {
    meta: GoldenMeta,
    input: serde_json::Value,
    output: GoldenOutput,
    lookback: usize,
    output_length: usize,
}

#[derive(Deserialize)]
struct GoldenOutput {
    values: Vec<Option<f64>>,
}

/// 核心对比函数：输出失败的 index 和偏差量
pub fn assert_close(actual: &[f64], golden: &[Option<f64>], epsilon: f64, label: &str) {
    let valid_golden: Vec<(usize, f64)> = golden
        .iter()
        .enumerate()
        .filter_map(|(i, v)| v.map(|f| (i, f)))
        .collect();

    assert_eq!(
        actual.len(), valid_golden.len(),
        "[{}] 输出长度不一致: actual={}, expected={}",
        label, actual.len(), valid_golden.len()
    );

    let mut failures = vec![];
    for (actual_i, (golden_i, expected)) in actual.iter().zip(valid_golden.iter()) {
        let diff = (actual_i - expected).abs();
        if diff > epsilon {
            failures.push(format!(
                "  index={}: actual={:.15}, expected={:.15}, diff={:.2e}",
                golden_i, actual_i, expected, diff
            ));
        }
    }

    if !failures.is_empty() {
        panic!(
            "[{}] Golden Test 失败，共 {} 处超出 epsilon={:.0e}:\n{}",
            label,
            failures.len(),
            epsilon,
            failures.join("\n")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars_ta_core::trend::sma;

    #[test]
    fn test_sma_20_normal_1000() {
        let golden: GoldenFile = load_golden("tests/golden/sma_period20_normal_1000.json");
        let input: Vec<f64> = parse_close(&golden.input);
        let actual = sma(&input, 20);
        assert_close(&actual, &golden.output.values, 1e-10, "sma/normal_1000");
    }

    #[test]
    fn test_sma_20_boundary_exact() {
        let golden: GoldenFile = load_golden("tests/golden/sma_period20_boundary_exact.json");
        let input: Vec<f64> = parse_close(&golden.input);
        let actual = sma(&input, 20);
        assert_eq!(actual.len(), 1, "boundary_exact 应恰好产生 1 个输出值");
        assert_close(&actual, &golden.output.values, 1e-10, "sma/boundary_exact");
    }

    #[test]
    fn test_sma_20_boundary_short() {
        // 输入长度 < period，应返回空
        let input = vec![1.0_f64; 19]; // period=20, lookback=19
        let actual = sma(&input, 20);
        assert!(actual.is_empty(), "输入不足时应返回空 Vec");
    }
}
```

### 4.7 新指标合并门控（Definition of Done）

每个指标合并至 `main` 分支的**必要且充分条件**：

- [ ] 全部 7 个测试数据集 Golden Test 通过
- [ ] 精度满足对应等级（默认 `1e-10`，宽松须有注释说明）
- [ ] Lookback 期数值与 ta-lib 文档/源码一致
- [ ] 空输入、长度不足输入返回空 Series（不 panic，不 unwrap）
- [ ] 性能基准：吞吐量 ≥ ta-lib C 版本的 80%（见第 5 章）
- [ ] 函数注释包含：参数说明、输出长度公式、用法示例
- [ ] CI 流水线全绿（Golden Test + clippy + fmt）

---

## 5. 性能基准框架

### 5.1 设计目标

性能基准有两个独立目的：

1. **对外**：证明 polars-ta 在关键指标上的性能不逊于 ta-lib C 版本
2. **对内**：防止性能退化（每次 PR 自动运行，对比历史基线）

### 5.2 对比对象

| 对比目标 | 说明 | 基准标识 |
|---|---|---|
| ta-lib C（通过 Python binding 调用） | 权威基线，C 语言实现 | `talib_c` |
| polars-ta（本项目） | 主测对象 | `polars_ta` |
| ta-rs（现有 Rust 库，已停止维护） | 横向对比 | `ta_rs` |
| 手写朴素 Rust 实现 | 验证优化效果下限 | `naive_rust` |

### 5.3 测试场景

| 场景 | 数据量 | 说明 |
|---|---|---|
| Small | 100 条 | 典型回测单资产短窗口 |
| Medium | 10,000 条 | 典型日线全历史 |
| Large | 1,000,000 条 | Tick 数据/高频场景 |
| Batch | 1,000 资产 × 1,000 条 | DataFrame 并行场景 |

### 5.4 基准指标（Metrics）

每个场景记录以下指标：

| 指标 | 单位 | 说明 |
|---|---|---|
| Throughput | elements/sec | 主性能指标 |
| Latency p50/p99 | µs | 单次调用延迟分布 |
| Memory（peak） | KB | 峰值内存使用 |
| Speedup vs talib_c | 倍数（x） | 相对于 ta-lib C 的性能比 |

### 5.5 Criterion 基准实现

```rust
// benches/bench_sma.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use polars_ta_core::trend::sma;

fn bench_sma(c: &mut Criterion) {
    let sizes = [100usize, 10_000, 1_000_000];
    let period = 20;
    let mut group = c.benchmark_group("sma");

    for size in &sizes {
        let data: Vec<f64> = (0..*size).map(|i| 100.0 + i as f64 * 0.01).collect();
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::new("polars_ta", size),
            size,
            |b, _| b.iter(|| sma(black_box(&data), black_box(period))),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_sma);
criterion_main!(benches);
```

### 5.6 Python 侧 ta-lib 基准（对齐方法论）

```python
# scripts/bench_talib.py
# 使用相同数据集，确保对比公平
import talib
import numpy as np
import time

def bench_talib_sma(size: int, period: int = 20, repeat: int = 1000):
    data = np.arange(size, dtype=np.float64) * 0.01 + 100.0
    times = []
    for _ in range(repeat):
        t0 = time.perf_counter_ns()
        _ = talib.SMA(data, timeperiod=period)
        times.append(time.perf_counter_ns() - t0)
    p50 = np.percentile(times, 50) / 1000  # µs
    p99 = np.percentile(times, 99) / 1000
    throughput = size / (np.mean(times) / 1e9)
    print(f"talib_c | size={size:>9} | p50={p50:>8.2f}µs | p99={p99:>8.2f}µs | {throughput:>15,.0f} elem/s")

if __name__ == "__main__":
    for size in [100, 10_000, 1_000_000]:
        bench_talib_sma(size)
```

### 5.7 性能报告格式（每次 Release 输出）

```
polars-ta v0.2.0 Performance Report
Benchmark Date: 2025-xx-xx
Platform: Linux x86_64, Rust 1.75, AVX2 enabled

=== SMA (period=20) ===

| 数据量     | talib_c (elem/s)  | polars_ta (elem/s) | Speedup | p50 latency |
|-----------|------------------|--------------------|---------|-------------|
| 100       |       15,200,000 |         18,400,000 |  1.21x  |      5.4 µs |
| 10,000    |      142,000,000 |        168,000,000 |  1.18x  |     59.4 µs |
| 1,000,000 |      198,000,000 |        231,000,000 |  1.17x  |  4,329.0 µs |

=== EMA (period=20) ===
...

=== 综合结论 ===
Phase 1 指标中：
  - 性能超越 ta-lib C (≥1.0x)：xx/30 个指标
  - 性能达标 (≥0.9x)：xx/30 个指标
  - 需优化 (<0.9x)：xx/30 个指标 [列出]
```

### 5.8 性能门控规则

- **合并门控**：单指标性能 ≥ ta-lib C 的 80%，否则不合并
- **退化告警**：CI 检测到性能下降 > 10%，自动标记 PR 为警告
- **优化机会**：对超过 ta-lib C 20% 以上的指标，记录为"已优化"，在 Release Notes 中标注

---

## 6. 开发路线图

### 6.1 Phase 0：基础设施（Week 1–2）

**目标：在写任何正式指标之前，先让整条流水线跑通。**

| 任务 | 产出 | 验收标准 |
|---|---|---|
| Workspace 骨架 | 4 个 crate，Cargo.toml 配置完毕 | `cargo build` 通过 |
| Golden 生成脚本 | Python 脚本，生成 SMA 的 7 个数据集 JSON | 文件格式正确，`talib_version` 字段存在 |
| `assert_close` 函数 | Rust 函数，输出失败 index 和偏差量 | 单元测试覆盖 3 种情形 |
| Criterion 基准骨架 | bench_sma.rs 可运行 | `cargo bench` 输出基准结果 |
| Python ta-lib 基准脚本 | `bench_talib.py` 可输出对比数据 | 数据格式与 Rust 侧对齐 |
| CI 流水线 | GitHub Actions：Golden Test + bench + clippy | PR 自动触发，结果可见 |
| **SMA 完整闭环** | SMA 实现 + Golden Test 通过 + 基准数据 | 所有 7 个数据集 Golden Test 通过，性能 ≥ talib_c 80% |

> **Phase 0 是 Go/No-Go 决策点**：如果 SMA 完整闭环无法在 2 周内完成，全局时间估算需重新评估。

### 6.2 Phase 1：核心 30 指标（Week 3–12）

| 周次 | 指标 | 重点挑战 |
|---|---|---|
| Week 3–4 | SMA, EMA, WMA, DEMA, TEMA | 趋势均线组，验证 EMA 的初始化方式（SMA seed vs 第一个值） |
| Week 5–6 | RSI, CCI, Williams %R, MFI | 注意 RSI 的 Wilder 平滑与普通 EMA 的区别 |
| Week 7–8 | MACD, BBands | 首个多输出指标，StructArray 接口验证 |
| Week 9–10 | ATR, NATR, TRange, ADX | 首批需要 OHLC 多列输入的指标，接口扩展 |
| Week 11–12 | OBV, AD, ADOSC, SAR, Stoch, Aroon, ULTOSC | SAR 和 Stoch 状态复杂，预留更多调试时间 |

**Phase 1 完成标准：**
- 30 个指标全部 Golden Test 通过
- 性能报告发布（含与 ta-lib C 的对比表）
- crates.io 发布 `polars-ta v0.1.0`（alpha 标注）

### 6.3 Phase 2：扩展至 80 指标（Month 4–6）

- 完成剩余趋势类：KAMA, TRIMA, T3, MIDPOINT, MIDPRICE 等
- 完成剩余震荡类：CMO, DX, MINUS_DI/PLUS_DI, PPO, ROC 等
- SIMD 优化 Pass：对 SMA/EMA 等高频指标做 AVX2 加速
- 发布 `polars-ta v0.2.0`，性能报告更新

### 6.4 Phase 3：完整 158 指标（Month 7–12）

- 补全剩余指标（含 Pattern Recognition 子集）
- 性能全面优化，争取关键指标超越 ta-lib C
- 发布 `v1.0.0`，宣布 API 稳定
- 考虑发布 Python Polars Plugin 至 PyPI

---

## 7. 风险登记

| 风险 | 概率 | 影响 | 缓解策略 |
|---|---|---|---|
| 数值一致性无法在 1e-10 满足 | 中 | 高 | 先跑 Golden Test，发现问题分析浮点原因，必要时放宽至 1e-7 并注释 |
| AI 生成代码存在静默逻辑错误 | 高 | 高 | Golden Test 是唯一裁判，生成后必须跑测试，不依赖代码 review 发现数值错误 |
| 指标数量膨胀导致中途放弃 | 高 | 高 | Phase 门控严格执行，Phase 1 未完成不进入 Phase 2 |
| Polars API 破坏性升级 | 中 | 中 | 固定最低版本，用 feature flag 隔离版本差异 |
| 性能低于 ta-lib C 的 80% | 低 | 中 | 先用朴素实现通过 Golden Test，再做针对性优化 |
| 竞争项目率先上线 | 低 | 低 | 差异化在于：完整验证体系 + Polars 原生 + 活跃维护 |

---

## 8. AI 辅助开发规范

### 8.1 AI 的角色边界

| 任务类型 | AI 参与度 | 人工职责 |
|---|---|---|
| 算法代码生成 | 90% | 审查逻辑，确保与 ta-lib 源码意图一致 |
| 测试代码生成 | 70% | 设计边界条件，验证覆盖完整性 |
| API 设计 | 20% | 所有接口决策由人工主导 |
| Golden Test 差异分析 | 30% | epsilon 调整和失败根因由人工决策 |
| 文档编写 | 60% | 示例代码必须来自真实使用场景 |
| 性能优化 | 20% | 需要 profile 真实数据，AI 提供方向 |

### 8.2 标准化 Prompt 模板

**指标实现 Prompt：**

```
实现 Rust 函数 `{indicator_name}`，要求：
1. 函数签名：pub fn {name}(data: &[f64], period: usize) -> Vec<f64>
2. 输出长度 = data.len() - lookback，lookback = {lookback_formula}
3. 算法对齐 ta-lib C 源码 {source_file}.c 的实现（附上源码片段）
4. 不使用任何外部 crate，只允许 std
5. 不要 panic，空输入或长度不足时返回空 Vec
6. 附上 #[test]，使用 assert_close(&result, &expected, 1e-10)
```

**Golden Test 失败修复 Prompt：**

```
以下 Golden Test 失败，请修复：
- 指标：{name}，参数：{params}
- 失败位置：index={i}，actual={a}，expected={e}，diff={d}
- 当前实现：{code}
- ta-lib 源码对应片段：{talib_source}
请分析偏差原因并给出修复方案。
```

### 8.3 验证失败处理流程

```
Golden Test 失败
      ↓
输出失败的 index 和偏差量
      ↓
将失败信息 + ta-lib 源码片段 → AI 修复
      ↓
重跑 Golden Test
      ↓ 仍然失败（≥3次）
人工阅读 ta-lib 源码，手动推导正确实现
      ↓
修复后补充"已知差异注释"，记录根因
```

---

## 9. 附录

### 9.1 核心依赖版本

| 依赖 | 最低版本 | 说明 |
|---|---|---|
| polars | 0.53 | Series API 稳定基线 |
| ta-lib（Golden 生成） | 0.4.28 | Python binding，仅用于生成 Golden 文件 |
| Rust toolchain | 1.75 stable | 支持 SIMD 稳定特性 |
| pyo3 | 0.22 | Polars Plugin 依赖 |
| criterion | 0.5 | 基准测试框架 |
| serde_json | 1.0 | Golden 文件读写 |

### 9.2 关键假设记录（达利奥式）

在项目启动时显式写下，定期对照：

| 假设 | 如果错误，会出现的信号 | 检查节点 |
|---|---|---|
| AI 生成的指标代码验证通过率 > 70% | 前 5 个指标中有 3 个以上需要人工大幅修改 | Phase 0 结束时 |
| 验证框架可在 2 周内建立完成 | Week 2 结束时 SMA 闭环未跑通 | Week 2 |
| polars-ta 性能 ≥ ta-lib C 的 90% | SMA benchmark 首次运行低于 80% | Phase 0 结束时 |
| Rust 量化社区有足够需求 | 发布 3 个月后 crates.io 月下载 < 100 | Month 4 |
| ta-lib 输出在相同输入下是确定性的 | 两次生成的 Golden 文件出现差异 | Golden 生成时 |

### 9.3 参考资料

- ta-lib 源码：https://github.com/TA-Lib/ta-lib
- Polars Plugin 文档：https://docs.pola.rs/user-guide/expressions/plugins/
- ta-lib-sys（FFI wrapper 参考）：https://crates.io/crates/ta-lib-sys
- traquer（现有 Rust 指标库）：https://crates.io/crates/traquer
- Criterion 文档：https://bheisler.github.io/criterion.rs/book/

---

*polars-ta PRD v0.1*
