# AI 辅助开发指南 — polars-ta

> **本文档面向 AI 助手**。如果你是 LLM 并需要为 polars-ta 添加新指标，请从头阅读本文档。

---

## 1. 项目结构速查

```
polars-ta/
├── crates/
│   ├── polars-ta-core/src/
│   │   ├── lib.rs                  ← 模块声明
│   │   ├── trend/
│   │   │   ├── mod.rs              ← pub mod + pub use
│   │   │   └── sma.rs              ← 参考实现（你应该模仿它）
│   │   ├── oscillator/mod.rs
│   │   ├── volume/mod.rs
│   │   └── volatility/mod.rs
│   ├── polars-ta/src/
│   │   └── trend/sma.rs            ← Polars Series 封装参考
│   └── polars-ta-verify/
│       ├── src/golden/mod.rs       ← assert_close / load_golden_file
│       └── tests/golden_sma.rs     ← Golden Test 参考
├── tests/golden/                   ← *.json 由 scripts/generate_golden.py 生成
└── scripts/generate_golden.py      ← 添加新指标的 generate_xxx() 函数
```

---

## 2. 如何添加一个新指标（完整流程）

### 步骤 1：实现核心算法（`polars-ta-core`）

**文件路径**：`crates/polars-ta-core/src/{category}/{name}.rs`

- `category` 为 `trend` / `oscillator` / `volume` / `volatility`

**函数签名规范**：

```rust
// 单输入单输出（如 SMA、RSI）
pub fn {name}(data: &[f64], period: usize) -> Vec<f64>

// 多列输入（如 ATR）
pub fn {name}(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64>

// 多输出（如 MACD）
pub struct MacdOutput {
    pub macd:   Vec<f64>,
    pub signal: Vec<f64>,
    pub hist:   Vec<f64>,
}
pub fn macd(data: &[f64], fast: usize, slow: usize, signal: usize) -> MacdOutput
```

**必须遵守的规则**：

1. **不能 panic**：`if data.len() < min_required { return vec![]; }` 或返回空结构体
2. **输出长度** = `input.len() - lookback`（lookback 见 ta-lib 文档）
3. **NaN 传播**：输入 NaN → 对应输出窗口 NaN
4. **零外部依赖**：只用 `std`，不引入任何 crate
5. **内联文档注释**：必须包含参数说明、输出长度公式、代码示例

**参考实现**：`crates/polars-ta-core/src/trend/sma.rs`（可直接作为模板）

```rust
// 最小模板
/// {indicator_name} — {description}
///
/// # Parameters
/// - `data`   — input price series
/// - `period` — window length
///
/// # Output
/// Length = `data.len() - {lookback_expr}`
/// Returns empty `Vec` when `data.len() < {min_input_len}`.
pub fn {name}(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    let lookback = /* TODO: formula */;
    if period == 0 || n <= lookback {
        return vec![];
    }
    let mut out = Vec::with_capacity(n - lookback);
    // TODO: algorithm
    out
}
```

### 步骤 2：在 mod.rs 中注册

```rust
// crates/polars-ta-core/src/{category}/mod.rs
pub mod {name};
pub use {name}::{name};
```

### 步骤 3：实现 Polars Series 封装（`polars-ta`）

**文件路径**：`crates/polars-ta/src/{category}/{name}.rs`

```rust
// 参考 crates/polars-ta/src/trend/sma.rs
use polars_core::prelude::*;
use polars_ta_core::{category}::{name} as {name}_core;

pub fn {name}_series(series: &Series, period: usize) -> PolarsResult<Series> {
    let ca = series.cast(&DataType::Float64)?;
    let ca = ca.f64()?;
    let data: Vec<f64> = ca.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect();
    let result = {name}_core(&data, period);
    Ok(Series::new(series.name().clone(), result))
}
```

### 步骤 4：生成 Golden 文件

在 `scripts/generate_golden.py` 中添加生成函数：

