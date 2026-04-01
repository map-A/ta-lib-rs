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
    """Realistic price-like data: sinusoidal to avoid monotonic fast-paths."""
    i = np.arange(size, dtype=np.float64)
    return 100.0 + np.sin(i * 0.01) * 10.0 + np.sin(i * 0.003) * 5.0


def make_ohlcv(size: int):
    """OHLCV data matching Rust make_ohlcv helper."""
    close = make_bench_data(size)
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


# ─── Phase 2 振荡器 ────────────────────────────────────────────────────────────

def bench_mom(sizes: list[int], period: int = 10) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.MOM(d, timeperiod=period), data)
        r["indicator"] = "mom"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_roc(sizes: list[int], period: int = 10) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.ROC(d, timeperiod=period), data)
        r["indicator"] = "roc"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_rocp(sizes: list[int], period: int = 10) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.ROCP(d, timeperiod=period), data)
        r["indicator"] = "rocp"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_rocr(sizes: list[int], period: int = 10) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.ROCR(d, timeperiod=period), data)
        r["indicator"] = "rocr"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_rocr100(sizes: list[int], period: int = 10) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.ROCR100(d, timeperiod=period), data)
        r["indicator"] = "rocr100"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_cmo(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.CMO(d, timeperiod=period), data)
        r["indicator"] = "cmo"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_apo(sizes: list[int], fast: int = 12, slow: int = 26) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.APO(d, fastperiod=fast, slowperiod=slow), data)
        r["indicator"] = "apo"
        r["params"] = {"fast": fast, "slow": slow}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_ppo(sizes: list[int], fast: int = 12, slow: int = 26) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.PPO(d, fastperiod=fast, slowperiod=slow), data)
        r["indicator"] = "ppo"
        r["params"] = {"fast": fast, "slow": slow}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_trix(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.TRIX(d, timeperiod=period), data)
        r["indicator"] = "trix"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_bop(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        open_ = close * 1.005
        r = bench_fn(lambda d: talib.BOP(d[0], d[1], d[2], d[3]), (open_, high, low, close))
        r["indicator"] = "bop"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_minus_dm(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, _, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.MINUS_DM(d[0], d[1], timeperiod=period), (high, low))
        r["indicator"] = "minus_dm"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_plus_dm(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, _, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.PLUS_DM(d[0], d[1], timeperiod=period), (high, low))
        r["indicator"] = "plus_dm"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_minus_di(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.MINUS_DI(d[0], d[1], d[2], timeperiod=period), (high, low, close))
        r["indicator"] = "minus_di"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_plus_di(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.PLUS_DI(d[0], d[1], d[2], timeperiod=period), (high, low, close))
        r["indicator"] = "plus_di"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_dx(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.DX(d[0], d[1], d[2], timeperiod=period), (high, low, close))
        r["indicator"] = "dx"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_adxr(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.ADXR(d[0], d[1], d[2], timeperiod=period), (high, low, close))
        r["indicator"] = "adxr"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


# ─── Phase 2 统计 ──────────────────────────────────────────────────────────────

def bench_beta(sizes: list[int], period: int = 5) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        data2 = data + 1.0
        r = bench_fn(lambda d: talib.BETA(d[0], d[1], timeperiod=period), (data, data2))
        r["indicator"] = "beta"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_correl(sizes: list[int], period: int = 30) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        data2 = data + 1.0
        r = bench_fn(lambda d: talib.CORREL(d[0], d[1], timeperiod=period), (data, data2))
        r["indicator"] = "correl"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_linearreg(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.LINEARREG(d, timeperiod=period), data)
        r["indicator"] = "linearreg"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_stddev(sizes: list[int], period: int = 5, nbdev: float = 1.0) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.STDDEV(d, timeperiod=period, nbdev=nbdev), data)
        r["indicator"] = "stddev"
        r["params"] = {"period": period, "nbdev": nbdev}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_tsf(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.TSF(d, timeperiod=period), data)
        r["indicator"] = "tsf"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_var(sizes: list[int], period: int = 5, nbdev: float = 1.0) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.VAR(d, timeperiod=period, nbdev=nbdev), data)
        r["indicator"] = "var"
        r["params"] = {"period": period, "nbdev": nbdev}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


# ─── Phase 2 趋势 ──────────────────────────────────────────────────────────────

def bench_kama(sizes: list[int], period: int = 30) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.KAMA(d, timeperiod=period), data)
        r["indicator"] = "kama"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_trima(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.TRIMA(d, timeperiod=period), data)
        r["indicator"] = "trima"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_t3(sizes: list[int], period: int = 5) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.T3(d, timeperiod=period), data)
        r["indicator"] = "t3"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_midpoint(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.MIDPOINT(d, timeperiod=period), data)
        r["indicator"] = "midpoint"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_midprice(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, _, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.MIDPRICE(d[0], d[1], timeperiod=period), (high, low))
        r["indicator"] = "midprice"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


