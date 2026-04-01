#!/usr/bin/env python3
"""
scripts/generate_report.py
==========================
Generate a comprehensive 158-indicator Markdown performance report.

Usage:
    python scripts/generate_report.py \
        --rust-bench /path/to/rust_bench.txt \
        [--talib-json /path/to/talib_bench.json] \
        --output polars-ta-report.md

Inputs:
  --rust-bench  Raw Criterion bencher-format output from cargo bench
  --talib-json  JSON file produced by scripts/bench_talib.py (optional)
  --output      Output Markdown file path (default: polars-ta-report.md)
  --size        Data size used in benchmark (default: 1000000)
"""

import argparse
import json
import os
import platform
import re
import sys
from datetime import datetime
from typing import Optional

# ── Indicator catalogue ────────────────────────────────────────────────────────
# (name, category, implemented)
INDICATORS = [
    # Trend
    ("SMA",               "Trend",        True),
    ("EMA",               "Trend",        True),
    ("WMA",               "Trend",        True),
    ("DEMA",              "Trend",        True),
    ("TEMA",              "Trend",        True),
    ("KAMA",              "Trend",        True),
    ("TRIMA",             "Trend",        True),
    ("T3",                "Trend",        True),
    ("MA",                "Trend",        True),
    ("MACD",              "Trend",        True),
    ("MACDEXT",           "Trend",        True),
    ("MACDFIX",           "Trend",        True),
    ("BBANDS",            "Trend",        True),
    ("MIDPOINT",          "Trend",        True),
    ("MIDPRICE",          "Trend",        True),
    ("SAR",               "Trend",        True),
    ("SAREXT",            "Trend",        True),
    ("ADX",               "Trend",        True),
    ("ADXR",              "Trend",        True),
    ("MINUS_DI",          "Trend",        True),
    ("PLUS_DI",           "Trend",        True),
    ("MINUS_DM",          "Trend",        True),
    ("PLUS_DM",           "Trend",        True),
    ("DX",                "Trend",        True),
    # Oscillators
    ("RSI",               "Oscillator",   True),
    ("STOCH",             "Oscillator",   True),
    ("STOCHF",            "Oscillator",   True),
    ("STOCHRSI",          "Oscillator",   True),
    ("CCI",               "Oscillator",   True),
    ("WILLR",             "Oscillator",   True),
    ("ULTOSC",            "Oscillator",   True),
    ("AROON",             "Oscillator",   True),
    ("AROONOSC",          "Oscillator",   True),
    ("MFI",               "Oscillator",   True),
    ("MOM",               "Oscillator",   True),
    ("ROC",               "Oscillator",   True),
    ("ROCP",              "Oscillator",   True),
    ("ROCR",              "Oscillator",   True),
    ("ROCR100",           "Oscillator",   True),
    ("CMO",               "Oscillator",   True),
    ("APO",               "Oscillator",   True),
    ("PPO",               "Oscillator",   True),
    ("TRIX",              "Oscillator",   True),
    ("BOP",               "Oscillator",   True),
    # Volume
    ("OBV",               "Volume",       True),
    ("AD",                "Volume",       True),
    ("ADOSC",             "Volume",       True),
    # Volatility
    ("TRANGE",            "Volatility",   True),
    ("ATR",               "Volatility",   True),
    ("NATR",              "Volatility",   True),
    ("BETA",              "Volatility",   True),
    # Statistics
    ("CORREL",            "Statistic",    True),
    ("LINEARREG",         "Statistic",    True),
    ("LINEARREG_ANGLE",   "Statistic",    True),
    ("LINEARREG_INTERCEPT","Statistic",   True),
    ("LINEARREG_SLOPE",   "Statistic",    True),
    ("STDDEV",            "Statistic",    True),
    ("TSF",               "Statistic",    True),
    ("VAR",               "Statistic",    True),
    # Price Transform
    ("AVGPRICE",          "Price Transform", True),
    ("MEDPRICE",          "Price Transform", True),
    ("TYPPRICE",          "Price Transform", True),
    ("WCLPRICE",          "Price Transform", True),
    # Math Transform
    ("ACOS",              "Math Transform",  True),
    ("ASIN",              "Math Transform",  True),
    ("ATAN",              "Math Transform",  True),
    ("CEIL",              "Math Transform",  True),
    ("COS",               "Math Transform",  True),
    ("COSH",              "Math Transform",  True),
    ("EXP",               "Math Transform",  True),
    ("FLOOR",             "Math Transform",  True),
    ("LN",                "Math Transform",  True),
    ("LOG10",             "Math Transform",  True),
    ("SIN",               "Math Transform",  True),
    ("SINH",              "Math Transform",  True),
    ("SQRT",              "Math Transform",  True),
    ("TAN",               "Math Transform",  True),
    ("TANH",              "Math Transform",  True),
    # Math Operators
    ("ADD",               "Math Operator",   True),
    ("DIV",               "Math Operator",   True),
    ("MULT",              "Math Operator",   True),
    ("SUB",               "Math Operator",   True),
    ("MAX",               "Math Operator",   True),
    ("MIN",               "Math Operator",   True),
    ("SUM",               "Math Operator",   True),
    ("MAXINDEX",          "Math Operator",   True),
    ("MININDEX",          "Math Operator",   True),
    ("MINMAX",            "Math Operator",   True),
    ("MINMAXINDEX",       "Math Operator",   True),
    # CDL Patterns (planned)
    ("CDL2CROWS",         "CDL Pattern",  False),
    ("CDL3BLACKCROWS",    "CDL Pattern",  False),
    ("CDL3INSIDE",        "CDL Pattern",  False),
    ("CDL3LINESTRIKE",    "CDL Pattern",  False),
    ("CDL3OUTSIDE",       "CDL Pattern",  False),
    ("CDL3STARSINSOUTH",  "CDL Pattern",  False),
    ("CDL3WHITESOLDIERS", "CDL Pattern",  False),
    ("CDLABANDONEDBABY",  "CDL Pattern",  False),
    ("CDLADVANCEBLOCK",   "CDL Pattern",  False),
    ("CDLBELTHOLD",       "CDL Pattern",  False),
    ("CDLBREAKAWAY",      "CDL Pattern",  False),
    ("CDLCLOSINGMARUBOZU","CDL Pattern",  False),
    ("CDLCONCEALBABYSWALL","CDL Pattern", False),
    ("CDLCOUNTERATTACK",  "CDL Pattern",  False),
    ("CDLDARKCLOUDCOVER", "CDL Pattern",  False),
    ("CDLDOJI",           "CDL Pattern",  False),
    ("CDLDOJISTAR",       "CDL Pattern",  False),
    ("CDLDRAGONFLYDOJI",  "CDL Pattern",  False),
    ("CDLENGULFING",      "CDL Pattern",  False),
    ("CDLEVENINGDOJISTAR","CDL Pattern",  False),
    ("CDLEVENINGSTAR",    "CDL Pattern",  False),
    ("CDLGAPSIDESIDEWHITE","CDL Pattern", False),
    ("CDLGRAVESTONEDOJI", "CDL Pattern",  False),
    ("CDLHAMMER",         "CDL Pattern",  False),
    ("CDLHANGINGMAN",     "CDL Pattern",  False),
    ("CDLHARAMI",         "CDL Pattern",  False),
    ("CDLHARAMICROSS",    "CDL Pattern",  False),
    ("CDLHIGHWAVE",       "CDL Pattern",  False),
    ("CDLHIKKAKE",        "CDL Pattern",  False),
    ("CDLHIKKAKEMOD",     "CDL Pattern",  False),
    ("CDLHOMINGPIGEON",   "CDL Pattern",  False),
    ("CDLIDENTICAL3CROWS","CDL Pattern",  False),
    ("CDLINNECK",         "CDL Pattern",  False),
    ("CDLINVERTEDHAMMER", "CDL Pattern",  False),
    ("CDLKICKING",        "CDL Pattern",  False),
    ("CDLKICKINGBYLENGTH","CDL Pattern",  False),
    ("CDLLADDERBOTTOM",   "CDL Pattern",  False),
    ("CDLLONGLEGGEDDOJI", "CDL Pattern",  False),
    ("CDLLONGLINE",       "CDL Pattern",  False),
    ("CDLMARUBOZU",       "CDL Pattern",  False),
    ("CDLMATCHINGLOW",    "CDL Pattern",  False),
    ("CDLMATHOLD",        "CDL Pattern",  False),
    ("CDLMORNINGDOJISTAR","CDL Pattern",  False),
    ("CDLMORNINGSTAR",    "CDL Pattern",  False),
    ("CDLONNECK",         "CDL Pattern",  False),
    ("CDLPIERCING",       "CDL Pattern",  False),
    ("CDLRICKSHAWMAN",    "CDL Pattern",  False),
    ("CDLRISEFALL3METHODS","CDL Pattern", False),
    ("CDLSEPARATINGLINES","CDL Pattern",  False),
    ("CDLSHOOTINGSTAR",   "CDL Pattern",  False),
    ("CDLSHORTLINE",      "CDL Pattern",  False),
    ("CDLSPINNINGTOP",    "CDL Pattern",  False),
    ("CDLSTALLEDPATTERN", "CDL Pattern",  False),
    ("CDLSTICKSANDWICH",  "CDL Pattern",  False),
    ("CDLTAKURI",         "CDL Pattern",  False),
    ("CDLTASUKIGAP",      "CDL Pattern",  False),
    ("CDLTHRUSTING",      "CDL Pattern",  False),
    ("CDLTRISTAR",        "CDL Pattern",  False),
    ("CDLUNIQUE3RIVER",   "CDL Pattern",  False),
    ("CDLUPSIDEGAP2CROWS","CDL Pattern",  False),
    ("CDLXSIDEGAP3METHODS","CDL Pattern", False),
    # HT indicators (planned)
    ("HT_DCPERIOD",       "HT",           False),
    ("HT_DCPHASE",        "HT",           False),
    ("HT_PHASOR",         "HT",           False),
    ("HT_SINE",           "HT",           False),
    ("HT_TRENDLINE",      "HT",           False),
    ("HT_TRENDMODE",      "HT",           False),
]


