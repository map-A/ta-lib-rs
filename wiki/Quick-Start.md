# Quick Start

## Installation

Add `polars-ta` to your `Cargo.toml`:

```toml
[dependencies]
polars-ta = "0.1"
polars = "0.46"
```

## Single-Output Indicator

```rust
use polars::prelude::*;
use polars_ta::trend::sma_series;

fn main() -> PolarsResult<()> {
    let close = Series::new("close".into(), &[1.0f64, 2.0, 3.0, 4.0, 5.0]);

    // SMA with period=3: output length = 5 - (3-1) = 3
    let sma = sma_series(&close, 3)?;
    println!("{sma}");  // shape: (3,) [2.0, 3.0, 4.0]

    Ok(())
}
```

## Multi-Output Indicator (MACD)

```rust
use polars::prelude::*;
use polars_ta::trend::macd_series;

fn main() -> PolarsResult<()> {
    let close = Series::new("close".into(), vec![
        44.34, 44.09, 44.15, 43.61, 44.33,
        44.83, 45.10, 45.15, 43.61, 44.33,
        44.83, 45.10, 45.15, 45.38, 46.00,
        44.83, 45.10, 45.15, 43.61, 44.33,
        44.83, 45.10, 45.15, 45.38, 46.00,
        44.83, 45.10, 45.15, 43.61, 44.33,
        44.83, 45.10, 45.15, 45.38, 46.00,
    ]);

    // Returns (macd_line, signal_line, histogram)
    let (macd_line, signal, hist) = macd_series(&close, 12, 26, 9)?;
    println!("MACD:   {macd_line}");
    println!("Signal: {signal}");
    println!("Hist:   {hist}");

    Ok(())
}
```

## Output Length Convention

All indicators follow ta-lib's output convention — output is **shorter** than input:

```
output_length = input_length - lookback

Example — EMA(period=14):
  lookback = 13
  input length = 100  →  output length = 87
```

You are responsible for index-aligning the output with your DataFrame. The `lookback` value is documented in each function's signature.

## DataFrame Integration

```rust
use polars::prelude::*;
use polars_ta::trend::ema_series;

fn add_ema_column(df: &DataFrame, period: usize) -> PolarsResult<DataFrame> {
    let close = df.column("close")?;
    let ema = ema_series(close, period)?;

    // Pad with leading NaNs to match original DataFrame length
    let lookback = period - 1;
    let nulls = Series::full_null("ema".into(), lookback, &DataType::Float64);
    let padded = concat_series(&[nulls, ema])?;

    df.clone().with_column(padded)
}
```

## Using the Core Layer Directly

For use without Polars (e.g., embedded systems or WASM), use `polars-ta-core` directly:

```toml
[dependencies]
polars-ta-core = "0.1"
```

```rust
use polars_ta_core::trend::sma;

let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let result = sma(&data, 3);
// result = [2.0, 3.0, 4.0]
```
