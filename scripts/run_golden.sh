#!/usr/bin/env bash
# scripts/run_golden.sh
# =====================
# Verify polars-ta correctness against ta-lib reference values.
#
# Steps:
#   1. (Optional) Regenerate golden JSON files using ta-lib Python binding
#   2. Build and run the Rust golden test runner (all 688 test cases)
#
# Usage:
#   ./scripts/run_golden.sh                  # regenerate golden files, then test
#   ./scripts/run_golden.sh --skip-generate  # skip generation, run tests only
#
# Requirements:
#   - Rust toolchain (cargo)
#   - Python 3 with ta-lib (for Step 1 only)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$REPO_ROOT"

# ── Detect Python ──────────────────────────────────────────────────────────────
PYTHON="python3"
if [ -f "$REPO_ROOT/.venv/bin/python3" ]; then
  PYTHON="$REPO_ROOT/.venv/bin/python3"
fi

SKIP_GENERATE=false
for arg in "$@"; do
  case "$arg" in
    --skip-generate) SKIP_GENERATE=true ;;
  esac
done

echo "══════════════════════════════════════════"
echo "  polars-ta Golden Test Suite"
echo "  $(date '+%Y-%m-%d %H:%M:%S')"
echo "══════════════════════════════════════════"
echo ""

# ── Step 1: Generate Golden Files ──────────────────────────────────────────────
if [ "$SKIP_GENERATE" = false ]; then
  echo "Step 1: Generating golden test files from ta-lib..."
  if "$PYTHON" -c "import talib" 2>/dev/null; then
    "$PYTHON" scripts/generate_golden.py --indicator all
    echo "  ✓ Golden files written to tests/golden/"
  else
    echo "  ⚠ ta-lib not available — skipping generation."
    echo "    Install with: uv pip install ta-lib  (or: pip install ta-lib)"
    echo "    Using existing golden files in tests/golden/"
  fi
else
  echo "Step 1: Skipped (--skip-generate)"
fi

echo ""

# ── Step 2: Build and run golden test binary ───────────────────────────────────
echo "Step 2: Building run-golden binary..."
cargo build --release --package polars-ta-verify --bin run-golden --quiet
echo "  ✓ Built"

echo ""
echo "Step 3: Running all golden tests..."
echo ""

"$REPO_ROOT/target/release/run-golden"

echo ""
echo "✅ Golden tests complete."
