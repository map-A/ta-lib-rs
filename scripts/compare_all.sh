#!/usr/bin/env bash
# scripts/compare_all.sh
# ======================
# One-click polars-ta vs ta-lib C performance comparison.
#
# Runs in ~1-2 minutes (uses quick-bench binary, not criterion).
# Outputs a side-by-side table: indicator, ta-lib µs, polars-ta µs, ratio, status.
#
# Requirements:
#   - Rust toolchain (cargo)
#   - Python 3 with ta-lib installed (.venv/bin/python3 or system python3)
#
# Usage:
#   ./scripts/compare_all.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$REPO_ROOT"

# ── Detect Python ──────────────────────────────────────────────────────────────
PYTHON="python3"
if [ -f "$REPO_ROOT/.venv/bin/python3" ]; then
  PYTHON="$REPO_ROOT/.venv/bin/python3"
fi

echo "══════════════════════════════════════════════════════════════════"
echo "  polars-ta vs ta-lib C  —  Performance Comparison"
echo "  $(date '+%Y-%m-%d %H:%M:%S')"
echo "══════════════════════════════════════════════════════════════════"
echo ""

# ── Step 1: Build quick-bench binary ──────────────────────────────────────────
echo "Step 1: Building polars-ta quick-bench binary..."
cargo build --release --package polars-ta-verify --bin quick-bench --quiet
RUST_BENCH="$REPO_ROOT/target/release/quick-bench"
echo "  ✓ Built: $RUST_BENCH"

# ── Step 2: Run Rust benchmark ─────────────────────────────────────────────────
echo ""
echo "Step 2: Running polars-ta benchmark (n=10,000)..."
RUST_RESULTS="$(mktemp /tmp/polars_ta_bench_XXXXXX.csv)"
"$RUST_BENCH" > "$RUST_RESULTS"
RUST_LINES=$(tail -n +2 "$RUST_RESULTS" | wc -l | tr -d ' ')
echo "  ✓ $RUST_LINES indicators benchmarked → $RUST_RESULTS"

# ── Step 3: Run ta-lib Python benchmark ───────────────────────────────────────
echo ""
echo "Step 3: Running ta-lib benchmark (n=10,000)..."
TALIB_RESULTS=""
if "$PYTHON" -c "import talib" 2>/dev/null; then
  TALIB_RESULTS="$(mktemp /tmp/talib_bench_XXXXXX.json)"
  "$PYTHON" scripts/bench_talib.py \
    --output json \
    --output-file "$TALIB_RESULTS" \
    2>/dev/null
  echo "  ✓ ta-lib results → $TALIB_RESULTS"
else
  echo "  ⚠ ta-lib not available (install: uv pip install ta-lib)"
  echo "  Showing polars-ta timings only."
fi

# ── Step 4: Print comparison table ────────────────────────────────────────────
echo ""
echo "══════════════════════════════════════════════════════════════════"
echo "  Results (µs = median latency, n=10,000 elements)"
echo "  Status: 🚀 Faster  ✅ Within 2×  ⚠️  Slower"
echo "══════════════════════════════════════════════════════════════════"
echo ""

"$PYTHON" - "$RUST_RESULTS" "${TALIB_RESULTS:-}" << 'PYEOF'
import sys, json, csv

rust_file = sys.argv[1]
talib_file = sys.argv[2] if len(sys.argv) > 2 and sys.argv[2] else None

rust = {}
for row in csv.DictReader(open(rust_file)):
    rust[row["indicator"]] = float(row["p50_us"])

talib = {}
if talib_file:
    data = json.load(open(talib_file))
    talib = {r["indicator"]: r["p50_us"] for r in data if r.get("size") == 10000}

# ta-lib.org/functions/ order
ORDER = [
    "bbands","dema","ema","ht_trendline","kama","ma","mama","mavp","midpoint","midprice",
    "sar","sarext","sma","t3","tema","trima","wma",
    "adx","adxr","apo","aroon","aroonosc","bop","cci","cmo","dx","macd","macdext","macdfix",
    "mfi","minus_di","minus_dm","mom","plus_di","plus_dm","ppo","roc","rocp","rocr","rocr100",
    "rsi","stoch","stochf","stochrsi","trix","ultosc","willr",
    "ad","adosc","obv",
    "atr","natr","trange",
    "avgprice","medprice","typprice","wclprice",
    "ht_dcperiod","ht_dcphase","ht_phasor","ht_sine","ht_trendmode",
    "cdl2crows","cdl3blackcrows","cdl3inside","cdl3linestrike","cdl3starsinsouth","cdl3whitesoldiers",
    "cdlabandonedbaby","cdladvanceblock","cdlbelthold","cdlbreakaway","cdlclosingmarubozu",
    "cdlconcealbabyswall","cdlcounterattack","cdldarkcloudcover","cdldoji","cdldojistar",
    "cdldragonflydoji","cdlengulfing","cdleveningdojistar","cdleveningstar","cdlgapsidesidewhite",
    "cdlgravestonedoji","cdlhammer","cdlhangingman","cdlharami","cdlharamicross","cdlhighwave",
    "cdlhikkake","cdlhikkakemod","cdlhomingpigeon","cdlidentical3crows","cdlinneck",
    "cdlinvertedhammer","cdlkicking","cdlkickingbylength","cdlladderbottom","cdllongleggeddoji",
    "cdllongline","cdlmarubozu","cdlmatchinglow","cdlmathold","cdlmorningdojistar","cdlmorningstar",
    "cdlonneck","cdlpiercing","cdlrickshawman","cdlrisefall3methods","cdlseparatinglines",
    "cdlshootingstar","cdlshortline","cdlspinningtop","cdlstalledpattern","cdlsticksandwich",
    "cdltakuri","cdltasukigap","cdlthrusting","cdltristar","cdlunique3river","cdlupsidegap2crows",
    "cdlxsidegap3methods",
    "beta","correl","linearreg","linearreg_angle","linearreg_intercept","linearreg_slope",
    "stddev","tsf","var",
    "acos","asin","atan","ceil","cos","cosh","exp","floor","ln","log10","sin","sinh","sqrt","tan","tanh",
    "add","div","max","maxindex","min","minindex","minmax","minmaxindex","mult","sub","sum",
]

fmt = "{:<22} {:>12} {:>15} {:>8}  {}"
print(fmt.format("INDICATOR", "ta-lib (µs)", "polars-ta (µs)", "Ratio", "Status"))
print("-" * 72)

faster = total_cmp = 0
for name in ORDER:
    r = rust.get(name)
    t = talib.get(name)
    r_str = f"{r:.2f}" if r is not None else "N/A"
    t_str = f"{t:.2f}" if t is not None else "N/A"
    if r is not None and t is not None:
        ratio = r / t
        total_cmp += 1
        if ratio <= 1.0:
            status = "🚀 Faster"
            faster += 1
        elif ratio <= 2.0:
            status = "✅ OK"
        else:
            status = "⚠️  Slower"
        ratio_str = f"{ratio:.2f}x"
    else:
        ratio_str = "N/A"
        status = "✅ Implemented" if r is not None else "❌"
    print(fmt.format(name.upper(), t_str, r_str, ratio_str, status))

print("-" * 72)
if total_cmp:
    print(f"\n  {faster}/{total_cmp} indicators faster than ta-lib ({faster*100//total_cmp}%)")
PYEOF

echo ""
echo "✅ Comparison complete."
rm -f "$RUST_RESULTS" "$TALIB_RESULTS"