# ─── Phase 1 missing: aroon, mfi ──────────────────────────────────────────────

def bench_aroon(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, _, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.AROON(d[0], d[1], timeperiod=period), (high, low))
        r["indicator"] = "aroon"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_mfi(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, volume = make_ohlcv(size)
        r = bench_fn(lambda d: talib.MFI(d[0], d[1], d[2], d[3], timeperiod=period), (high, low, close, volume))
        r["indicator"] = "mfi"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


# ─── Phase 3: Math Operators ──────────────────────────────────────────────────

def bench_add(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        data2 = make_bench_data(size)
        r = bench_fn(lambda d: talib.ADD(d[0], d[1]), (data, data2))
        r["indicator"] = "add"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_div(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        data2 = make_bench_data(size)
        r = bench_fn(lambda d: talib.DIV(d[0], d[1]), (data, data2))
        r["indicator"] = "div"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_mult(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        data2 = make_bench_data(size)
        r = bench_fn(lambda d: talib.MULT(d[0], d[1]), (data, data2))
        r["indicator"] = "mult"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_sub(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        data2 = make_bench_data(size)
        r = bench_fn(lambda d: talib.SUB(d[0], d[1]), (data, data2))
        r["indicator"] = "sub"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_max(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.MAX(d, timeperiod=period), data)
        r["indicator"] = "max"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_min(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.MIN(d, timeperiod=period), data)
        r["indicator"] = "min"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_sum(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.SUM(d, timeperiod=period), data)
        r["indicator"] = "sum"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_maxindex(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.MAXINDEX(d, timeperiod=period), data)
        r["indicator"] = "maxindex"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_minindex(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.MININDEX(d, timeperiod=period), data)
        r["indicator"] = "minindex"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_minmax(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.MINMAX(d, timeperiod=period), data)
        r["indicator"] = "minmax"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_minmaxindex(sizes: list[int], period: int = 20) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.MINMAXINDEX(d, timeperiod=period), data)
        r["indicator"] = "minmaxindex"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


# ─── Phase 3: Math Transform ──────────────────────────────────────────────────

def bench_acos(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        # 归一化到 [-1, 1] 以确保 acos 有合法输入
        close_norm = (data - data.min()) / (data.max() - data.min()) * 2 - 1
        r = bench_fn(lambda d: talib.ACOS(d), close_norm)
        r["indicator"] = "acos"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_asin(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        close_norm = (data - data.min()) / (data.max() - data.min()) * 2 - 1
        r = bench_fn(lambda d: talib.ASIN(d), close_norm)
        r["indicator"] = "asin"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_atan(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.ATAN(d), data)
        r["indicator"] = "atan"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_ceil(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.CEIL(d), data)
        r["indicator"] = "ceil"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_cos(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.COS(d), data)
        r["indicator"] = "cos"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_cosh(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.COSH(d), data)
        r["indicator"] = "cosh"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_exp(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.EXP(d), data)
        r["indicator"] = "exp"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_floor(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.FLOOR(d), data)
        r["indicator"] = "floor"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_ln(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.LN(d), data)
        r["indicator"] = "ln"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_log10(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.LOG10(d), data)
        r["indicator"] = "log10"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_sin(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.SIN(d), data)
        r["indicator"] = "sin"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_sinh(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.SINH(d), data)
        r["indicator"] = "sinh"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_sqrt(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.SQRT(d), data)
        r["indicator"] = "sqrt"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_tan(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.TAN(d), data)
        r["indicator"] = "tan"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_tanh(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.TANH(d), data)
        r["indicator"] = "tanh"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


# ─── Phase 3: Price Transform ─────────────────────────────────────────────────

def bench_avgprice(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        open_ = close * 0.995
        r = bench_fn(lambda d: talib.AVGPRICE(d[0], d[1], d[2], d[3]), (open_, high, low, close))
        r["indicator"] = "avgprice"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_medprice(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        high, low, _, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.MEDPRICE(d[0], d[1]), (high, low))
        r["indicator"] = "medprice"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_typprice(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.TYPPRICE(d[0], d[1], d[2]), (high, low, close))
        r["indicator"] = "typprice"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_wclprice(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.WCLPRICE(d[0], d[1], d[2]), (high, low, close))
        r["indicator"] = "wclprice"
        r["params"] = {}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


# ─── Phase 3: 振荡器与趋势 ─────────────────────────────────────────────────────

def bench_aroonosc(sizes: list[int], period: int = 14) -> list[dict]:
    results = []
    for size in sizes:
        high, low, _, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.AROONOSC(d[0], d[1], timeperiod=period), (high, low))
        r["indicator"] = "aroonosc"
        r["params"] = {"period": period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_stochf(sizes: list[int], fastk_period: int = 5, fastd_period: int = 3) -> list[dict]:
    results = []
    for size in sizes:
        high, low, close, _ = make_ohlcv(size)
        r = bench_fn(
            lambda d: talib.STOCHF(d[0], d[1], d[2], fastk_period=fastk_period, fastd_period=fastd_period),
            (high, low, close),
        )
        r["indicator"] = "stochf"
        r["params"] = {"fastk_period": fastk_period, "fastd_period": fastd_period}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_ma(sizes: list[int], period: int = 20, matype: int = 1) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.MA(d, timeperiod=period, matype=matype), data)
        r["indicator"] = "ma"
        r["params"] = {"period": period, "matype": matype}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_macdext(
    sizes: list[int],
    fastperiod: int = 12,
    fastmatype: int = 1,
    slowperiod: int = 26,
    slowmatype: int = 1,
    signalperiod: int = 9,
    signalmatype: int = 1,
) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(
            lambda d: talib.MACDEXT(
                d,
                fastperiod=fastperiod,
                fastmatype=fastmatype,
                slowperiod=slowperiod,
                slowmatype=slowmatype,
                signalperiod=signalperiod,
                signalmatype=signalmatype,
            ),
            data,
        )
        r["indicator"] = "macdext"
        r["params"] = {"fastperiod": fastperiod, "slowperiod": slowperiod, "signalperiod": signalperiod}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_macdfix(sizes: list[int], signalperiod: int = 9) -> list[dict]:
    results = []
    for size in sizes:
        data = make_bench_data(size)
        r = bench_fn(lambda d: talib.MACDFIX(d, signalperiod=signalperiod), data)
        r["indicator"] = "macdfix"
        r["params"] = {"signalperiod": signalperiod}
        r["implementation"] = "talib_c"
        results.append(r)
    return results


def bench_sarext(sizes: list[int]) -> list[dict]:
    results = []
    for size in sizes:
        high, low, _, _ = make_ohlcv(size)
        r = bench_fn(lambda d: talib.SAREXT(d[0], d[1]), (high, low))
        r["indicator"] = "sarext"
        r["params"] = {}
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
    # Phase 2 振荡器
    "mom":      bench_mom,
    "roc":      bench_roc,
    "rocp":     bench_rocp,
    "rocr":     bench_rocr,
    "rocr100":  bench_rocr100,
    "cmo":      bench_cmo,
    "apo":      bench_apo,
    "ppo":      bench_ppo,
    "trix":     bench_trix,
    "bop":      bench_bop,
    "minus_dm": bench_minus_dm,
    "plus_dm":  bench_plus_dm,
    "minus_di": bench_minus_di,
    "plus_di":  bench_plus_di,
    "dx":       bench_dx,
    "adxr":     bench_adxr,
    # Phase 2 统计
    "beta":     bench_beta,
    "correl":   bench_correl,
    "linearreg": bench_linearreg,
    "stddev":   bench_stddev,
    "tsf":      bench_tsf,
    "var":      bench_var,
    # Phase 2 趋势
    "kama":     bench_kama,
    "trima":    bench_trima,
    "t3":       bench_t3,
    "midpoint": bench_midpoint,
    "midprice": bench_midprice,
    # Phase 1 missing
    "aroon":    bench_aroon,
    "mfi":      bench_mfi,
    # Phase 3 数学运算符
    "add":          bench_add,
    "div":          bench_div,
    "mult":         bench_mult,
    "sub":          bench_sub,
    "max":          bench_max,
    "min":          bench_min,
    "sum":          bench_sum,
    "maxindex":     bench_maxindex,
    "minindex":     bench_minindex,
    "minmax":       bench_minmax,
    "minmaxindex":  bench_minmaxindex,
    # Phase 3 数学变换
    "acos":     bench_acos,
    "asin":     bench_asin,
    "atan":     bench_atan,
    "ceil":     bench_ceil,
    "cos":      bench_cos,
    "cosh":     bench_cosh,
    "exp":      bench_exp,
    "floor":    bench_floor,
    "ln":       bench_ln,
    "log10":    bench_log10,
    "sin":      bench_sin,
    "sinh":     bench_sinh,
    "sqrt":     bench_sqrt,
    "tan":      bench_tan,
    "tanh":     bench_tanh,
    # Phase 3 价格变换
    "avgprice": bench_avgprice,
    "medprice": bench_medprice,
    "typprice": bench_typprice,
    "wclprice": bench_wclprice,
    # Phase 3 振荡器与趋势
    "aroonosc": bench_aroonosc,
    "stochf":   bench_stochf,
    "ma":       bench_ma,
    "macdext":  bench_macdext,
    "macdfix":  bench_macdfix,
    "sarext":   bench_sarext,
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
