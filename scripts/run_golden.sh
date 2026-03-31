#!/usr/bin/env bash
# scripts/run_golden.sh
# =====================
# Generate golden test data from ta-lib, then run all Golden Tests.
#
# Usage:
#   ./scripts/run_golden.sh [--skip-generate]
#
# Steps:
#   1. Generate golden JSON files (requires Python + ta-lib)
#   2. Run Rust golden tests (cargo test)
#   3. Run the golden test runner binary (cargo run --bin run-golden)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$REPO_ROOT"

SKIP_GENERATE=false
for arg in "$@"; do
  case "$arg" in
    --skip-generate) SKIP_GENERATE=true ;;
  esac
done

# ── Step 1: Generate Golden Files ──────────────────────────────────────────────
if [ "$SKIP_GENERATE" = false ]; then
  echo "══════════════════════════════════════════"
  echo "  Step 1: Generating Golden Test Files"
  echo "══════════════════════════════════════════"
  if command -v python3 &> /dev/null; then
    python3 scripts/generate_golden.py --indicator all
  else
    echo "WARNING: python3 not found. Skipping golden file generation."
    echo "  Install Python 3.11+ and ta-lib to generate golden files."
  fi
else
  echo "Skipping golden file generation (--skip-generate)"
fi

echo ""
echo "══════════════════════════════════════════"
echo "  Step 2: Running Rust Golden Tests"
echo "══════════════════════════════════════════"
cargo test --package polars-ta-verify --test golden_sma 2>&1

echo ""
echo "══════════════════════════════════════════"
echo "  Step 3: Golden Test Runner Report"
echo "══════════════════════════════════════════"
cargo run --package polars-ta-verify --bin run-golden 2>&1

echo ""
echo "✅ Golden tests complete."