```python
def generate_{name}(output_dir: Path, period: int = 20):
    """Generate {NAME} golden files."""
    print(f"\n[{NAME}] period={period}")
    lookback = period - 1  # TODO: 替换为正确公式
    for dataset_name, make_fn in DATASETS.items():
        data = make_fn()
        close = data["close"]
        result = talib.{TALIB_NAME}(close, timeperiod=period)
        _write_golden(output_dir, "{name}", {"period": period},
                      {"close": close}, result, lookback, dataset_name)
    # boundary_exact 和 boundary_short
    bdata = make_boundary_exact(lookback)
    result = talib.{TALIB_NAME}(bdata["close"], timeperiod=period)
    _write_golden(output_dir, "{name}", {"period": period},
                  {"close": bdata["close"]}, result, lookback, "boundary_exact")
    sdata = make_boundary_short(lookback)
    result = talib.{TALIB_NAME}(sdata["close"], timeperiod=period)
    _write_golden(output_dir, "{name}", {"period": period},
                  {"close": sdata["close"]}, result, lookback, "boundary_short")

# 注册到 GENERATORS 字典
GENERATORS["{name}"] = lambda out_dir: generate_{name}(out_dir)
```

然后运行：

```bash
python scripts/generate_golden.py --indicator {name}
```

### 步骤 5：编写 Golden Test

**文件路径**：`crates/polars-ta-verify/tests/golden_{name}.rs`

```rust
use polars_ta_core::{category}::{name};
use polars_ta_verify::golden::{assert_close, load_golden_file};
use std::path::PathBuf;

fn golden_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests").join("golden").join(filename)
}

fn run_{name}_golden(filename: &str, period: usize, epsilon: f64) {
    let path = golden_path(filename);
    if !path.exists() {
        println!("SKIP: {filename}");
        return;
    }
    let golden = load_golden_file(&path).unwrap();
    let input = golden.close_input().unwrap();
    let actual = {name}(&input, period);
    assert_close(&actual, &golden.output.values, epsilon, &format!("{name}/period={period}"));
}

#[test] fn {name}_normal_1000()     { run_{name}_golden("{name}_period20_normal_1000.json", 20, 1e-10); }
#[test] fn {name}_boundary_exact()  { run_{name}_golden("{name}_period20_boundary_exact.json", 20, 1e-10); }
#[test] fn {name}_boundary_short()  { /* verify empty output */ }
#[test] fn {name}_with_nan()        { run_{name}_golden("{name}_period20_with_nan_5pct.json", 20, 1e-10); }
#[test] fn {name}_all_same_value()  { run_{name}_golden("{name}_period20_all_same_value.json", 20, 1e-10); }
#[test] fn {name}_real_btcusdt()    { run_{name}_golden("{name}_period20_real_btcusdt_1d.json", 20, 1e-10); }
#[test] fn {name}_real_flat()       { run_{name}_golden("{name}_period20_real_flat_period.json", 20, 1e-10); }
```

### 步骤 6：在 run-golden binary 中注册

在 `crates/polars-ta-verify/src/bin/run_golden.rs` 的 match 分支中添加：

```rust
"{name}" => {
    let period = golden.meta.params["period"].as_u64().unwrap_or(20) as usize;
    let input = match golden.close_input() { Ok(v) => v, Err(e) => { /* skip */ continue; } };
    let actual = {name}(&input, period);
    check_close(&actual, &golden.output.values, 1e-10, &filename)
}
```

### 步骤 7：添加 Criterion 基准

在 `crates/polars-ta-verify/benches/bench_vs_talib.rs` 中添加：

```rust
fn bench_{name}(c: &mut Criterion) {
    let sizes = [100usize, 10_000, 1_000_000];
    let period = 20;
    let mut group = c.benchmark_group("{name}");
    for &size in &sizes {
        let data = make_data(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("polars_ta", size), &size,
            |b, _| b.iter(|| {name}(black_box(&data), black_box(period))));
    }
    group.finish();
}
// 注册到 criterion_group!
```

### 步骤 8：更新 TODO.md

将对应行的状态从 `🔄` 改为 `✅`（所有 7 个 dataset Golden Test 通过 + 性能 ≥ 80%）。

---

## 3. 精度规范

| 等级 | epsilon | 使用场景 |
|------|---------|----------|
| 严格（默认） | `1e-10` | 大多数指标 |
| 宽松 | `1e-7` | 有浮点累积误差的指标（必须在代码注释中说明原因） |
| 失败 | `> 1e-7` | 必须修复，不可合并 |

---

## 4. Golden Test 失败修复流程

