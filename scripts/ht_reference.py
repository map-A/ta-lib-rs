"""
Complete ta-lib HT reference implementation.
Verified against ta-lib Python wrapper.
"""
import math
import talib
import numpy as np

# Load golden test data
import json, os

def load_close():
    gold_dir = "crates/polars-ta-verify/src/golden"
    # use ht_dcperiod.json to get the price data
    with open(f"{gold_dir}/ht_dcperiod.json") as f:
        d = json.load(f)
    # the input is stored in a polars parquet file - find it
    import glob
    parquet_files = glob.glob("tests/**/*.parquet", recursive=True) + glob.glob("crates/**/*.parquet", recursive=True)
    print("parquet files:", parquet_files[:5])
    return None

a = 0.0962
b = 0.5769
SMOOTH_PRICE_SIZE = 50

def do_hilbert(buf_odd, buf_even, prev_odd, prev_even,
               prev_input_odd, prev_input_even,
               hilbert_idx, input_val, is_even, adj):
    """Returns (new_output, new_prev_odd, new_prev_even, 
                new_prev_input_odd, new_prev_input_even)"""
    ht = a * input_val
    if is_even:
        output = -buf_even[hilbert_idx]
        buf_even[hilbert_idx] = ht
        output += ht
        output -= prev_even
        new_prev_even = b * prev_input_even
        output += new_prev_even
        new_prev_input_even = input_val
        output *= adj
        return output, prev_odd, new_prev_even, prev_input_odd, new_prev_input_even
    else:
        output = -buf_odd[hilbert_idx]
        buf_odd[hilbert_idx] = ht
        output += ht
        output -= prev_odd
        new_prev_odd = b * prev_input_odd
        output += new_prev_odd
        new_prev_input_odd = input_val
        output *= adj
        return output, new_prev_odd, prev_even, new_prev_input_odd, prev_input_even

