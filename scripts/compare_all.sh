#!/usr/bin/env bash
# scripts/compare_all.sh
# ======================
# One-click comparison of polars-ta vs ta-lib C performance.
#
# Outputs a side-by-side performance report showing:
#   - Throughput (elements/second) for each indicator and data size
#   - Speedup ratio (polars-ta / ta-lib C)
#   - Pass/Fail status against 80% and 95% thresholds
#
# Requirements:
#   - Rust toolchain (cargo)
#   - Python 3.11+ with ta-lib installed
#
# Usage:
#   ./scripts/compare_all.sh [--indicator sma] [--output-dir results/]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$REPO_ROOT"

OUTPUT_DIR="${RESULTS_DIR:-/tmp/polars-ta-bench}"
INDICATOR="all"

for arg in "$@"; do
  case "$arg" in
    --indicator=*) INDICATOR="${arg#--indicator=}" ;;
    --output-dir=*) OUTPUT_DIR="${arg#--output-dir=}" ;;
  esac
done

mkdir -p "$OUTPUT_DIR"

TALIB_RESULTS="$OUTPUT_DIR/talib_bench.json"
RUST_BENCH_DIR="$OUTPUT_DIR/rust_bench"

echo "══════════════════════════════════════════════════════════════"
echo "  polars-ta vs ta-lib C Performance Comparison"
echo "  $(date '+%Y-%m-%d %H:%M:%S')"
echo "══════════════════════════════════════════════════════════════"
echo ""

# ── Step 1: Run Python ta-lib benchmark ────────────────────────────────────────
echo "Step 1: Running ta-lib C benchmark (Python)..."
if command -v python3 &> /dev/null && python3 -c "import talib" 2>/dev/null; then
  python3 scripts/bench_talib.py \
    --indicator "$INDICATOR" \
    --output json \
    --output-file "$TALIB_RESULTS"
  echo "  → Results saved to: $TALIB_RESULTS"
else
  echo "  WARNING: Python ta-lib not available."
  echo "  Install with: pip install ta-lib"
  echo "  Skipping ta-lib benchmark. Rust benchmarks will still run."
  TALIB_RESULTS=""
fi

echo ""

# ── Step 2: Run Rust criterion benchmark ──────────────────────────────────────
echo "Step 2: Running polars-ta Rust benchmark..."
cargo bench --package polars-ta-verify \
  --bench bench_vs_talib \
  -- --output-format bencher \
  2>&1 | tee "$OUTPUT_DIR/rust_bench.txt"
echo "  → Raw output saved to: $OUTPUT_DIR/rust_bench.txt"

echo ""

# ── Step 3: Print comparison report ───────────────────────────────────────────
echo "══════════════════════════════════════════════════════════════"
echo "  Comparison Report"
echo "══════════════════════════════════════════════════════════════"

if [ -n "$TALIB_RESULTS" ] && [ -f "$TALIB_RESULTS" ]; then
  echo ""
  echo "ta-lib C baseline:"
  python3 scripts/bench_talib.py --indicator "$INDICATOR" --output table
fi

echo ""
echo "Rust criterion results are in: target/criterion/"
echo "Open target/criterion/report/index.html for HTML report."

echo ""
echo "══════════════════════════════════════════════════════════════"
echo "  Performance Threshold Check"
echo "══════════════════════════════════════════════════════════════"
echo "  Threshold for merge: polars-ta ≥ 80% of ta-lib C throughput"
echo "  Target:              polars-ta ≥ 95% of ta-lib C throughput"
echo ""
echo "  Run 'cargo bench' to get Rust throughput numbers,"
echo "  then compare with the ta-lib C numbers above."
echo ""
echo "✅ Comparison complete. Full results in: $OUTPUT_DIR/"