```
1. 找到失败的 index 和偏差量（assert_close 会输出）
2. 对比 ta-lib 源码：https://github.com/TA-Lib/ta-lib/tree/master/src/ta_func
3. 检查以下常见原因：
   a. EMA 初始化方式（ta-lib 用 SMA 做种子，不是第一个值）
   b. Wilder 平滑（RSI/ATR：alpha = 1/period，不是 2/(period+1)）
   c. lookback 计算错误（输出长度不对）
   d. 多列指标的对齐问题
4. 修复后重新运行：cargo test --package polars-ta-verify
5. 如仍失败 ≥ 3 次，放宽至 1e-7 并在代码注释写明根因
```

---

## 5. 指标实现速查表

### 5a. 已实现指标及基准状态（Phase 1）

所有指标均已实现 Criterion 基准（3 档规模：100 / 10,000 / 1,000,000）及 ta-lib Python 对比基准。

| 指标 | 分类 | Criterion 基准 | Python 基准 | run_golden |
|------|------|:--------------:|:-----------:|:----------:|
| SMA  | trend | ✅ | ✅ | ✅ |
| EMA  | trend | ✅ | ✅ | ✅ |
| WMA  | trend | ✅ | ✅ | ✅ |
| DEMA | trend | ✅ | ✅ | ✅ |
| TEMA | trend | ✅ | ✅ | ✅ |
| MACD | trend | ✅ | ✅ | ✅ |
| BBands | trend | ✅ | ✅ | ✅ |
| SAR  | trend | ✅ | ✅ | ✅ |
| ADX  | trend | ✅ | ✅ | ✅ |
| RSI  | oscillator | ✅ | ✅ | ✅ |
| Stoch | oscillator | ✅ | ✅ | ✅ |
| StochRSI | oscillator | ✅ | ✅ | ✅ |
| CCI  | oscillator | ✅ | ✅ | ✅ |
| WillR | oscillator | ✅ | ✅ | ✅ |
| ULTOSC | oscillator | ✅ | ✅ | ✅ |
| Aroon | oscillator | — | — | ✅ |
| MFI  | oscillator | — | — | ✅ |
| OBV  | volume | ✅ | ✅ | ✅ |
| AD   | volume | ✅ | ✅ | ✅ |
| ADOSC | volume | ✅ | ✅ | ✅ |
| TRange | volatility | ✅ | ✅ | ✅ |
| ATR  | volatility | ✅ | ✅ | ✅ |
| NATR | volatility | ✅ | ✅ | ✅ |

> Aroon と MFI の Criterion / Python ベンチマークは次フェーズで追加予定。

### 5b. 実装注意事項

| 指标 | 关键注意事项 |
|------|-------------|
| EMA | 初始值 = SMA(data[0..period])，alpha = 2/(period+1) |
| RSI | 使用 Wilder 平滑（alpha = 1/period），lookback = period |
| ATR | Wilder 平滑，lookback = period，True Range = max(H-L, |H-PC|, |L-PC|) |
| MACD | macd = EMA(fast) - EMA(slow)，signal = EMA(macd, signal_period) |
| BBands | middle = SMA，upper = middle + k*std，lower = middle - k*std（ta-lib 用总体标准差） |
| SAR | 状态机实现，参见 ta-lib ta_SAR.c |
| ADX | 基于 +DI/-DI/DX，Wilder 平滑，lookback = 2*period - 1 |

---

## 6. 性能检查清单

- [ ] 使用滑动窗口而非每次重算（避免 O(n²)）
- [ ] `Vec::with_capacity` 预分配
- [ ] 避免不必要的 `.clone()`
- [ ] 热路径不分配 `String` 或 `Box`
- [ ] 目标：throughput ≥ ta-lib C 的 95%（最低门控 80%）

---

## 7. 向 `scripts/generate_golden.py` 添加多列指标

对于需要 high/low/close 的指标（如 ATR、CCI）：

```python
def generate_atr(output_dir: Path, period: int = 14):
    lookback = period
    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.ATR(data["high"], data["low"], data["close"], timeperiod=period)
        _write_golden(output_dir, "atr", {"period": period},
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      result, lookback, name)
```

Golden 文件的 `input` 字段会包含多列，Golden Test 侧用 `golden.get_input("high")` 读取。