def run_ht_engine(close_arr, warmup_i):
    """
    Run the HT engine. 
    warmup_i=9 → main loop starts at bar 12 (for lookback=32)
    warmup_i=34 → main loop starts at bar 37 (for lookback=63)
    
    Returns list of per-bar state dicts.
    """
    n = len(close_arr)
    rad2deg = 45.0 / math.atan(1.0)   # = 180/pi
    deg2rad = 1.0 / rad2deg             # = pi/180
    const_deg2rad_360 = math.atan(1.0) * 8.0  # = 2*pi

    # WMA init: 3 bars
    trailing_wma_idx = 0
    today = 0
    c0 = close_arr[today]; today += 1
    period_wma_sub = c0; period_wma_sum = c0
    c1 = close_arr[today]; today += 1
    period_wma_sub += c1; period_wma_sum += c1 * 2.0
    c2 = close_arr[today]; today += 1
    period_wma_sub += c2; period_wma_sum += c2 * 3.0
    trailing_wma_value = 0.0

    def do_price_wma(new_price):
        nonlocal period_wma_sub, period_wma_sum, trailing_wma_idx, trailing_wma_value
        period_wma_sub += new_price
        period_wma_sub -= trailing_wma_value
        period_wma_sum += new_price * 4.0
        trailing_wma_value = close_arr[trailing_wma_idx]; trailing_wma_idx += 1
        smoothed = period_wma_sum * 0.1
        period_wma_sum -= period_wma_sub
        return smoothed

    # WMA warmup
    for _ in range(warmup_i):
        do_price_wma(close_arr[today]); today += 1

    # Init HT variables
    hilbert_idx = 0
    det_odd = [0.0]*3; det_even = [0.0]*3
    q1_odd  = [0.0]*3; q1_even  = [0.0]*3
    ji_odd  = [0.0]*3; ji_even  = [0.0]*3
    jq_odd  = [0.0]*3; jq_even  = [0.0]*3
    prev_det_odd = prev_det_even = 0.0
    prev_q1_odd  = prev_q1_even  = 0.0
    prev_ji_odd  = prev_ji_even  = 0.0
    prev_jq_odd  = prev_jq_even  = 0.0
    prev_det_in_odd = prev_det_in_even = 0.0
    prev_q1_in_odd  = prev_q1_in_even  = 0.0
    prev_ji_in_odd  = prev_ji_in_even  = 0.0
    prev_jq_in_odd  = prev_jq_in_even  = 0.0

    I1_for_odd_prev2 = I1_for_odd_prev3 = 0.0
    I1_for_even_prev2 = I1_for_even_prev3 = 0.0
    prev_I2 = prev_Q2 = 0.0
    Re = Im = 0.0
    period = 0.0
    smooth_period = 0.0

    smooth_price = [0.0] * SMOOTH_PRICE_SIZE
    sp_idx = 0

    dc_phase = 0.0
    i_trend1 = i_trend2 = i_trend3 = 0.0

    results = [None] * today  # bars before main loop have no state
    
    while today < n:
        adj = 0.075 * period + 0.54
        smoothed = do_price_wma(close_arr[today])
        smooth_price[sp_idx] = smoothed
        is_even = (today % 2 == 0)

        # Detrender
        det, prev_det_odd, prev_det_even, prev_det_in_odd, prev_det_in_even = do_hilbert(
            det_odd, det_even, prev_det_odd, prev_det_even,
            prev_det_in_odd, prev_det_in_even,
            hilbert_idx, smoothed, is_even, adj)
        # Q1
        q1, prev_q1_odd, prev_q1_even, prev_q1_in_odd, prev_q1_in_even = do_hilbert(
            q1_odd, q1_even, prev_q1_odd, prev_q1_even,
            prev_q1_in_odd, prev_q1_in_even,
            hilbert_idx, det, is_even, adj)
        # jI
        if is_even:
            ji_input = I1_for_even_prev3
        else:
            ji_input = I1_for_odd_prev3
        ji, prev_ji_odd, prev_ji_even, prev_ji_in_odd, prev_ji_in_even = do_hilbert(
            ji_odd, ji_even, prev_ji_odd, prev_ji_even,
            prev_ji_in_odd, prev_ji_in_even,
            hilbert_idx, ji_input, is_even, adj)
        # jQ
        jq, prev_jq_odd, prev_jq_even, prev_jq_in_odd, prev_jq_in_even = do_hilbert(
            jq_odd, jq_even, prev_jq_odd, prev_jq_even,
            prev_jq_in_odd, prev_jq_in_even,
            hilbert_idx, q1, is_even, adj)

        if is_even:
            hilbert_idx = (hilbert_idx + 1) % 3
            Q2 = 0.2 * (q1 + ji) + 0.8 * prev_Q2
            I2 = 0.2 * (I1_for_even_prev3 - jq) + 0.8 * prev_I2
            I1_for_odd_prev3 = I1_for_odd_prev2
            I1_for_odd_prev2 = det
        else:
            Q2 = 0.2 * (q1 + ji) + 0.8 * prev_Q2
            I2 = 0.2 * (I1_for_odd_prev3 - jq) + 0.8 * prev_I2
            I1_for_even_prev3 = I1_for_even_prev2
            I1_for_even_prev2 = det

        Re = 0.2 * (I2 * prev_I2 + Q2 * prev_Q2) + 0.8 * Re
        Im = 0.2 * (I2 * prev_Q2 - Q2 * prev_I2) + 0.8 * Im
        prev_Q2 = Q2; prev_I2 = I2
        prev_period = period
        if Im != 0.0 and Re != 0.0:
            period = 360.0 / (math.atan(Im / Re) * rad2deg)
        period = min(1.5 * prev_period, max(0.67 * prev_period, period))
        period = max(6.0, min(50.0, period))
        period = 0.2 * period + 0.8 * prev_period
        smooth_period = 0.33 * period + 0.67 * smooth_period

        # I1 for phasor: use the delayed version
        if is_even:
            I1_out = I1_for_even_prev3
        else:
            I1_out = I1_for_odd_prev3

        # DCPhase via DFT
        prev_dc_phase = dc_phase
        dc_period_int = int(smooth_period + 0.5)
        real_part = 0.0; imag_part = 0.0
        idx = sp_idx
        for i in range(dc_period_int):
            tr = const_deg2rad_360 * i / dc_period_int if dc_period_int > 0 else 0.0
            real_part += math.sin(tr) * smooth_price[idx]
            imag_part += math.cos(tr) * smooth_price[idx]
            idx = (idx - 1) % SMOOTH_PRICE_SIZE

        temp = abs(imag_part)
        if temp > 0.0:
            dc_phase = math.atan(real_part / imag_part) * rad2deg
        elif temp <= 0.01:
            if real_part < 0.0:
                dc_phase -= 90.0
            elif real_part > 0.0:
                dc_phase += 90.0
        dc_phase += 90.0
        if smooth_period != 0.0:
            dc_phase += 360.0 / smooth_period
        if imag_part < 0.0:
            dc_phase += 180.0
        if dc_phase > 315.0:
            dc_phase -= 360.0

        sine = math.sin(dc_phase * deg2rad)
        lead_sine = math.sin((dc_phase + 45.0) * deg2rad)

        # Trendline
        dc_period_int2 = int(smooth_period + 0.5)
        idx2 = today; avg = 0.0
        for i in range(dc_period_int2):
            avg += close_arr[idx2]; idx2 -= 1
        if dc_period_int2 > 0:
            avg /= dc_period_int2
        trendline = (4.0 * avg + 3.0 * i_trend1 + 2.0 * i_trend2 + i_trend3) / 10.0
        i_trend3 = i_trend2; i_trend2 = i_trend1; i_trend1 = avg

        results.append({
            'smooth_period': smooth_period,
            'I1': I1_out,
            'Q1': q1,
            'dc_phase': dc_phase,
            'sine': sine,
            'lead_sine': lead_sine,
            'trendline': trendline,
            'smooth_price_cur': smooth_price[sp_idx],
            'prev_dc_phase': prev_dc_phase,
        })

        # CIRCBUF_NEXT for smooth_price
        sp_idx = (sp_idx + 1) % SMOOTH_PRICE_SIZE
        today += 1

    return results