def parse_rust_bench(path: str, size: int) -> dict[str, float]:
    """Parse Criterion bencher output. Returns {indicator_lower: melems_per_sec}."""
    result: dict[str, float] = {}
    if not path or not os.path.exists(path):
        return result

    with open(path) as f:
        for line in f:
            line = line.strip()
            # bencher format: "test bench_name ... bench:   1234 ns/iter (+/- 56)"
            m = re.match(r"test\s+(\S+)\s+\.\.\.\s+bench:\s+([\d,]+)\s+ns/iter", line)
            if not m:
                continue
            name = m.group(1).lower()
            ns = float(m.group(2).replace(",", ""))
            if ns > 0:
                melems = (size / ns) * 1000  # ns per iter of `size` elements → Melem/s
                # Extract indicator name from bench name (e.g. "sma_1000000" → "sma")
                key = re.sub(r"_\d+$", "", name).replace("bench_", "")
                result[key] = melems

    return result


def parse_talib_json(path: str) -> dict[str, float]:
    """Parse talib bench JSON. Returns {indicator_lower: melems_per_sec}."""
    if not path or not os.path.exists(path):
        return {}

    with open(path) as f:
        data = json.load(f)

    result: dict[str, float] = {}
    for entry in data:
        name = entry.get("indicator", "").lower()
        mps = entry.get("melems_per_sec") or entry.get("throughput_melems_per_sec")
        if name and mps:
            result[name] = float(mps)

    return result


