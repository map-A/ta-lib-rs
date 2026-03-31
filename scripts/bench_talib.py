#!/usr/bin/env python3
"""
scripts/bench_talib.py
======================
Benchmark ta-lib C performance (via Python binding) for comparison with polars-ta.

Uses identical data to the Rust criterion benchmarks to ensure fair comparison.

Requirements:
    pip install ta-lib numpy

Usage:
    python scripts/bench_talib.py [--indicator sma] [--output json]

Output format matches the Rust bench output so compare_all.sh can parse both.
"""

import argparse
import json
import sys
import time

import numpy as np

try:
    import talib
    TALIB_AVAILABLE = True
except ImportError:
    TALIB_AVAILABLE = False
    print("WARNING: ta-lib not installed.", file=sys.stderr)


def make_bench_data(size: int) -> np.ndarray:
    """Identical data to Rust: 100.0 + i * 0.01"""
    return np.arange(size, dtype=np.float64) * 0.01 + 100.0


def make_ohlcv(size: int):
    """OHLCV data matching Rust make_ohlcv helper."""
    close = np.arange(size, dtype=np.float64) * 0.01 + 100.0
    high = close * 1.01
    low = close * 0.99
    volume = np.arange(size, dtype=np.float64) * 10.0 + 1_000_000.0
    return high, low, close, volume


def bench_fn(fn, data, repeat: int = 200) -> dict:
    """
    Run fn(data) repeatedly and collect timing statistics.

    Returns dict with p50, p99 (µs) and throughput (elem/s).
    """
    # 热身
    for _ in range(min(10, repeat)):
        fn(data)

    times_ns = []
    for _ in range(repeat):
        t0 = time.perf_counter_ns()
        fn(data)
        times_ns.append(time.perf_counter_ns() - t0)

    times_ns = np.array(times_ns)
    p50_us = float(np.percentile(times_ns, 50)) / 1000.0
    p99_us = float(np.percentile(times_ns, 99)) / 1000.0
    mean_s = float(np.mean(times_ns)) / 1e9
    size = data[0].shape[0] if isinstance(data, tuple) else len(data)
    throughput = size / mean_s

    return {
        "p50_us": p50_us,
        "p99_us": p99_us,
        "throughput_elem_per_sec": throughput,
        "mean_us": float(np.mean(times_ns)) / 1000.0,
        "size": size,
    }


# ─── 指标基准函数 ──────────────────────────────────────────────────────────────

