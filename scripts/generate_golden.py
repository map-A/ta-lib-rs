#!/usr/bin/env python3
"""
scripts/generate_golden.py
==========================
Generate Golden Test JSON files for all implemented indicators.

Golden files are the authoritative reference for numerical correctness.
They are generated ONCE from ta-lib C (via Python binding) and version-controlled.
Do NOT edit golden files manually.

Requirements:
    pip install ta-lib numpy

Usage:
    python scripts/generate_golden.py [--indicator sma] [--output-dir tests/golden]

Generates 7 datasets per indicator (5 standard + 2 boundary, unless lookback=0):
    1. normal_1000       — random OHLCV, seed=42, 1000 rows
    2. boundary_exact    — length = lookback + 1 (exactly 1 output)
    3. boundary_short    — length = lookback (empty output expected)
    4. with_nan_5pct     — normal_1000 with 5% random NaN inserted
    5. all_same_value    — all values = 100.0, 1000 rows
    6. real_btcusdt_1d   — BTC/USDT daily candles (synthetic if unavailable)
    7. real_flat_period  — low-volatility flat market, 500 rows
"""

import argparse
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

import numpy as np

try:
    import talib
    TALIB_AVAILABLE = True
    TALIB_VERSION = talib.__version__
except ImportError:
    TALIB_AVAILABLE = False
    TALIB_VERSION = "unavailable"
    print("WARNING: ta-lib not installed. Golden files cannot be generated.", file=sys.stderr)


# ─── 数据集生成 ────────────────────────────────────────────────────────────────

def make_normal_1000() -> dict:
    rng = np.random.default_rng(seed=42)
    close = 100.0 + rng.normal(0, 2, 1000).cumsum()
    close = np.abs(close)
    high  = close * (1 + rng.uniform(0, 0.02, 1000))
    low   = close * (1 - rng.uniform(0, 0.02, 1000))
    open_ = close * (1 + rng.normal(0, 0.005, 1000))
    volume = rng.uniform(1e6, 1e7, 1000)
    return {"open": open_, "high": high, "low": low, "close": close, "volume": volume}


def make_boundary_exact(lookback: int) -> dict:
    n = lookback + 1
    rng = np.random.default_rng(seed=123)
    close = 100.0 + rng.normal(0, 1, n).cumsum()
    close = np.abs(close)
    high  = close * 1.01
    low   = close * 0.99
    open_ = close
    volume = np.ones(n) * 1e6
    return {"open": open_, "high": high, "low": low, "close": close, "volume": volume}


def make_boundary_short(lookback: int) -> dict:
    n = max(lookback, 1)
    close = np.ones(n) * 100.0
    high  = close * 1.01
    low   = close * 0.99
    open_ = close
    volume = np.ones(n) * 1e6
    return {"open": open_, "high": high, "low": low, "close": close, "volume": volume}


def make_with_nan_5pct() -> dict:
    data = make_normal_1000()
    rng = np.random.default_rng(seed=99)
    for col in ["close", "high", "low"]:
        arr = data[col].copy()
        nan_idx = rng.choice(len(arr), size=int(len(arr) * 0.05), replace=False)
        arr[nan_idx] = np.nan
        data[col] = arr
    return data


def make_all_same_value() -> dict:
    n = 1000
    close = np.ones(n) * 100.0
    high  = np.ones(n) * 101.0
    low   = np.ones(n) * 99.0
    open_ = close.copy()
    volume = np.ones(n) * 1e6
    return {"open": open_, "high": high, "low": low, "close": close, "volume": volume}


def make_real_btcusdt_1d() -> dict:
    rng = np.random.default_rng(seed=2024)
    log_returns = rng.normal(0.001, 0.03, 1000)
    close = 30000.0 * np.exp(np.cumsum(log_returns))
    high  = close * (1 + rng.uniform(0.001, 0.03, 1000))
    low   = close * (1 - rng.uniform(0.001, 0.03, 1000))
    open_ = np.roll(close, 1)
    open_[0] = close[0]
    volume = rng.uniform(1e9, 1e10, 1000)
    return {"open": open_, "high": high, "low": low, "close": close, "volume": volume}


def make_real_flat_period() -> dict:
    n = 500
    rng = np.random.default_rng(seed=555)
    close = 100.0 + rng.normal(0, 0.05, n).cumsum() * 0.001
    close = np.abs(close)
    high  = close + 0.05
    low   = close - 0.05
    open_ = close.copy()
    volume = rng.uniform(1e5, 5e5, n)
    return {"open": open_, "high": high, "low": low, "close": close, "volume": volume}


DATASETS = {
    "normal_1000":      make_normal_1000,
    "with_nan_5pct":    make_with_nan_5pct,
    "all_same_value":   make_all_same_value,
    "real_btcusdt_1d":  make_real_btcusdt_1d,
    "real_flat_period": make_real_flat_period,
}


def _serialize_array(arr: np.ndarray) -> list:
    return [None if (np.isnan(v) or np.isinf(v)) else float(v) for v in arr]


def _write_golden(
    output_dir: Path,
    indicator: str,
    params: dict,
    input_cols: dict,
    output_arrays: dict,
    lookback: int,
    dataset_name: str,
):
    """Write a single golden JSON file.

    output_arrays: dict mapping output key names to np.ndarray
      e.g. {"values": result} or {"macd": arr1, "signal": arr2, "hist": arr3}
    """
    param_str = "_".join(f"{k}{v}" for k, v in params.items())
    fname = output_dir / f"{indicator}_{param_str}_{dataset_name}.json"

    # output_length = count of non-NaN in FIRST output array
    first_arr = next(iter(output_arrays.values()))
    valid_count = int((~np.isnan(first_arr)).sum())

    payload = {
        "meta": {
            "indicator": indicator,
            "params": params,
            "talib_version": TALIB_VERSION,
            "generated_at": datetime.now(timezone.utc).isoformat(),
            "dataset": dataset_name,
        },
        "input": {col: _serialize_array(arr) for col, arr in input_cols.items()},
        "output": {key: _serialize_array(arr) for key, arr in output_arrays.items()},
        "lookback": lookback,
        "output_length": valid_count,
    }
    fname.write_text(json.dumps(payload, indent=2))
    print(f"  Generated: {fname.name}")


def _compute_lookback(result: np.ndarray) -> int:
    """Compute lookback from a ta-lib result array (count of leading NaNs)."""
    return int(np.sum(np.isnan(result)))


# ─── 指标生成函数 ──────────────────────────────────────────────────────────────