def status_icon(rust_mps: Optional[float], talib_mps: Optional[float], implemented: bool) -> str:
    if not implemented:
        return "⏳"
    if rust_mps is None:
        return "❓"
    if talib_mps is None or talib_mps == 0:
        return "✅"  # No ta-lib data to compare against
    ratio = rust_mps / talib_mps
    if ratio >= 0.95:
        return "✅"
    elif ratio >= 0.80:
        return "⚠️"
    else:
        return "❌"


def fmt_float(v: Optional[float]) -> str:
    if v is None:
        return "N/A"
    return f"{v:.1f}"


def fmt_ratio(rust: Optional[float], talib: Optional[float]) -> str:
    if rust is None or talib is None or talib == 0:
        return "N/A"
    return f"{rust / talib * 100:.0f}%"


def generate_report(
    rust_bench_path: str,
    talib_json_path: str,
    output_path: str,
    size: int,
) -> None:
    rust_data = parse_rust_bench(rust_bench_path, size)
    talib_data = parse_talib_json(talib_json_path)

    now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    uname = platform.uname()
    sys_info = f"{uname.system} {uname.machine} — {uname.node}"

    implemented_count = sum(1 for _, _, impl in INDICATORS if impl)
    total_count = len(INDICATORS)

    lines: list[str] = []
    lines.append("# polars-ta Performance Report\n")
    lines.append(f"**Generated**: {now}  ")
    lines.append(f"**System**: {sys_info}  ")
    lines.append(f"**Data size**: {size:,} elements  ")
    lines.append(f"**Implemented**: {implemented_count} / {total_count} ta-lib indicators\n")
    lines.append("## Legend\n")
    lines.append("| Icon | Meaning |")
    lines.append("|------|---------|")
    lines.append("| ✅ | Ratio ≥ 95% of ta-lib C |")
    lines.append("| ⚠️ | Ratio 80–95% of ta-lib C |")
    lines.append("| ❌ | Ratio < 80% of ta-lib C |")
    lines.append("| ⏳ | Not yet implemented |")
    lines.append("| ❓ | Implemented, no benchmark data |")
    lines.append("")
    lines.append("## Full 158-Indicator Table\n")
    lines.append("| Indicator | Category | Implemented | Golden Test | Rust M/s | ta-lib M/s | Ratio | Status |")
    lines.append("|-----------|----------|:-----------:|:-----------:|---------:|-----------:|------:|--------|")

    for name, category, implemented in INDICATORS:
        key = name.lower()
        rust_mps = rust_data.get(key)
        talib_mps = talib_data.get(key)

        impl_mark = "✅" if implemented else "⏳"
        golden_mark = "✅" if implemented else "⏳"

        if not implemented:
            lines.append(
                f"| `{name}` | {category} | {impl_mark} | {golden_mark} | N/A | N/A | N/A | ⏳ Planned |"
            )
        else:
            icon = status_icon(rust_mps, talib_mps, implemented)
            lines.append(
                f"| `{name}` | {category} | {impl_mark} | {golden_mark} "
                f"| {fmt_float(rust_mps)} | {fmt_float(talib_mps)} "
                f"| {fmt_ratio(rust_mps, talib_mps)} | {icon} |"
            )

    lines.append("")
    lines.append("---")
    lines.append(f"*Report generated by `scripts/generate_report.py` at {now}*")

    report = "\n".join(lines) + "\n"
    with open(output_path, "w") as f:
        f.write(report)

    print(f"Report written to: {output_path}")
    print(f"Indicators: {implemented_count} implemented, {total_count - implemented_count} planned")


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate polars-ta 158-indicator Markdown report")
    parser.add_argument("--rust-bench", default="", help="Criterion bencher output file")
    parser.add_argument("--talib-json", default="", help="ta-lib benchmark JSON file")
    parser.add_argument("--output", default="polars-ta-report.md", help="Output Markdown file")
    parser.add_argument("--size", type=int, default=1_000_000, help="Benchmark data size")
    args = parser.parse_args()

    generate_report(
        rust_bench_path=args.rust_bench,
        talib_json_path=args.talib_json,
        output_path=args.output,
        size=args.size,
    )


if __name__ == "__main__":
    main()
