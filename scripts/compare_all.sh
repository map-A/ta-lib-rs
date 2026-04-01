#!/usr/bin/env bash
# scripts/compare_all.sh
# ======================
# One-click comparison of polars-ta vs ta-lib C performance.
#
# Outputs a side-by-side performance report showing:
#   - Throughput (elements/second) for each indicator and data size
#   - Speedup ratio (polars-ta / ta-lib C)
#   - Pass/Fail status against 80% and 100% thresholds
#
# Requirements:
#   - Rust toolchain (cargo)
#   - Python 3.11+ with ta-lib installed (in .venv or system)
#
# Usage:
#   ./scripts/compare_all.sh                       # full bench (~5-15 min)
#   ./scripts/compare_all.sh --quick               # quick mode (~1-2 min)
#   ./scripts/compare_all.sh --indicator=sma       # single indicator
#   ./scripts/compare_all.sh --output-dir=results/

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$REPO_ROOT"

OUTPUT_DIR="${RESULTS_DIR:-/tmp/polars-ta-bench}"
INDICATOR="all"
QUICK=0
SIZE=1000000
BENCH_EXTRA_ARGS=""

for arg in "$@"; do
  case "$arg" in
    --indicator=*) INDICATOR="${arg#--indicator=}" ;;
    --output-dir=*) OUTPUT_DIR="${arg#--output-dir=}" ;;
    --quick) QUICK=1 ;;
    --size=*) SIZE="${arg#--size=}" ;;
  esac
done

# In quick mode: fewer iterations, smaller measurement window
if [ "$QUICK" = "1" ]; then
  BENCH_EXTRA_ARGS="--measurement-time 2 --warm-up-time 1 --sample-size 10"
  echo "  [Quick mode: reduced iterations for faster results]"
fi

mkdir -p "$OUTPUT_DIR"

TALIB_RESULTS="$OUTPUT_DIR/talib_bench.json"

# ── Activate Python venv if present ───────────────────────────────────────────
PYTHON="python3"
if [ -f "$REPO_ROOT/.venv/bin/python3" ]; then
  PYTHON="$REPO_ROOT/.venv/bin/python3"
elif [ -f "$REPO_ROOT/.venv/bin/python" ]; then
  PYTHON="$REPO_ROOT/.venv/bin/python"
fi

echo "══════════════════════════════════════════════════════════════════"
echo "  polars-ta vs ta-lib C Performance Comparison"
echo "  $(date '+%Y-%m-%d %H:%M:%S')"
echo "══════════════════════════════════════════════════════════════════"
echo ""

# ── Step 1: Run Python ta-lib benchmark ────────────────────────────────────────
echo "Step 1: Running ta-lib C benchmark (Python)..."
if "$PYTHON" -c "import talib" 2>/dev/null; then
  "$PYTHON" scripts/bench_talib.py \
    --indicator "$INDICATOR" \
    --output json \
    --output-file "$TALIB_RESULTS"
  echo "  → Results saved to: $TALIB_RESULTS"
else
  echo "  WARNING: Python ta-lib not available."
  echo "  Install with: cd $REPO_ROOT && uv pip install ta-lib"
  echo "  Skipping ta-lib benchmark. Rust benchmarks will still run."
  TALIB_RESULTS=""
fi

echo ""

# ── Step 2: Run Rust criterion benchmark ──────────────────────────────────────
echo "Step 2: Running polars-ta Rust benchmark..."
# shellcheck disable=SC2086
cargo bench --package polars-ta-verify \
  --bench bench_vs_talib \
  -- --output-format bencher \
  $BENCH_EXTRA_ARGS \
  2>&1 | tee "$OUTPUT_DIR/rust_bench.txt"
echo "  → Raw output saved to: $OUTPUT_DIR/rust_bench.txt"

echo ""

# ── Step 3: Side-by-side comparison table ─────────────────────────────────────
echo "══════════════════════════════════════════════════════════════════"
echo "  Side-by-Side Performance Comparison (size=${SIZE})"
echo "══════════════════════════════════════════════════════════════════"
echo "  Target: polars-ta ≥ 100% of ta-lib C throughput"
echo "  Floor:  polars-ta ≥  80% of ta-lib C throughput"

TALIB_ARG=""
if [ -n "$TALIB_RESULTS" ] && [ -f "$TALIB_RESULTS" ]; then
  TALIB_ARG="--talib-json $TALIB_RESULTS"
fi

"$PYTHON" scripts/compare_bench.py \
  --rust-bench "$OUTPUT_DIR/rust_bench.txt" \
  $TALIB_ARG \
  --size "$SIZE"

echo ""
echo "  HTML criterion report: open target/criterion/report/index.html"
echo ""
echo "✅ Comparison complete. Full results in: $OUTPUT_DIR/"