def generate_sma(output_dir: Path, period: int = 20):
    print(f"\n[SMA] period={period}")
    lookback = period - 1

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.SMA(data["close"], timeperiod=period)
        _write_golden(output_dir, "sma", {"period": period},
                      {"close": data["close"]}, {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.SMA(bdata["close"], timeperiod=period)
    _write_golden(output_dir, "sma", {"period": period},
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.SMA(sdata["close"], timeperiod=period)
    _write_golden(output_dir, "sma", {"period": period},
                  {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_ema(output_dir: Path, period: int = 20):
    print(f"\n[EMA] period={period}")
    data0 = make_normal_1000()
    r0 = talib.EMA(data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.EMA(data["close"], timeperiod=period)
        _write_golden(output_dir, "ema", {"period": period},
                      {"close": data["close"]}, {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.EMA(bdata["close"], timeperiod=period)
    _write_golden(output_dir, "ema", {"period": period},
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.EMA(sdata["close"], timeperiod=period)
    _write_golden(output_dir, "ema", {"period": period},
                  {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_wma(output_dir: Path, period: int = 20):
    print(f"\n[WMA] period={period}")
    data0 = make_normal_1000()
    r0 = talib.WMA(data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.WMA(data["close"], timeperiod=period)
        _write_golden(output_dir, "wma", {"period": period},
                      {"close": data["close"]}, {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.WMA(bdata["close"], timeperiod=period)
    _write_golden(output_dir, "wma", {"period": period},
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.WMA(sdata["close"], timeperiod=period)
    _write_golden(output_dir, "wma", {"period": period},
                  {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_dema(output_dir: Path, period: int = 20):
    print(f"\n[DEMA] period={period}")
    data0 = make_normal_1000()
    r0 = talib.DEMA(data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.DEMA(data["close"], timeperiod=period)
        _write_golden(output_dir, "dema", {"period": period},
                      {"close": data["close"]}, {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.DEMA(bdata["close"], timeperiod=period)
    _write_golden(output_dir, "dema", {"period": period},
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.DEMA(sdata["close"], timeperiod=period)
    _write_golden(output_dir, "dema", {"period": period},
                  {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_tema(output_dir: Path, period: int = 20):
    print(f"\n[TEMA] period={period}")
    data0 = make_normal_1000()
    r0 = talib.TEMA(data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.TEMA(data["close"], timeperiod=period)
        _write_golden(output_dir, "tema", {"period": period},
                      {"close": data["close"]}, {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.TEMA(bdata["close"], timeperiod=period)
    _write_golden(output_dir, "tema", {"period": period},
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.TEMA(sdata["close"], timeperiod=period)
    _write_golden(output_dir, "tema", {"period": period},
                  {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_macd(output_dir: Path, fast_period: int = 12, slow_period: int = 26, signal_period: int = 9):
    print(f"\n[MACD] fast={fast_period} slow={slow_period} signal={signal_period}")
    data0 = make_normal_1000()
    m0, s0, h0 = talib.MACD(data0["close"], fastperiod=fast_period, slowperiod=slow_period, signalperiod=signal_period)
    lookback = _compute_lookback(m0)
    params = {"fast": fast_period, "slow": slow_period, "signal": signal_period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        macd_r, sig_r, hist_r = talib.MACD(data["close"], fastperiod=fast_period, slowperiod=slow_period, signalperiod=signal_period)
        _write_golden(output_dir, "macd", params,
                      {"close": data["close"]},
                      {"macd": macd_r, "signal": sig_r, "hist": hist_r},
                      lookback, name)

    bdata = make_boundary_exact(lookback)
    macd_r, sig_r, hist_r = talib.MACD(bdata["close"], fastperiod=fast_period, slowperiod=slow_period, signalperiod=signal_period)
    _write_golden(output_dir, "macd", params, {"close": bdata["close"]},
                  {"macd": macd_r, "signal": sig_r, "hist": hist_r}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    macd_r, sig_r, hist_r = talib.MACD(sdata["close"], fastperiod=fast_period, slowperiod=slow_period, signalperiod=signal_period)
    _write_golden(output_dir, "macd", params, {"close": sdata["close"]},
                  {"macd": macd_r, "signal": sig_r, "hist": hist_r}, lookback, "boundary_short")


def generate_bbands(output_dir: Path, period: int = 20, nbdevup: float = 2.0, nbdevdn: float = 2.0):
    print(f"\n[BBands] period={period} nbdevup={nbdevup} nbdevdn={nbdevdn}")
    data0 = make_normal_1000()
    u0, m0, l0 = talib.BBANDS(data0["close"], timeperiod=period, nbdevup=nbdevup, nbdevdn=nbdevdn, matype=0)
    lookback = _compute_lookback(u0)
    params = {"period": period, "nbdevup": nbdevup, "nbdevdn": nbdevdn}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        u, m, l = talib.BBANDS(data["close"], timeperiod=period, nbdevup=nbdevup, nbdevdn=nbdevdn, matype=0)
        _write_golden(output_dir, "bbands", params,
                      {"close": data["close"]},
                      {"upper": u, "middle": m, "lower": l},
                      lookback, name)

    bdata = make_boundary_exact(lookback)
    u, m, l = talib.BBANDS(bdata["close"], timeperiod=period, nbdevup=nbdevup, nbdevdn=nbdevdn, matype=0)
    _write_golden(output_dir, "bbands", params, {"close": bdata["close"]},
                  {"upper": u, "middle": m, "lower": l}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    u, m, l = talib.BBANDS(sdata["close"], timeperiod=period, nbdevup=nbdevup, nbdevdn=nbdevdn, matype=0)
    _write_golden(output_dir, "bbands", params, {"close": sdata["close"]},
                  {"upper": u, "middle": m, "lower": l}, lookback, "boundary_short")


def generate_sar(output_dir: Path, acceleration: float = 0.02, maximum: float = 0.2):
    print(f"\n[SAR] acceleration={acceleration} maximum={maximum}")
    data0 = make_normal_1000()
    r0 = talib.SAR(data0["high"], data0["low"], acceleration=acceleration, maximum=maximum)
    lookback = _compute_lookback(r0)
    params = {"acceleration": acceleration, "maximum": maximum}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.SAR(data["high"], data["low"], acceleration=acceleration, maximum=maximum)
        _write_golden(output_dir, "sar", params,
                      {"high": data["high"], "low": data["low"]},
                      {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.SAR(bdata["high"], bdata["low"], acceleration=acceleration, maximum=maximum)
    _write_golden(output_dir, "sar", params, {"high": bdata["high"], "low": bdata["low"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.SAR(sdata["high"], sdata["low"], acceleration=acceleration, maximum=maximum)
    _write_golden(output_dir, "sar", params, {"high": sdata["high"], "low": sdata["low"]},
                  {"values": result}, lookback, "boundary_short")


def generate_adx(output_dir: Path, period: int = 14):
    print(f"\n[ADX] period={period}")
    data0 = make_normal_1000()
    r0 = talib.ADX(data0["high"], data0["low"], data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.ADX(data["high"], data["low"], data["close"], timeperiod=period)
        _write_golden(output_dir, "adx", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.ADX(bdata["high"], bdata["low"], bdata["close"], timeperiod=period)
    _write_golden(output_dir, "adx", params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.ADX(sdata["high"], sdata["low"], sdata["close"], timeperiod=period)
    _write_golden(output_dir, "adx", params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"]},
                  {"values": result}, lookback, "boundary_short")


def generate_rsi(output_dir: Path, period: int = 14):
    print(f"\n[RSI] period={period}")
    data0 = make_normal_1000()
    r0 = talib.RSI(data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.RSI(data["close"], timeperiod=period)
        _write_golden(output_dir, "rsi", params,
                      {"close": data["close"]}, {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.RSI(bdata["close"], timeperiod=period)
    _write_golden(output_dir, "rsi", params, {"close": bdata["close"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.RSI(sdata["close"], timeperiod=period)
    _write_golden(output_dir, "rsi", params, {"close": sdata["close"]},
                  {"values": result}, lookback, "boundary_short")


def generate_stoch(output_dir: Path, fastk_period: int = 5, slowk_period: int = 3, slowd_period: int = 3):
    print(f"\n[Stoch] fastk={fastk_period} slowk={slowk_period} slowd={slowd_period}")
    data0 = make_normal_1000()
    sk0, sd0 = talib.STOCH(data0["high"], data0["low"], data0["close"],
                            fastk_period=fastk_period, slowk_period=slowk_period,
                            slowk_matype=0, slowd_period=slowd_period, slowd_matype=0)
    lookback = _compute_lookback(sk0)
    params = {"fastk": fastk_period, "slowk": slowk_period, "slowd": slowd_period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        sk, sd = talib.STOCH(data["high"], data["low"], data["close"],
                             fastk_period=fastk_period, slowk_period=slowk_period,
                             slowk_matype=0, slowd_period=slowd_period, slowd_matype=0)
        _write_golden(output_dir, "stoch", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"slowk": sk, "slowd": sd}, lookback, name)

    bdata = make_boundary_exact(lookback)
    sk, sd = talib.STOCH(bdata["high"], bdata["low"], bdata["close"],
                         fastk_period=fastk_period, slowk_period=slowk_period,
                         slowk_matype=0, slowd_period=slowd_period, slowd_matype=0)
    _write_golden(output_dir, "stoch", params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"]},
                  {"slowk": sk, "slowd": sd}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    sk, sd = talib.STOCH(sdata["high"], sdata["low"], sdata["close"],
                         fastk_period=fastk_period, slowk_period=slowk_period,
                         slowk_matype=0, slowd_period=slowd_period, slowd_matype=0)
    _write_golden(output_dir, "stoch", params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"]},
                  {"slowk": sk, "slowd": sd}, lookback, "boundary_short")


def generate_stochrsi(output_dir: Path, period: int = 14, fastk_period: int = 5, fastd_period: int = 3):
    print(f"\n[StochRSI] period={period} fastk={fastk_period} fastd={fastd_period}")
    data0 = make_normal_1000()
    fk0, fd0 = talib.STOCHRSI(data0["close"], timeperiod=period,
                               fastk_period=fastk_period, fastd_period=fastd_period, fastd_matype=0)
    lookback = _compute_lookback(fk0)
    params = {"period": period, "fastk": fastk_period, "fastd": fastd_period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        fk, fd = talib.STOCHRSI(data["close"], timeperiod=period,
                                 fastk_period=fastk_period, fastd_period=fastd_period, fastd_matype=0)
        _write_golden(output_dir, "stochrsi", params,
                      {"close": data["close"]}, {"fastk": fk, "fastd": fd}, lookback, name)

    bdata = make_boundary_exact(lookback)
    fk, fd = talib.STOCHRSI(bdata["close"], timeperiod=period,
                             fastk_period=fastk_period, fastd_period=fastd_period, fastd_matype=0)
    _write_golden(output_dir, "stochrsi", params, {"close": bdata["close"]},
                  {"fastk": fk, "fastd": fd}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    fk, fd = talib.STOCHRSI(sdata["close"], timeperiod=period,
                             fastk_period=fastk_period, fastd_period=fastd_period, fastd_matype=0)
    _write_golden(output_dir, "stochrsi", params, {"close": sdata["close"]},
                  {"fastk": fk, "fastd": fd}, lookback, "boundary_short")


def generate_cci(output_dir: Path, period: int = 20):
    print(f"\n[CCI] period={period}")
    data0 = make_normal_1000()
    r0 = talib.CCI(data0["high"], data0["low"], data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.CCI(data["high"], data["low"], data["close"], timeperiod=period)
        _write_golden(output_dir, "cci", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.CCI(bdata["high"], bdata["low"], bdata["close"], timeperiod=period)
    _write_golden(output_dir, "cci", params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.CCI(sdata["high"], sdata["low"], sdata["close"], timeperiod=period)
    _write_golden(output_dir, "cci", params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"]},
                  {"values": result}, lookback, "boundary_short")


def generate_willr(output_dir: Path, period: int = 14):
    print(f"\n[WillR] period={period}")
    data0 = make_normal_1000()
    r0 = talib.WILLR(data0["high"], data0["low"], data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.WILLR(data["high"], data["low"], data["close"], timeperiod=period)
        _write_golden(output_dir, "willr", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.WILLR(bdata["high"], bdata["low"], bdata["close"], timeperiod=period)
    _write_golden(output_dir, "willr", params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.WILLR(sdata["high"], sdata["low"], sdata["close"], timeperiod=period)
    _write_golden(output_dir, "willr", params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"]},
                  {"values": result}, lookback, "boundary_short")


def generate_ultosc(output_dir: Path, period1: int = 7, period2: int = 14, period3: int = 28):
    print(f"\n[ULTOSC] period1={period1} period2={period2} period3={period3}")
    data0 = make_normal_1000()
    r0 = talib.ULTOSC(data0["high"], data0["low"], data0["close"],
                       timeperiod1=period1, timeperiod2=period2, timeperiod3=period3)
    lookback = _compute_lookback(r0)
    params = {"period1": period1, "period2": period2, "period3": period3}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.ULTOSC(data["high"], data["low"], data["close"],
                               timeperiod1=period1, timeperiod2=period2, timeperiod3=period3)
        _write_golden(output_dir, "ultosc", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.ULTOSC(bdata["high"], bdata["low"], bdata["close"],
                           timeperiod1=period1, timeperiod2=period2, timeperiod3=period3)
    _write_golden(output_dir, "ultosc", params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.ULTOSC(sdata["high"], sdata["low"], sdata["close"],
                           timeperiod1=period1, timeperiod2=period2, timeperiod3=period3)
    _write_golden(output_dir, "ultosc", params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"]},
                  {"values": result}, lookback, "boundary_short")


def generate_aroon(output_dir: Path, period: int = 14):
    print(f"\n[Aroon] period={period}")
    data0 = make_normal_1000()
    ad0, au0 = talib.AROON(data0["high"], data0["low"], timeperiod=period)
    lookback = _compute_lookback(ad0)
    params = {"period": period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        aroon_down, aroon_up = talib.AROON(data["high"], data["low"], timeperiod=period)
        _write_golden(output_dir, "aroon", params,
                      {"high": data["high"], "low": data["low"]},
                      {"aroon_down": aroon_down, "aroon_up": aroon_up}, lookback, name)

    bdata = make_boundary_exact(lookback)
    aroon_down, aroon_up = talib.AROON(bdata["high"], bdata["low"], timeperiod=period)
    _write_golden(output_dir, "aroon", params,
                  {"high": bdata["high"], "low": bdata["low"]},
                  {"aroon_down": aroon_down, "aroon_up": aroon_up}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    aroon_down, aroon_up = talib.AROON(sdata["high"], sdata["low"], timeperiod=period)
    _write_golden(output_dir, "aroon", params,
                  {"high": sdata["high"], "low": sdata["low"]},
                  {"aroon_down": aroon_down, "aroon_up": aroon_up}, lookback, "boundary_short")


def generate_mfi(output_dir: Path, period: int = 14):
    print(f"\n[MFI] period={period}")
    data0 = make_normal_1000()
    r0 = talib.MFI(data0["high"], data0["low"], data0["close"], data0["volume"], timeperiod=period)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.MFI(data["high"], data["low"], data["close"], data["volume"], timeperiod=period)
        _write_golden(output_dir, "mfi", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"], "volume": data["volume"]},
                      {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.MFI(bdata["high"], bdata["low"], bdata["close"], bdata["volume"], timeperiod=period)
    _write_golden(output_dir, "mfi", params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"], "volume": bdata["volume"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.MFI(sdata["high"], sdata["low"], sdata["close"], sdata["volume"], timeperiod=period)
    _write_golden(output_dir, "mfi", params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"], "volume": sdata["volume"]},
                  {"values": result}, lookback, "boundary_short")


def generate_obv(output_dir: Path):
    """OBV has lookback=0, so no boundary tests."""
    print(f"\n[OBV]")
    lookback = 0
    params = {}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.OBV(data["close"], data["volume"])
        _write_golden(output_dir, "obv", params,
                      {"close": data["close"], "volume": data["volume"]},
                      {"values": result}, lookback, name)


def generate_ad(output_dir: Path):
    """AD has lookback=0, so no boundary tests."""
    print(f"\n[AD]")
    lookback = 0
    params = {}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.AD(data["high"], data["low"], data["close"], data["volume"])
        _write_golden(output_dir, "ad", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"], "volume": data["volume"]},
                      {"values": result}, lookback, name)


def generate_adosc(output_dir: Path, fast_period: int = 3, slow_period: int = 10):
    print(f"\n[ADOSC] fast={fast_period} slow={slow_period}")
    data0 = make_normal_1000()
    r0 = talib.ADOSC(data0["high"], data0["low"], data0["close"], data0["volume"],
                      fastperiod=fast_period, slowperiod=slow_period)
    lookback = _compute_lookback(r0)
    params = {"fast": fast_period, "slow": slow_period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.ADOSC(data["high"], data["low"], data["close"], data["volume"],
                              fastperiod=fast_period, slowperiod=slow_period)
        _write_golden(output_dir, "adosc", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"], "volume": data["volume"]},
                      {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.ADOSC(bdata["high"], bdata["low"], bdata["close"], bdata["volume"],
                          fastperiod=fast_period, slowperiod=slow_period)
    _write_golden(output_dir, "adosc", params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"], "volume": bdata["volume"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.ADOSC(sdata["high"], sdata["low"], sdata["close"], sdata["volume"],
                          fastperiod=fast_period, slowperiod=slow_period)
    _write_golden(output_dir, "adosc", params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"], "volume": sdata["volume"]},
                  {"values": result}, lookback, "boundary_short")


def generate_trange(output_dir: Path):
    print(f"\n[TRange]")
    lookback = 1
    params = {}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.TRANGE(data["high"], data["low"], data["close"])
        _write_golden(output_dir, "trange", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.TRANGE(bdata["high"], bdata["low"], bdata["close"])
    _write_golden(output_dir, "trange", params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.TRANGE(sdata["high"], sdata["low"], sdata["close"])
    _write_golden(output_dir, "trange", params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"]},
                  {"values": result}, lookback, "boundary_short")


def generate_atr(output_dir: Path, period: int = 14):
    print(f"\n[ATR] period={period}")
    data0 = make_normal_1000()
    r0 = talib.ATR(data0["high"], data0["low"], data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.ATR(data["high"], data["low"], data["close"], timeperiod=period)
        _write_golden(output_dir, "atr", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.ATR(bdata["high"], bdata["low"], bdata["close"], timeperiod=period)
    _write_golden(output_dir, "atr", params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.ATR(sdata["high"], sdata["low"], sdata["close"], timeperiod=period)
    _write_golden(output_dir, "atr", params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"]},
                  {"values": result}, lookback, "boundary_short")


def generate_natr(output_dir: Path, period: int = 14):
    print(f"\n[NATR] period={period}")
    data0 = make_normal_1000()
    r0 = talib.NATR(data0["high"], data0["low"], data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.NATR(data["high"], data["low"], data["close"], timeperiod=period)
        _write_golden(output_dir, "natr", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.NATR(bdata["high"], bdata["low"], bdata["close"], timeperiod=period)
    _write_golden(output_dir, "natr", params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.NATR(sdata["high"], sdata["low"], sdata["close"], timeperiod=period)
    _write_golden(output_dir, "natr", params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"]},
                  {"values": result}, lookback, "boundary_short")


def generate_trima(output_dir: Path, period: int = 14):
    print(f"\n[TRIMA] period={period}")
    data0 = make_normal_1000()
    r0 = talib.TRIMA(data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.TRIMA(data["close"], timeperiod=period)
        _write_golden(output_dir, "trima", {"period": period},
                      {"close": data["close"]}, {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.TRIMA(bdata["close"], timeperiod=period)
    _write_golden(output_dir, "trima", {"period": period},
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.TRIMA(sdata["close"], timeperiod=period)
    _write_golden(output_dir, "trima", {"period": period},
                  {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_t3(output_dir: Path, period: int = 5, vfactor: float = 0.7):
    print(f"\n[T3] period={period}, vfactor={vfactor}")
    data0 = make_normal_1000()
    r0 = talib.T3(data0["close"], timeperiod=period, vfactor=vfactor)
    lookback = _compute_lookback(r0)

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.T3(data["close"], timeperiod=period, vfactor=vfactor)
        _write_golden(output_dir, "t3", {"period": period, "vfactor": vfactor},
                      {"close": data["close"]}, {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.T3(bdata["close"], timeperiod=period, vfactor=vfactor)
    _write_golden(output_dir, "t3", {"period": period, "vfactor": vfactor},
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.T3(sdata["close"], timeperiod=period, vfactor=vfactor)
    _write_golden(output_dir, "t3", {"period": period, "vfactor": vfactor},
                  {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_kama(output_dir: Path, period: int = 10):
    print(f"\n[KAMA] period={period}")
    data0 = make_normal_1000()
    r0 = talib.KAMA(data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.KAMA(data["close"], timeperiod=period)
        _write_golden(output_dir, "kama", {"period": period},
                      {"close": data["close"]}, {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.KAMA(bdata["close"], timeperiod=period)
    _write_golden(output_dir, "kama", {"period": period},
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.KAMA(sdata["close"], timeperiod=period)
    _write_golden(output_dir, "kama", {"period": period},
                  {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_midpoint(output_dir: Path, period: int = 14):
    print(f"\n[MIDPOINT] period={period}")
    lookback = period - 1

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.MIDPOINT(data["close"], timeperiod=period)
        _write_golden(output_dir, "midpoint", {"period": period},
                      {"close": data["close"]}, {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.MIDPOINT(bdata["close"], timeperiod=period)
    _write_golden(output_dir, "midpoint", {"period": period},
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.MIDPOINT(sdata["close"], timeperiod=period)
    _write_golden(output_dir, "midpoint", {"period": period},
                  {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_midprice(output_dir: Path, period: int = 14):
    print(f"\n[MIDPRICE] period={period}")
    lookback = period - 1

    for name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.MIDPRICE(data["high"], data["low"], timeperiod=period)
        _write_golden(output_dir, "midprice", {"period": period},
                      {"high": data["high"], "low": data["low"]}, {"values": result}, lookback, name)

    bdata = make_boundary_exact(lookback)
    result = talib.MIDPRICE(bdata["high"], bdata["low"], timeperiod=period)
    _write_golden(output_dir, "midprice", {"period": period},
                  {"high": bdata["high"], "low": bdata["low"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.MIDPRICE(sdata["high"], sdata["low"], timeperiod=period)
    _write_golden(output_dir, "midprice", {"period": period},
                  {"high": sdata["high"], "low": sdata["low"]}, {"values": result}, lookback, "boundary_short")


# ─── 主入口 ───────────────────────────────────────────────────────────────────


# ─── Phase 2: Oscillators ────────────────────────────────────────────────────

def _gen_single_close(name: str, talib_fn, output_dir: Path, period: int, talib_kwargs: dict = None):
    """通用辅助：单 close 输入, 有 period 参数的指标."""
    kwargs = talib_kwargs or {"timeperiod": period}
    print(f"\n[{name.upper()}] {kwargs}")
    data0 = make_normal_1000()
    r0 = talib_fn(data0["close"], **kwargs)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib_fn(data["close"], **kwargs)
        _write_golden(output_dir, name, params, {"close": data["close"]}, {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = talib_fn(bdata["close"], **kwargs)
    _write_golden(output_dir, name, params, {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib_fn(sdata["close"], **kwargs)
    _write_golden(output_dir, name, params, {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def _gen_hilo_no_close(name: str, talib_fn, output_dir: Path, period: int):
    """通用辅助：high+low 输入 (无 close), 有 period 参数."""
    print(f"\n[{name.upper()}] period={period}")
    data0 = make_normal_1000()
    r0 = talib_fn(data0["high"], data0["low"], timeperiod=period)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib_fn(data["high"], data["low"], timeperiod=period)
        _write_golden(output_dir, name, params,
                      {"high": data["high"], "low": data["low"]},
                      {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = talib_fn(bdata["high"], bdata["low"], timeperiod=period)
    _write_golden(output_dir, name, params,
                  {"high": bdata["high"], "low": bdata["low"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib_fn(sdata["high"], sdata["low"], timeperiod=period)
    _write_golden(output_dir, name, params,
                  {"high": sdata["high"], "low": sdata["low"]},
                  {"values": result}, lookback, "boundary_short")


def _gen_hlc(name: str, talib_fn, output_dir: Path, period: int):
    """通用辅助：high+low+close 输入, 有 period 参数."""
    print(f"\n[{name.upper()}] period={period}")
    data0 = make_normal_1000()
    r0 = talib_fn(data0["high"], data0["low"], data0["close"], timeperiod=period)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib_fn(data["high"], data["low"], data["close"], timeperiod=period)
        _write_golden(output_dir, name, params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = talib_fn(bdata["high"], bdata["low"], bdata["close"], timeperiod=period)
    _write_golden(output_dir, name, params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib_fn(sdata["high"], sdata["low"], sdata["close"], timeperiod=period)
    _write_golden(output_dir, name, params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"]},
                  {"values": result}, lookback, "boundary_short")


def generate_mom(output_dir: Path, period: int = 10):
    _gen_single_close("mom", talib.MOM, output_dir, period)


def generate_roc(output_dir: Path, period: int = 10):
    _gen_single_close("roc", talib.ROC, output_dir, period)


def generate_rocp(output_dir: Path, period: int = 10):
    _gen_single_close("rocp", talib.ROCP, output_dir, period)


def generate_rocr(output_dir: Path, period: int = 10):
    _gen_single_close("rocr", talib.ROCR, output_dir, period)


def generate_rocr100(output_dir: Path, period: int = 10):
    _gen_single_close("rocr100", talib.ROCR100, output_dir, period)


def generate_cmo(output_dir: Path, period: int = 14):
    _gen_single_close("cmo", talib.CMO, output_dir, period)


def generate_apo(output_dir: Path, fast_period: int = 12, slow_period: int = 26):
    print(f"\n[APO] fast={fast_period} slow={slow_period}")
    data0 = make_normal_1000()
    r0 = talib.APO(data0["close"], fastperiod=fast_period, slowperiod=slow_period, matype=0)
    lookback = _compute_lookback(r0)
    params = {"fast": fast_period, "slow": slow_period}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.APO(data["close"], fastperiod=fast_period, slowperiod=slow_period, matype=0)
        _write_golden(output_dir, "apo", params, {"close": data["close"]}, {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = talib.APO(bdata["close"], fastperiod=fast_period, slowperiod=slow_period, matype=0)
    _write_golden(output_dir, "apo", params, {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.APO(sdata["close"], fastperiod=fast_period, slowperiod=slow_period, matype=0)
    _write_golden(output_dir, "apo", params, {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_ppo(output_dir: Path, fast_period: int = 12, slow_period: int = 26):
    print(f"\n[PPO] fast={fast_period} slow={slow_period}")
    data0 = make_normal_1000()
    r0 = talib.PPO(data0["close"], fastperiod=fast_period, slowperiod=slow_period, matype=0)
    lookback = _compute_lookback(r0)
    params = {"fast": fast_period, "slow": slow_period}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.PPO(data["close"], fastperiod=fast_period, slowperiod=slow_period, matype=0)
        _write_golden(output_dir, "ppo", params, {"close": data["close"]}, {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = talib.PPO(bdata["close"], fastperiod=fast_period, slowperiod=slow_period, matype=0)
    _write_golden(output_dir, "ppo", params, {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.PPO(sdata["close"], fastperiod=fast_period, slowperiod=slow_period, matype=0)
    _write_golden(output_dir, "ppo", params, {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_trix(output_dir: Path, period: int = 5):
    _gen_single_close("trix", talib.TRIX, output_dir, period)


def generate_bop(output_dir: Path):
    print(f"\n[BOP]")
    lookback = 0
    params = {}
    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.BOP(data["open"], data["high"], data["low"], data["close"])
        _write_golden(output_dir, "bop", params,
                      {"open": data["open"], "high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, ds_name)


def generate_minus_dm(output_dir: Path, period: int = 14):
    _gen_hilo_no_close("minus_dm", talib.MINUS_DM, output_dir, period)


def generate_plus_dm(output_dir: Path, period: int = 14):
    _gen_hilo_no_close("plus_dm", talib.PLUS_DM, output_dir, period)


def generate_minus_di(output_dir: Path, period: int = 14):
    _gen_hlc("minus_di", talib.MINUS_DI, output_dir, period)


def generate_plus_di(output_dir: Path, period: int = 14):
    _gen_hlc("plus_di", talib.PLUS_DI, output_dir, period)


def generate_dx(output_dir: Path, period: int = 14):
    _gen_hlc("dx", talib.DX, output_dir, period)


def generate_adxr(output_dir: Path, period: int = 14):
    _gen_hlc("adxr", talib.ADXR, output_dir, period)


# ─── Phase 2: Statistics ─────────────────────────────────────────────────────

def _gen_two_real(name: str, talib_fn, output_dir: Path, period: int):
    """通用辅助：real0+real1 两输入, 有 period 参数."""
    print(f"\n[{name.upper()}] period={period}")
    data0 = make_normal_1000()
    # 用 close 作 real0，high 作 real1（确定性配对）
    r0 = talib_fn(data0["close"], data0["high"], timeperiod=period)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib_fn(data["close"], data["high"], timeperiod=period)
        _write_golden(output_dir, name, params,
                      {"real0": data["close"], "real1": data["high"]},
                      {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = talib_fn(bdata["close"], bdata["high"], timeperiod=period)
    _write_golden(output_dir, name, params,
                  {"real0": bdata["close"], "real1": bdata["high"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib_fn(sdata["close"], sdata["high"], timeperiod=period)
    _write_golden(output_dir, name, params,
                  {"real0": sdata["close"], "real1": sdata["high"]},
                  {"values": result}, lookback, "boundary_short")


def generate_beta(output_dir: Path, period: int = 5):
    _gen_two_real("beta", talib.BETA, output_dir, period)


def generate_correl(output_dir: Path, period: int = 30):
    _gen_two_real("correl", talib.CORREL, output_dir, period)


def generate_linearreg(output_dir: Path, period: int = 14):
    _gen_single_close("linearreg", talib.LINEARREG, output_dir, period)


def generate_linearreg_angle(output_dir: Path, period: int = 14):
    _gen_single_close("linearreg_angle", talib.LINEARREG_ANGLE, output_dir, period)


def generate_linearreg_intercept(output_dir: Path, period: int = 14):
    _gen_single_close("linearreg_intercept", talib.LINEARREG_INTERCEPT, output_dir, period)


def generate_linearreg_slope(output_dir: Path, period: int = 14):
    _gen_single_close("linearreg_slope", talib.LINEARREG_SLOPE, output_dir, period)


def generate_stddev(output_dir: Path, period: int = 5, nbdev: float = 1.0):
    print(f"\n[STDDEV] period={period} nbdev={nbdev}")
    data0 = make_normal_1000()
    r0 = talib.STDDEV(data0["close"], timeperiod=period, nbdev=nbdev)
    lookback = _compute_lookback(r0)
    params = {"period": period, "nbdev": nbdev}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.STDDEV(data["close"], timeperiod=period, nbdev=nbdev)
        _write_golden(output_dir, "stddev", params, {"close": data["close"]}, {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = talib.STDDEV(bdata["close"], timeperiod=period, nbdev=nbdev)
    _write_golden(output_dir, "stddev", params, {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.STDDEV(sdata["close"], timeperiod=period, nbdev=nbdev)
    _write_golden(output_dir, "stddev", params, {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_tsf(output_dir: Path, period: int = 14):
    _gen_single_close("tsf", talib.TSF, output_dir, period)


def generate_var(output_dir: Path, period: int = 5, nbdev: float = 1.0):
    print(f"\n[VAR] period={period} nbdev={nbdev}")
    data0 = make_normal_1000()
    r0 = talib.VAR(data0["close"], timeperiod=period, nbdev=nbdev)
    lookback = _compute_lookback(r0)
    params = {"period": period, "nbdev": nbdev}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.VAR(data["close"], timeperiod=period, nbdev=nbdev)
        _write_golden(output_dir, "var", params, {"close": data["close"]}, {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = talib.VAR(bdata["close"], timeperiod=period, nbdev=nbdev)
    _write_golden(output_dir, "var", params, {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.VAR(sdata["close"], timeperiod=period, nbdev=nbdev)
    _write_golden(output_dir, "var", params, {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


# ─── Phase 3: Price Transform ────────────────────────────────────────────────

def generate_avgprice(output_dir: Path):
    print(f"\n[AVGPRICE]")
    lookback = 0
    params = {}
    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.AVGPRICE(data["open"], data["high"], data["low"], data["close"])
        _write_golden(output_dir, "avgprice", params,
                      {"open": data["open"], "high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, ds_name)


def generate_medprice(output_dir: Path):
    print(f"\n[MEDPRICE]")
    lookback = 0
    params = {}
    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.MEDPRICE(data["high"], data["low"])
        _write_golden(output_dir, "medprice", params,
                      {"high": data["high"], "low": data["low"]},
                      {"values": result}, lookback, ds_name)


def generate_typprice(output_dir: Path):
    print(f"\n[TYPPRICE]")
    lookback = 0
    params = {}
    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.TYPPRICE(data["high"], data["low"], data["close"])
        _write_golden(output_dir, "typprice", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, ds_name)


def generate_wclprice(output_dir: Path):
    print(f"\n[WCLPRICE]")
    lookback = 0
    params = {}
    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.WCLPRICE(data["high"], data["low"], data["close"])
        _write_golden(output_dir, "wclprice", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"values": result}, lookback, ds_name)


# ─── Phase 3: Oscillator / Trend additions ────────────────────────────────────

def generate_aroonosc(output_dir: Path, period: int = 14):
    print(f"\n[AROONOSC] period={period}")
    data0 = make_normal_1000()
    r0 = talib.AROONOSC(data0["high"], data0["low"], timeperiod=period)
    lookback = _compute_lookback(r0)
    params = {"period": period}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.AROONOSC(data["high"], data["low"], timeperiod=period)
        _write_golden(output_dir, "aroonosc", params,
                      {"high": data["high"], "low": data["low"]},
                      {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = talib.AROONOSC(bdata["high"], bdata["low"], timeperiod=period)
    _write_golden(output_dir, "aroonosc", params,
                  {"high": bdata["high"], "low": bdata["low"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.AROONOSC(sdata["high"], sdata["low"], timeperiod=period)
    _write_golden(output_dir, "aroonosc", params,
                  {"high": sdata["high"], "low": sdata["low"]},
                  {"values": result}, lookback, "boundary_short")


def generate_stochf(output_dir: Path, fastk_period: int = 5, fastd_period: int = 3):
    print(f"\n[STOCHF] fastk={fastk_period} fastd={fastd_period}")
    data0 = make_normal_1000()
    fk0, fd0 = talib.STOCHF(data0["high"], data0["low"], data0["close"],
                              fastk_period=fastk_period, fastd_period=fastd_period, fastd_matype=0)
    lookback = _compute_lookback(fd0)
    params = {"fastk": fastk_period, "fastd": fastd_period}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        fk, fd = talib.STOCHF(data["high"], data["low"], data["close"],
                               fastk_period=fastk_period, fastd_period=fastd_period, fastd_matype=0)
        _write_golden(output_dir, "stochf", params,
                      {"high": data["high"], "low": data["low"], "close": data["close"]},
                      {"fastk": fk, "fastd": fd}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    fk, fd = talib.STOCHF(bdata["high"], bdata["low"], bdata["close"],
                           fastk_period=fastk_period, fastd_period=fastd_period, fastd_matype=0)
    _write_golden(output_dir, "stochf", params,
                  {"high": bdata["high"], "low": bdata["low"], "close": bdata["close"]},
                  {"fastk": fk, "fastd": fd}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    fk, fd = talib.STOCHF(sdata["high"], sdata["low"], sdata["close"],
                           fastk_period=fastk_period, fastd_period=fastd_period, fastd_matype=0)
    _write_golden(output_dir, "stochf", params,
                  {"high": sdata["high"], "low": sdata["low"], "close": sdata["close"]},
                  {"fastk": fk, "fastd": fd}, lookback, "boundary_short")


def generate_macdext(output_dir: Path, fast_period: int = 12, slow_period: int = 26, signal_period: int = 9):
    print(f"\n[MACDEXT] fast={fast_period} slow={slow_period} signal={signal_period} (all EMA)")
    data0 = make_normal_1000()
    m0, s0, h0 = talib.MACDEXT(data0["close"], fastperiod=fast_period, fastmatype=1,
                                 slowperiod=slow_period, slowmatype=1,
                                 signalperiod=signal_period, signalmatype=1)
    lookback = _compute_lookback(m0)
    params = {"fast": fast_period, "slow": slow_period, "signal": signal_period, "matype": 1}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        m, s, h = talib.MACDEXT(data["close"], fastperiod=fast_period, fastmatype=1,
                                  slowperiod=slow_period, slowmatype=1,
                                  signalperiod=signal_period, signalmatype=1)
        _write_golden(output_dir, "macdext", params,
                      {"close": data["close"]},
                      {"macd": m, "signal": s, "hist": h}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    m, s, h = talib.MACDEXT(bdata["close"], fastperiod=fast_period, fastmatype=1,
                              slowperiod=slow_period, slowmatype=1,
                              signalperiod=signal_period, signalmatype=1)
    _write_golden(output_dir, "macdext", params,
                  {"close": bdata["close"]},
                  {"macd": m, "signal": s, "hist": h}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    m, s, h = talib.MACDEXT(sdata["close"], fastperiod=fast_period, fastmatype=1,
                              slowperiod=slow_period, slowmatype=1,
                              signalperiod=signal_period, signalmatype=1)
    _write_golden(output_dir, "macdext", params,
                  {"close": sdata["close"]},
                  {"macd": m, "signal": s, "hist": h}, lookback, "boundary_short")


def generate_macdfix(output_dir: Path, signal_period: int = 9):
    print(f"\n[MACDFIX] signal={signal_period}")
    data0 = make_normal_1000()
    m0, s0, h0 = talib.MACDFIX(data0["close"], signalperiod=signal_period)
    lookback = _compute_lookback(m0)
    params = {"signal": signal_period}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        m, s, h = talib.MACDFIX(data["close"], signalperiod=signal_period)
        _write_golden(output_dir, "macdfix", params,
                      {"close": data["close"]},
                      {"macd": m, "signal": s, "hist": h}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    m, s, h = talib.MACDFIX(bdata["close"], signalperiod=signal_period)
    _write_golden(output_dir, "macdfix", params,
                  {"close": bdata["close"]},
                  {"macd": m, "signal": s, "hist": h}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    m, s, h = talib.MACDFIX(sdata["close"], signalperiod=signal_period)
    _write_golden(output_dir, "macdfix", params,
                  {"close": sdata["close"]},
                  {"macd": m, "signal": s, "hist": h}, lookback, "boundary_short")


def generate_ma(output_dir: Path, period: int = 30, matype: int = 1):
    print(f"\n[MA] period={period} matype={matype}")
    data0 = make_normal_1000()
    r0 = talib.MA(data0["close"], timeperiod=period, matype=matype)
    lookback = _compute_lookback(r0)
    params = {"period": period, "matype": matype}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.MA(data["close"], timeperiod=period, matype=matype)
        _write_golden(output_dir, "ma", params,
                      {"close": data["close"]}, {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = talib.MA(bdata["close"], timeperiod=period, matype=matype)
    _write_golden(output_dir, "ma", params,
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.MA(sdata["close"], timeperiod=period, matype=matype)
    _write_golden(output_dir, "ma", params,
                  {"close": sdata["close"]}, {"values": result}, lookback, "boundary_short")


def generate_sarext(output_dir: Path):
    print(f"\n[SAREXT] (default params)")
    data0 = make_normal_1000()
    r0 = talib.SAREXT(data0["high"], data0["low"])
    lookback = _compute_lookback(r0)
    params = {}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib.SAREXT(data["high"], data["low"])
        _write_golden(output_dir, "sarext", params,
                      {"high": data["high"], "low": data["low"]},
                      {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = talib.SAREXT(bdata["high"], bdata["low"])
    _write_golden(output_dir, "sarext", params,
                  {"high": bdata["high"], "low": bdata["low"]},
                  {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    result = talib.SAREXT(sdata["high"], sdata["low"])
    _write_golden(output_dir, "sarext", params,
                  {"high": sdata["high"], "low": sdata["low"]},
                  {"values": result}, lookback, "boundary_short")


# ─── Phase 3: Math Transform ─────────────────────────────────────────────────

def _gen_math_transform(name: str, talib_fn, output_dir: Path, normalize: bool = False):
    """通用辅助：单输入 close，无参数，lookback=0."""
    print(f"\n[{name.upper()}]")
    lookback = 0
    params = {}
    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        close = data["close"]
        if normalize:
            # 归一化到 [-1,1] 范围，适用于 ACOS/ASIN
            max_val = np.nanmax(np.abs(close))
            if max_val > 0:
                close = close / max_val
        result = talib_fn(close)
        _write_golden(output_dir, name, params,
                      {"close": close},
                      {"values": result}, lookback, ds_name)


def generate_acos(output_dir: Path):
    _gen_math_transform("acos", talib.ACOS, output_dir, normalize=True)


def generate_asin(output_dir: Path):
    _gen_math_transform("asin", talib.ASIN, output_dir, normalize=True)


def generate_atan(output_dir: Path):
    _gen_math_transform("atan", talib.ATAN, output_dir)


def generate_ceil(output_dir: Path):
    _gen_math_transform("ceil", talib.CEIL, output_dir)


def generate_cos(output_dir: Path):
    _gen_math_transform("cos", talib.COS, output_dir)


def generate_cosh(output_dir: Path):
    _gen_math_transform("cosh", talib.COSH, output_dir)


def generate_exp(output_dir: Path):
    _gen_math_transform("exp", talib.EXP, output_dir)


def generate_floor(output_dir: Path):
    _gen_math_transform("floor", talib.FLOOR, output_dir)


def generate_ln(output_dir: Path):
    _gen_math_transform("ln", talib.LN, output_dir)


def generate_log10(output_dir: Path):
    _gen_math_transform("log10", talib.LOG10, output_dir)


def generate_sin(output_dir: Path):
    _gen_math_transform("sin", talib.SIN, output_dir)


def generate_sinh(output_dir: Path):
    _gen_math_transform("sinh", talib.SINH, output_dir)


def generate_sqrt(output_dir: Path):
    _gen_math_transform("sqrt", talib.SQRT, output_dir)


def generate_tan(output_dir: Path):
    _gen_math_transform("tan", talib.TAN, output_dir)


def generate_tanh(output_dir: Path):
    _gen_math_transform("tanh", talib.TANH, output_dir)


# --- Phase 3: Math Ops -------------------------------------------------------

def _gen_two_real_no_period(name: str, talib_fn, output_dir: Path):
    """Two-input (real0=close, real1=high), no period, lookback=0."""
    print(f"\n[{name.upper()}]")
    lookback = 0
    params = {}
    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = talib_fn(data["close"], data["high"])
        _write_golden(output_dir, name, params,
                      {"real0": data["close"], "real1": data["high"]},
                      {"values": result}, lookback, ds_name)


def generate_add(output_dir: Path):
    _gen_two_real_no_period("add", talib.ADD, output_dir)


def generate_div(output_dir: Path):
    _gen_two_real_no_period("div", talib.DIV, output_dir)


def generate_mult(output_dir: Path):
    _gen_two_real_no_period("mult", talib.MULT, output_dir)


def generate_sub(output_dir: Path):
    _gen_two_real_no_period("sub", talib.SUB, output_dir)


def generate_max(output_dir: Path, period: int = 30):
    _gen_single_close("max", lambda c, **kw: talib.MAX(c, **kw), output_dir, period)


def generate_min(output_dir: Path, period: int = 30):
    _gen_single_close("min", lambda c, **kw: talib.MIN(c, **kw), output_dir, period)


def generate_sum(output_dir: Path, period: int = 30):
    _gen_single_close("sum", lambda c, **kw: talib.SUM(c, **kw), output_dir, period)


def generate_maxindex(output_dir: Path, period: int = 30):
    """MAXINDEX returns int array with leading zeros (not NaN); handle specially."""
    print(f"\n[MAXINDEX] period={period}")
    lookback = period - 1  # ta-lib MAXINDEX has int output, no NaN; lookback is period-1
    params = {"period": period}

    def _run(data):
        raw = talib.MAXINDEX(data["close"], timeperiod=period).astype(float)
        # Insert NaN for leading lookback positions so _serialize_array handles them
        result = np.empty(len(raw))
        result[:lookback] = np.nan
        result[lookback:] = raw[lookback:]
        return result

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = _run(data)
        _write_golden(output_dir, "maxindex", params,
                      {"close": data["close"]}, {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = _run(bdata)
    _write_golden(output_dir, "maxindex", params,
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    _write_golden(output_dir, "maxindex", params,
                  {"close": sdata["close"]},
                  {"values": np.full(len(sdata["close"]), np.nan)}, lookback, "boundary_short")


def generate_minindex(output_dir: Path, period: int = 30):
    """MININDEX returns int array with leading zeros (not NaN); handle specially."""
    print(f"\n[MININDEX] period={period}")
    lookback = period - 1
    params = {"period": period}

    def _run(data):
        raw = talib.MININDEX(data["close"], timeperiod=period).astype(float)
        result = np.empty(len(raw))
        result[:lookback] = np.nan
        result[lookback:] = raw[lookback:]
        return result

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        result = _run(data)
        _write_golden(output_dir, "minindex", params,
                      {"close": data["close"]}, {"values": result}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    result = _run(bdata)
    _write_golden(output_dir, "minindex", params,
                  {"close": bdata["close"]}, {"values": result}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    _write_golden(output_dir, "minindex", params,
                  {"close": sdata["close"]},
                  {"values": np.full(len(sdata["close"]), np.nan)}, lookback, "boundary_short")


def generate_minmax(output_dir: Path, period: int = 30):
    print(f"\n[MINMAX] period={period}")
    data0 = make_normal_1000()
    mn0, mx0 = talib.MINMAX(data0["close"], timeperiod=period)
    lookback = _compute_lookback(mn0)
    params = {"period": period}

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        mn, mx = talib.MINMAX(data["close"], timeperiod=period)
        _write_golden(output_dir, "minmax", params,
                      {"close": data["close"]},
                      {"min": mn, "max": mx}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    mn, mx = talib.MINMAX(bdata["close"], timeperiod=period)
    _write_golden(output_dir, "minmax", params,
                  {"close": bdata["close"]}, {"min": mn, "max": mx}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    _write_golden(output_dir, "minmax", params,
                  {"close": sdata["close"]},
                  {"min": np.array([]), "max": np.array([])}, lookback, "boundary_short")


def generate_minmaxindex(output_dir: Path, period: int = 30):
    print(f"\n[MINMAXINDEX] period={period}")
    lookback = period - 1  # int output, no NaN; lookback is period-1
    params = {"period": period}

    def _run(data):
        mi_raw, xi_raw = talib.MINMAXINDEX(data["close"], timeperiod=period)
        mi = np.empty(len(mi_raw), dtype=float)
        xi = np.empty(len(xi_raw), dtype=float)
        mi[:lookback] = np.nan
        xi[:lookback] = np.nan
        mi[lookback:] = mi_raw[lookback:].astype(float)
        xi[lookback:] = xi_raw[lookback:].astype(float)
        return mi, xi

    for ds_name, make_fn in DATASETS.items():
        data = make_fn()
        mi, xi = _run(data)
        _write_golden(output_dir, "minmaxindex", params,
                      {"close": data["close"]},
                      {"minidx": mi, "maxidx": xi}, lookback, ds_name)

    bdata = make_boundary_exact(lookback)
    mi, xi = _run(bdata)
    _write_golden(output_dir, "minmaxindex", params,
                  {"close": bdata["close"]}, {"minidx": mi, "maxidx": xi}, lookback, "boundary_exact")

    sdata = make_boundary_short(lookback)
    n = len(sdata["close"])
    _write_golden(output_dir, "minmaxindex", params,
                  {"close": sdata["close"]},
                  {"minidx": np.full(n, np.nan), "maxidx": np.full(n, np.nan)}, lookback, "boundary_short")


GENERATORS = {
    # ── Phase 1 ───────────────────────────────────────────────────────────────
    "sma":      lambda out_dir: generate_sma(out_dir, period=20),
    "ema":      lambda out_dir: generate_ema(out_dir, period=20),
    "wma":      lambda out_dir: generate_wma(out_dir, period=20),
    "dema":     lambda out_dir: generate_dema(out_dir, period=20),
    "tema":     lambda out_dir: generate_tema(out_dir, period=20),
    "macd":     lambda out_dir: generate_macd(out_dir, fast_period=12, slow_period=26, signal_period=9),
    "bbands":   lambda out_dir: generate_bbands(out_dir, period=20, nbdevup=2.0, nbdevdn=2.0),
    "sar":      lambda out_dir: generate_sar(out_dir, acceleration=0.02, maximum=0.2),
    "adx":      lambda out_dir: generate_adx(out_dir, period=14),
    "rsi":      lambda out_dir: generate_rsi(out_dir, period=14),
    "stoch":    lambda out_dir: generate_stoch(out_dir, fastk_period=5, slowk_period=3, slowd_period=3),
    "stochrsi": lambda out_dir: generate_stochrsi(out_dir, period=14, fastk_period=5, fastd_period=3),
    "cci":      lambda out_dir: generate_cci(out_dir, period=20),
    "willr":    lambda out_dir: generate_willr(out_dir, period=14),
    "ultosc":   lambda out_dir: generate_ultosc(out_dir, period1=7, period2=14, period3=28),
    "aroon":    lambda out_dir: generate_aroon(out_dir, period=14),
    "mfi":      lambda out_dir: generate_mfi(out_dir, period=14),
    "obv":      lambda out_dir: generate_obv(out_dir),
    "ad":       lambda out_dir: generate_ad(out_dir),
    "adosc":    lambda out_dir: generate_adosc(out_dir, fast_period=3, slow_period=10),
    "trange":   lambda out_dir: generate_trange(out_dir),
    "atr":      lambda out_dir: generate_atr(out_dir, period=14),
    "natr":     lambda out_dir: generate_natr(out_dir, period=14),
    # ── Phase 2: Trend ────────────────────────────────────────────────────────
    "trima":    lambda out_dir: generate_trima(out_dir, period=14),
    "t3":       lambda out_dir: generate_t3(out_dir, period=5, vfactor=0.7),
    "kama":     lambda out_dir: generate_kama(out_dir, period=10),
    "midpoint": lambda out_dir: generate_midpoint(out_dir, period=14),
    "midprice": lambda out_dir: generate_midprice(out_dir, period=14),
    # ── Phase 2: Oscillators ─────────────────────────────────────────────────
    "mom":           lambda out_dir: generate_mom(out_dir, period=10),
    "roc":           lambda out_dir: generate_roc(out_dir, period=10),
    "rocp":          lambda out_dir: generate_rocp(out_dir, period=10),
    "rocr":          lambda out_dir: generate_rocr(out_dir, period=10),
    "rocr100":       lambda out_dir: generate_rocr100(out_dir, period=10),
    "cmo":           lambda out_dir: generate_cmo(out_dir, period=14),
    "apo":           lambda out_dir: generate_apo(out_dir, fast_period=12, slow_period=26),
    "ppo":           lambda out_dir: generate_ppo(out_dir, fast_period=12, slow_period=26),
    "trix":          lambda out_dir: generate_trix(out_dir, period=5),
    "bop":           lambda out_dir: generate_bop(out_dir),
    "minus_dm":      lambda out_dir: generate_minus_dm(out_dir, period=14),
    "plus_dm":       lambda out_dir: generate_plus_dm(out_dir, period=14),
    "minus_di":      lambda out_dir: generate_minus_di(out_dir, period=14),
    "plus_di":       lambda out_dir: generate_plus_di(out_dir, period=14),
    "dx":            lambda out_dir: generate_dx(out_dir, period=14),
    "adxr":          lambda out_dir: generate_adxr(out_dir, period=14),
    # ── Phase 2: Statistics ───────────────────────────────────────────────────
    "beta":                lambda out_dir: generate_beta(out_dir, period=5),
    "correl":              lambda out_dir: generate_correl(out_dir, period=30),
    "linearreg":           lambda out_dir: generate_linearreg(out_dir, period=14),
    "linearreg_angle":     lambda out_dir: generate_linearreg_angle(out_dir, period=14),
    "linearreg_intercept": lambda out_dir: generate_linearreg_intercept(out_dir, period=14),
    "linearreg_slope":     lambda out_dir: generate_linearreg_slope(out_dir, period=14),
    "stddev":              lambda out_dir: generate_stddev(out_dir, period=5, nbdev=1.0),
    "tsf":                 lambda out_dir: generate_tsf(out_dir, period=14),
    "var":                 lambda out_dir: generate_var(out_dir, period=5, nbdev=1.0),
    # ── Phase 3: Price Transform ──────────────────────────────────────────────
    "avgprice":  lambda out_dir: generate_avgprice(out_dir),
    "medprice":  lambda out_dir: generate_medprice(out_dir),
    "typprice":  lambda out_dir: generate_typprice(out_dir),
    "wclprice":  lambda out_dir: generate_wclprice(out_dir),
    # ── Phase 3: Math Transform ───────────────────────────────────────────────
    "acos":      lambda out_dir: generate_acos(out_dir),
    "asin":      lambda out_dir: generate_asin(out_dir),
    "atan":      lambda out_dir: generate_atan(out_dir),
    "ceil":      lambda out_dir: generate_ceil(out_dir),
    "cos":       lambda out_dir: generate_cos(out_dir),
    "cosh":      lambda out_dir: generate_cosh(out_dir),
    "exp":       lambda out_dir: generate_exp(out_dir),
    "floor":     lambda out_dir: generate_floor(out_dir),
    "ln":        lambda out_dir: generate_ln(out_dir),
    "log10":     lambda out_dir: generate_log10(out_dir),
    "sin":       lambda out_dir: generate_sin(out_dir),
    "sinh":      lambda out_dir: generate_sinh(out_dir),
    "sqrt":      lambda out_dir: generate_sqrt(out_dir),
    "tan":       lambda out_dir: generate_tan(out_dir),
    "tanh":      lambda out_dir: generate_tanh(out_dir),
    # ── Phase 3: Math Ops ─────────────────────────────────────────────────────
    "add":          lambda out_dir: generate_add(out_dir),
    "div":          lambda out_dir: generate_div(out_dir),
    "mult":         lambda out_dir: generate_mult(out_dir),
    "sub":          lambda out_dir: generate_sub(out_dir),
    "max":          lambda out_dir: generate_max(out_dir, period=30),
    "min":          lambda out_dir: generate_min(out_dir, period=30),
    "sum":          lambda out_dir: generate_sum(out_dir, period=30),
    "maxindex":     lambda out_dir: generate_maxindex(out_dir, period=30),
    "minindex":     lambda out_dir: generate_minindex(out_dir, period=30),
    "minmax":       lambda out_dir: generate_minmax(out_dir, period=30),
    "minmaxindex":  lambda out_dir: generate_minmaxindex(out_dir, period=30),
    # ── Phase 3: Oscillator / Trend additions ─────────────────────────────────
    "aroonosc": lambda out_dir: generate_aroonosc(out_dir, period=14),
    "stochf":   lambda out_dir: generate_stochf(out_dir, fastk_period=5, fastd_period=3),
    "macdext":  lambda out_dir: generate_macdext(out_dir, fast_period=12, slow_period=26, signal_period=9),
    "macdfix":  lambda out_dir: generate_macdfix(out_dir, signal_period=9),
    "ma":       lambda out_dir: generate_ma(out_dir, period=30, matype=1),
    "sarext":   lambda out_dir: generate_sarext(out_dir),
}


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--indicator", default="all", help="Indicator to generate (default: all)")
    parser.add_argument("--output-dir", default="tests/golden", help="Output directory (default: tests/golden)")
    args = parser.parse_args()

    if not TALIB_AVAILABLE:
        print("ERROR: ta-lib is required. Install with: pip install ta-lib")
        sys.exit(1)

    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    if args.indicator == "all":
        for name, gen_fn in GENERATORS.items():
            gen_fn(output_dir)
    elif args.indicator in GENERATORS:
        GENERATORS[args.indicator](output_dir)
    else:
        print(f"ERROR: Unknown indicator '{args.indicator}'. Available: {', '.join(GENERATORS)}")
        sys.exit(1)

    print(f"\nDone. Golden files written to: {output_dir}/")


if __name__ == "__main__":
    main()