def check_ht(close_arr, warmup_i, lookback, extract_fn, ta_vals, name, tol=1e-4):
    results = run_ht_engine(close_arr, warmup_i)
    matches = 0; total = 0
    first_fail = None
    for i in range(lookback, len(close_arr)):
        ta_v = ta_vals[i - lookback] if (i - lookback) < len(ta_vals) else None
        if ta_v is None or math.isnan(ta_v):
            continue
        r = results[i]
        if r is None:
            if first_fail is None:
                first_fail = (i, "no result", ta_v)
            total += 1; continue
        our_v = extract_fn(r)
        total += 1
        if abs(our_v - ta_v) < tol:
            matches += 1
        elif first_fail is None:
            first_fail = (i, our_v, ta_v)
    print(f"{name}: {matches}/{total}", end="")
    if first_fail:
        print(f"  FIRST FAIL: bar={first_fail[0]}, ours={first_fail[1]:.6f}, ta={first_fail[2]:.6f}")
    else:
        print(" ✓")
    return matches == total


if __name__ == "__main__":
    # Load close prices from the parquet test file
    import glob
    parquet = glob.glob("tests/**/*.parquet", recursive=True)
    if not parquet:
        parquet = glob.glob("crates/**/*.parquet", recursive=True)
    print("Found parquet:", parquet[:3])
    
    import polars as pl
    df = pl.read_parquet(parquet[0])
    print("Columns:", df.columns[:5])
    close = df["close"].to_numpy().astype(float)
    print(f"n={len(close)}, close[0]={close[0]:.4f}")

    # Compute ta-lib reference values
    ta_dc    = talib.HT_DCPERIOD(close)
    ta_phase = talib.HT_DCPHASE(close)
    ta_ip, ta_q = talib.HT_PHASOR(close)
    ta_sine, ta_lead = talib.HT_SINE(close)
    ta_tl    = talib.HT_TRENDLINE(close)
    ta_trend = talib.HT_TRENDMODE(close)

    print(f"ta_dc[32]={ta_dc[32]:.6f}, ta_dc[33]={ta_dc[33]:.6f}")
    print(f"ta_phase[63]={ta_phase[63]:.6f}, ta_phase[64]={ta_phase[64]:.6f}")
    print(f"ta_sine[63]={ta_sine[63]:.6f}, ta_lead[63]={ta_lead[63]:.6f}")
    print(f"ta_tl[63]={ta_tl[63]:.6f}")
    print(f"ta_trend[63]={ta_trend[63]}")

    # Check all 6 indicators
    # Group 1: warmup_i=9, lookback=32
    check_ht(close, 9, 32, lambda r: r['smooth_period'], ta_dc[32:], "DCPeriod")
    check_ht(close, 9, 32, lambda r: r['I1'], ta_ip[32:], "Phasor_I1")
    check_ht(close, 9, 32, lambda r: r['Q1'], ta_q[32:], "Phasor_Q1")

    # Group 2: warmup_i=34, lookback=63
    check_ht(close, 34, 63, lambda r: r['dc_phase'], ta_phase[63:], "DCPhase")
    check_ht(close, 34, 63, lambda r: r['sine'], ta_sine[63:], "Sine")
    check_ht(close, 34, 63, lambda r: r['lead_sine'], ta_lead[63:], "LeadSine")
    check_ht(close, 34, 63, lambda r: r['trendline'], ta_tl[63:], "Trendline")

    # Trendmode uses integer values
    ta_trend_float = ta_trend.astype(float)
    def extract_trend_mode(close_arr, warmup_i, lookback):
        results = run_ht_engine(close_arr, warmup_i)
        days_in_trend = 0
        prev_sine = 0.0; prev_lead_sine = 0.0
        prev_dc_phase_saved = 0.0
        out = []
        for i in range(lookback, len(close_arr)):
            r = results[i]
            if r is None:
                out.append(0.0); continue
            sp = r['smooth_period']
            sine = r['sine']; lead_sine = r['lead_sine']
            dc_phase = r['dc_phase']
            trendline = r['trendline']
            smooth_cur = r['smooth_price_cur']
            prev_dc_phase = r['prev_dc_phase']
            
            trend = 1
            if ((sine > lead_sine and prev_sine <= prev_lead_sine) or
                (sine < lead_sine and prev_sine >= prev_lead_sine)):
                days_in_trend = 0
                trend = 0
            days_in_trend += 1
            if days_in_trend < 0.5 * sp:
                trend = 0
            temp = dc_phase - prev_dc_phase
            if sp != 0.0 and (0.67 * 360.0 / sp) < temp < (1.5 * 360.0 / sp):
                trend = 0
            if trendline != 0.0 and abs((smooth_cur - trendline) / trendline) >= 0.015:
                trend = 1
            prev_sine = sine; prev_lead_sine = lead_sine
            out.append(float(trend))
        return out

    trend_ours = extract_trend_mode(close, 34, 63)
    matches = sum(1 for a, b in zip(trend_ours, ta_trend_float[63:]) if abs(a - b) < 0.5)
    total = len([v for v in ta_trend_float[63:] if not math.isnan(v)])
    print(f"Trendmode: {matches}/{total}", end="")
    if matches != total:
        # Find first fail
        for i, (ours, ta) in enumerate(zip(trend_ours, ta_trend_float[63:])):
            if abs(ours - ta) >= 0.5:
                print(f"  FIRST FAIL: bar={i+63}, ours={ours}, ta={ta}")
                break
    else:
        print(" ✓")
