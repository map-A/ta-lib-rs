#!/usr/bin/env python3
"""
scripts/compare_bench.py
========================
Parse Criterion bencher output + ta-lib JSON results and produce a
unified side-by-side performance comparison table.

Usage (called by compare_all.sh):
    cargo bench ... --output-format bencher 2>&1 | \
        python scripts/compare_bench.py --talib-json /tmp/talib_bench.json

Or standalone:
    python scripts/compare_bench.py \
        --rust-bench /tmp/rust_bench.txt \
        --talib-json /tmp/talib_bench.json \
        --size 1000000
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Optional

# ─── ANSI colors ──────────────────────────────────────────────────────────────
GREEN  = "\033[92m"
YELLOW = "\033[93m"
RED    = "\033[91m"
BOLD   = "\033[1m"
RESET  = "\033[0m"

THRESHOLD_PASS   = 1.00   # ≥100%: green ✅
THRESHOLD_WARN   = 0.80   # ≥80%: yellow ⚠️
# <80%: red ❌

# ─── Parse Criterion bencher output ───────────────────────────────────────────
# Format: "test {group}/polars_ta/{size} ... bench:   1,234,567 ns/iter (+/- N)"
BENCHER_RE = re.compile(
    r"test\s+(\w+)/polars_ta/(\d+)\s+\.\.\.\s+bench:\s+([\d,]+)\s+ns/iter"
)

def parse_criterion_bencher(text: str) -> dict[tuple[str, int], float]:
    """Returns {(indicator, size): throughput_elem_per_sec}."""
    results: dict[tuple[str, int], float] = {}
    for line in text.splitlines():
        m = BENCHER_RE.search(line)
        if m:
            indicator = m.group(1)
            size = int(m.group(2))
            ns_per_iter = float(m.group(3).replace(",", ""))
            throughput = size / (ns_per_iter / 1e9)
            results[(indicator, size)] = throughput
    return results


def parse_criterion_json(criterion_dir: Path, size: int) -> dict[str, float]:
    """
    Read Criterion's estimates.json files from target/criterion/.
    Returns {indicator: throughput_elem_per_sec}.
    """
    results: dict[str, float] = {}
    for est_path in criterion_dir.glob("*/polars_ta/{size}/new/estimates.json".format(size=size)):
        indicator = est_path.parts[-5]  # target/criterion/{indicator}/polars_ta/{size}/...
        try:
            data = json.loads(est_path.read_text())
            mean_ns = data["mean"]["point_estimate"]
            results[indicator] = size / (mean_ns / 1e9)
        except Exception:
            pass
    return results


# ─── Load ta-lib JSON results ──────────────────────────────────────────────────
def load_talib_results(path: str) -> dict[tuple[str, int], float]:
    """Returns {(indicator, size): throughput_elem_per_sec}."""
    results: dict[tuple[str, int], float] = {}
    try:
        data = json.loads(Path(path).read_text())
        for r in data:
            ind = r.get("indicator", "")
            size = r.get("size", 0)
            tput = r.get("throughput_elem_per_sec", 0.0)
            if ind and size and tput:
                results[(ind, size)] = tput
    except Exception as e:
        print(f"  WARNING: Could not load ta-lib results from {path}: {e}", file=sys.stderr)
    return results


# ─── Format helpers ────────────────────────────────────────────────────────────
def fmt_throughput(t: float) -> str:
    if t >= 1e9:
        return f"{t/1e9:6.2f} G/s"
    elif t >= 1e6:
        return f"{t/1e6:6.1f} M/s"
    elif t >= 1e3:
        return f"{t/1e3:6.1f} K/s"
    return f"{t:6.1f} /s"


def status_str(ratio: Optional[float]) -> str:
    if ratio is None:
        return "  N/A  "
    if ratio >= THRESHOLD_PASS:
        return f"{GREEN}✅ {ratio*100:5.0f}%{RESET}"
    elif ratio >= THRESHOLD_WARN:
        return f"{YELLOW}⚠️  {ratio*100:5.0f}%{RESET}"
    else:
        return f"{RED}❌ {ratio*100:5.0f}%{RESET}"


# ─── Main comparison table ─────────────────────────────────────────────────────
def print_comparison(
    rust: dict[tuple[str, int], float],
    talib: dict[tuple[str, int], float],
    target_size: int = 1_000_000,
) -> int:
    """Print comparison table. Returns number of failing indicators."""

    # Collect all indicators that have Rust results at target_size
    indicators = sorted({ind for (ind, sz) in rust if sz == target_size})

    if not indicators:
        print(f"  No Rust benchmark results found for size={target_size:,}")
        print("  Make sure you ran: cargo bench --bench bench_vs_talib -- --output-format bencher")
        return 0

    col_w = 16
    print()
    print(f"  {BOLD}{'Indicator':<{col_w}} {'Rust':>12}  {'ta-lib C':>12}  {'Ratio':>10}  Status{RESET}")
    print("  " + "─" * 72)

    failures = 0
    missing_talib = 0

    for ind in indicators:
        rust_t = rust.get((ind, target_size))
        talib_t = talib.get((ind, target_size))

        rust_s  = fmt_throughput(rust_t)  if rust_t  else "   —   "
        talib_s = fmt_throughput(talib_t) if talib_t else "   —   "

        ratio: Optional[float] = None
        if rust_t and talib_t and talib_t > 0:
            ratio = rust_t / talib_t

        if talib_t is None:
            missing_talib += 1

        st = status_str(ratio)
        if ratio is not None and ratio < THRESHOLD_WARN:
            failures += 1

        print(f"  {ind:<{col_w}} {rust_s:>12}  {talib_s:>12}  {st}")

    print()

    # Summary
    total = len(indicators)
    passing = sum(
        1 for ind in indicators
        if (rust.get((ind, target_size)) or 0) / max(talib.get((ind, target_size)) or 1e-9, 1e-9) >= THRESHOLD_WARN
        and talib.get((ind, target_size)) is not None
    )
    print(f"  Total: {total} indicators benchmarked")
    if missing_talib:
        print(f"  {YELLOW}Missing ta-lib baseline: {missing_talib} indicators (run bench_talib.py){RESET}")
    if failures:
        print(f"  {RED}Below 80% threshold: {failures} indicators{RESET}")
    else:
        print(f"  {GREEN}All benchmarked indicators meet ≥80% threshold ✅{RESET}")

    return failures


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--rust-bench", help="Path to file containing Criterion bencher output (reads stdin if omitted)")
    p.add_argument("--talib-json", help="Path to bench_talib.py JSON output file")
    p.add_argument("--size", type=int, default=1_000_000, help="Data size to compare (default: 1,000,000)")
    p.add_argument("--criterion-dir", help="Path to target/criterion/ for JSON mode")
    args = p.parse_args()

    # Load Rust results
    rust: dict[tuple[str, int], float] = {}
    if args.criterion_dir:
        rust = parse_criterion_json(Path(args.criterion_dir), args.size)
    elif args.rust_bench:
        rust = parse_criterion_bencher(Path(args.rust_bench).read_text())
    else:
        # Read from stdin
        rust = parse_criterion_bencher(sys.stdin.read())

    # Load ta-lib results
    talib: dict[tuple[str, int], float] = {}
    if args.talib_json and Path(args.talib_json).exists():
        talib = load_talib_results(args.talib_json)

    failures = print_comparison(rust, talib, args.size)
    sys.exit(1 if failures > 0 else 0)


if __name__ == "__main__":
    main()