def bench_sma(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.SMA(d, timeperiod=period), data)
        r["indicator"] = "sma"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_ema(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.EMA(d, timeperiod=period), data)
        r["indicator"] = "ema"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_wma(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.WMA(d, timeperiod=period), data)
        r["indicator"] = "wma"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_dema(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.DEMA(d, timeperiod=period), data)
        r["indicator"] = "dema"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_tema(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.TEMA(d, timeperiod=period), data)
        r["indicator"] = "tema"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_macd(sizes: list[int], fast: int = 12, slow: int = 26, signal: int = 9) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.MACD(d, fastperiod=fast, slowperiod=slow, signalperiod=signal), data)
        r["indicator"] = "macd"
        r["params"] = {"fast": fast, "slow": slow, "signal": signal}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_bbands(sizes: list[int], period: int = 20, nbdevup: float = 2.0, nbdevdn: float = 2.0) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.BBANDS(d, timeperiod=period, nbdevup=nbdevup, nbdevdn=nbdevdn), data)
        r["indicator"] = "bbands"
        r["params"] = {"period": period, "nbdevup": nbdevup, "nbdevdn": nbdevdn}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_sar(sizes: list[int], acceleration: float = 0.02, maximum: float = 0.2) -> list[dict]:
    results = []
    for size in sizes:
        high, low, _, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.SAR(d[0], d[1], acceleration=acceleration, maximum=maximum), (high, low))
        r["indicator"] = "sar"
        r["params"] = {"acceleration": acceleration, "maximum": maximum}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_adx(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.ADX(d[0], d[1], d[2], timeperiod=period), (high, low, close))
        r["indicator"] = "adx"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_rsi(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.RSI(d, timeperiod=period), data)
        r["indicator"] = "rsi"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_cci(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.CCI(d[0], d[1], d[2], timeperiod=period), (high, low, close))
        r["indicator"] = "cci"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_willr(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.WILLR(d[0], d[1], d[2], timeperiod=period), (high, low, close))
        r["indicator"] = "willr"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_stoch(sizes: list[int], fastk: int = 5, slowk: int = 3, slowd: int = 3) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(
            lambda d: talib.STOCH(d[0], d[1], d[2], fastk_period=fastk, slowk_period=slowk, slowd_period=slowd),
            (high, low, close),
        )
        r["indicator"] = "stoch"
        r["params"] = {"fastk": fastk, "slowk": slowk, "slowd": slowd}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_stochrsi(sizes: list[int], period: int = 14, fastk: int = 5, fastd: int = 3) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(
            lambda d: talib.STOCHRSI(d, timeperiod=period, fastk_period=fastk, fastd_period=fastd),
            data,
        )
        r["indicator"] = "stochrsi"
        r["params"] = {"period": period, "fastk": fastk, "fastd": fastd}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_ultosc(sizes: list[int], period1: int = 7, period2: int = 14, period3: int = 28) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(
            lambda d: talib.ULTOSC(d[0], d[1], d[2], timeperiod1=period1, timeperiod2=period2, timeperiod3=period3),
            (high, low, close),
        )
        r["indicator"] = "ultosc"
        r["params"] = {"period1": period1, "period2": period2, "period3": period3}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_obv(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        _, _, close, volume = make_ohlcv(size)
        r = bench_fn(lambda d: talib.OBV(d[0], d[1]), (close, volume))
        r["indicator"] = "obv"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_ad(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, volume = make_ohlcv(size)
        r = bench_fn(lambda d: talib.AD(d[0], d[1], d[2], d[3]), (high, low, close, volume))
        r["indicator"] = "ad"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_adosc(sizes: list[int], fast: int = 3, slow: int = 10) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, volume = make_ohlcv(size)
        r = bench_fn(
            lambda d: talib.ADOSC(d[0], d[1], d[2], d[3], fastperiod=fast, slowperiod=slow),
            (high, low, close, volume),
        )
        r["indicator"] = "adosc"
        r["params"] = {"fast": fast, "slow": slow}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_trange(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.TRANGE(d[0], d[1], d[2]), (high, low, close))
        r["indicator"] = "trange"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_atr(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.ATR(d[0], d[1], d[2], timeperiod=period), (high, low, close))
        r["indicator"] = "atr"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_natr(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.NATR(d[0], d[1], d[2], timeperiod=period), (high, low, close))
        r["indicator"] = "natr"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


BENCH_FUNCTIONS = {
    "sma":      bench_sma,
    "ema":      bench_ema,
    "wma":      bench_wma,
    "dema":     bench_dema,
    "tema":     bench_tema,
    "macd":     bench_macd,
    "bbands":   bench_bbands,
    "sar":      bench_sar,
    "adx":      bench_adx,
    "rsi":      bench_rsi,
    "cci":      bench_cci,
    "willr":    bench_willr,
    "stoch":    bench_stoch,
    "stochrsi": bench_stochrsi,
    "ultosc":   bench_ultosc,
    "obv":      bench_obv,
    "ad":       bench_ad,
    "adosc":    bench_adosc,
    "trange":   bench_trange,
    "atr":      bench_atr,
    "natr":     bench_natr,
}

SIZES = [100, 10_000, 1_000_000]


# ─── 报告输出 ─────────────────────────────────────────────────────────────────

def print_table(results: list[dict]):
    """Print a human-readable benchmark table."""
    print(f"\n{'Indicator':<12} {'Impl':<12} {'Size':>12} {'Throughput (elem/s)':>22} {'p50 (µs)':>10} {'p99 (µs)':>10}")
    print("-" * 85)
    for r in results:
        print(
            f"{r['indicator']:<12} {r['implementation']:<12} {r['size']:>12,} "
            f"{r['throughput_elem_per_sec']:>22,.0f} {r['p50_us']:>10.2f} {r['p99_us']:>10.2f}"
        )


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--indicator", default="all")
    parser.add_argument("--output", choices=["table", "json"], default="table")
    parser.add_argument("--output-file", default=None, help="Write JSON results to file")
    args = parser.parse_args()

    if not TALIB_AVAILABLE:
        print("ERROR: ta-lib is required. Install with: pip install ta-lib")
        sys.exit(1)

    all_results = []

    if args.indicator == "all":
        for name, fn in BENCH_FUNCTIONS.items():
            all_results.extend(fn(SIZES))
    elif args.indicator in BENCH_FUNCTIONS:
        all_results.extend(BENCH_FUNCTIONS[args.indicator](SIZES))
    else:
        print(f"ERROR: Unknown indicator '{args.indicator}'. Available: {', '.join(BENCH_FUNCTIONS)}")
        sys.exit(1)

    if args.output == "json":
        print(json.dumps(all_results, indent=2))
    else:
        print_table(all_results)

    if args.output_file:
        Path(args.output_file).write_text(json.dumps(all_results, indent=2))
        print(f"\nResults written to: {args.output_file}")


if __name__ == "__main__":
    from pathlib import Path
    main()
