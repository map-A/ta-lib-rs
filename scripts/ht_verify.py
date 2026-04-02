"""
Verify HT reference implementation against golden test data.
"""
import json, math, sys

a_coef = 0.0962
b_coef = 0.5769
SMOOTH_PRICE_SIZE = 50

def do_hilbert_transform(buf_arr, prev_scalar, prev_input, hilbert_idx, input_val, adj):
    """Single hilbert transform step. Returns (output, new_buf_arr, new_prev_scalar, new_prev_input)"""
    ht = a_coef * input_val
    output = -buf_arr[hilbert_idx]
    buf_arr[hilbert_idx] = ht
    output += ht
    output -= prev_scalar
    new_prev_scalar = b_coef * prev_input
    output += new_prev_scalar
    output *= adj
    return output, new_prev_scalar, input_val

def run_ht(close_arr, warmup_i):
    """
    Run HT engine. warmup_i=9 for lookback=32, warmup_i=34 for lookback=63.
    Returns list of state dicts indexed by bar number.
    """
    n = len(close_arr)
    rad2deg = 45.0 / math.atan(1.0)
    deg2rad = 1.0 / rad2deg
    c2r360 = math.atan(1.0) * 8.0  # 2*pi

    # --- WMA init (3-bar unrolled) ---
    today = 0; trailing_wma_idx = 0
    pws = close_arr[today]; today += 1      # periodWMASub
    pw_sum = close_arr[today-1]             # periodWMASum (init with c[0])
    pws += close_arr[today]; pw_sum += close_arr[today] * 2.0; today += 1
    pws += close_arr[today]; pw_sum += close_arr[today] * 3.0; today += 1
    trailing_wma_val = 0.0

    def wma_step(new_price):
        nonlocal pws, pw_sum, trailing_wma_idx, trailing_wma_val
        pws += new_price
        pws -= trailing_wma_val
        pw_sum += new_price * 4.0
        trailing_wma_val = close_arr[trailing_wma_idx]; trailing_wma_idx += 1
        smoothed = pw_sum * 0.1
        pw_sum -= pws
        return smoothed

    # WMA warmup
    for _ in range(warmup_i):
        wma_step(close_arr[today]); today += 1

    # --- Init HT state ---
    hilbert_idx = 0
    # Each HT stage: [odd_buf(3), even_buf(3), prev_odd, prev_even, prev_in_odd, prev_in_even]
    def make_ht_stage():
        return [0.0]*3, [0.0]*3, 0.0, 0.0, 0.0, 0.0

    det_ob, det_eb, det_po, det_pe, det_pio, det_pie = make_ht_stage()
    q1_ob,  q1_eb,  q1_po,  q1_pe,  q1_pio,  q1_pie  = make_ht_stage()
    ji_ob,  ji_eb,  ji_po,  ji_pe,  ji_pio,  ji_pie   = make_ht_stage()
    jq_ob,  jq_eb,  jq_po,  jq_pe,  jq_pio,  jq_pie   = make_ht_stage()

    I1_odd_p2 = I1_odd_p3 = 0.0
    I1_even_p2 = I1_even_p3 = 0.0
    prev_I2 = prev_Q2 = Re = Im = 0.0
    period = smooth_period = 0.0

    smooth_price = [0.0] * SMOOTH_PRICE_SIZE
    sp_idx = 0
    dc_phase = 0.0
    it1 = it2 = it3 = 0.0  # i_trend1/2/3

    results = {}

    while today < n:
        adj = 0.075 * period + 0.54
        sv = wma_step(close_arr[today])  # smoothedValue
        smooth_price[sp_idx] = sv
        is_even = (today % 2 == 0)

        # ---- DO_HILBERT_TRANSFORM for each stage ----
        # detrender (input = sv)
        if is_even:
            det, det_pe, det_pie = do_hilbert_transform(det_eb, det_pe, det_pie, hilbert_idx, sv, adj)
        else:
            det, det_po, det_pio = do_hilbert_transform(det_ob, det_po, det_pio, hilbert_idx, sv, adj)

        # Q1 (input = det)
        if is_even:
            q1, q1_pe, q1_pie = do_hilbert_transform(q1_eb, q1_pe, q1_pie, hilbert_idx, det, adj)
        else:
            q1, q1_po, q1_pio = do_hilbert_transform(q1_ob, q1_po, q1_pio, hilbert_idx, det, adj)

        # jI (input = delayed I1)
        if is_even:
            ji_inp = I1_even_p3
            ji, ji_pe, ji_pie = do_hilbert_transform(ji_eb, ji_pe, ji_pie, hilbert_idx, ji_inp, adj)
        else:
            ji_inp = I1_odd_p3
            ji, ji_po, ji_pio = do_hilbert_transform(ji_ob, ji_po, ji_pio, hilbert_idx, ji_inp, adj)

        # jQ (input = q1)
        if is_even:
            jq, jq_pe, jq_pie = do_hilbert_transform(jq_eb, jq_pe, jq_pie, hilbert_idx, q1, adj)
        else:
            jq, jq_po, jq_pio = do_hilbert_transform(jq_ob, jq_po, jq_pio, hilbert_idx, q1, adj)

        if is_even:
            hilbert_idx = (hilbert_idx + 1) % 3
            Q2 = 0.2 * (q1 + ji) + 0.8 * prev_Q2
            I2 = 0.2 * (I1_even_p3 - jq) + 0.8 * prev_I2
            I1_odd_p3 = I1_odd_p2
            I1_odd_p2 = det
            I1_out = I1_even_p3
        else:
            Q2 = 0.2 * (q1 + ji) + 0.8 * prev_Q2
            I2 = 0.2 * (I1_odd_p3 - jq) + 0.8 * prev_I2
            I1_even_p3 = I1_even_p2
            I1_even_p2 = det
            I1_out = I1_odd_p3

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

        # DCPhase via DFT
        prev_dc_phase = dc_phase
        dcp_int = int(smooth_period + 0.5)
        real_p = imag_p = 0.0
        idx = sp_idx
        for i in range(dcp_int):
            tr = c2r360 * i / dcp_int
            real_p += math.sin(tr) * smooth_price[idx]
            imag_p += math.cos(tr) * smooth_price[idx]
            idx = (idx - 1) % SMOOTH_PRICE_SIZE

        tmp = abs(imag_p)
        if tmp > 0.0:
            dc_phase = math.atan(real_p / imag_p) * rad2deg
        elif tmp <= 0.01:
            if real_p < 0.0:
                dc_phase -= 90.0
            elif real_p > 0.0:
                dc_phase += 90.0
        dc_phase += 90.0
        if smooth_period != 0.0:
            dc_phase += 360.0 / smooth_period
        if imag_p < 0.0:
            dc_phase += 180.0
        if dc_phase > 315.0:
            dc_phase -= 360.0

        sine = math.sin(dc_phase * deg2rad)
        lead_sine = math.sin((dc_phase + 45.0) * deg2rad)

        # Trendline: average of last dcp_int raw close bars
        dcp_int2 = int(smooth_period + 0.5)
        avg = 0.0
        idx2 = today
        for _ in range(dcp_int2):
            avg += close_arr[idx2]; idx2 -= 1
        if dcp_int2 > 0:
            avg /= dcp_int2
        trendline = (4.0 * avg + 3.0 * it1 + 2.0 * it2 + it3) / 10.0
        it3 = it2; it2 = it1; it1 = avg

        results[today] = {
            'smooth_period': smooth_period,
            'I1': I1_out,
            'Q1': q1,
            'dc_phase': dc_phase,
            'prev_dc_phase': prev_dc_phase,
            'sine': sine,
            'lead_sine': lead_sine,
            'trendline': trendline,
            'smooth_price_cur': sv,
        }

        # CIRCBUF_NEXT
        sp_idx = (sp_idx + 1) % SMOOTH_PRICE_SIZE
        today += 1

    return results


def cmp(ours, ta, name, tol=1e-4):
    matches = total = 0
    first_fail = None
    for bar, v_ta in enumerate(ta):
        if v_ta is None: continue
        total += 1
        v_our = ours.get(bar)
        if v_our is None:
            if first_fail is None: first_fail = (bar, 'MISSING', v_ta)
            continue
        if abs(v_our - v_ta) < tol:
            matches += 1
        elif first_fail is None:
            first_fail = (bar, v_our, v_ta)
    print(f"  {name}: {matches}/{total}", end="")
    if first_fail:
        b, o, t = first_fail
        print(f"  ✗ first_fail bar={b}: ours={o:.6f} ta={t:.6f}" if isinstance(o, float) else f"  ✗ bar={b}: ours={o}")
    else:
        print(" ✓")
    return matches == total


if __name__ == "__main__":
    gold = {}
    for name in ['ht_dcperiod','ht_dcphase','ht_phasor','ht_sine','ht_trendline','ht_trendmode']:
        with open(f'tests/golden/{name}__real_btcusdt_1d.json') as f:
            gold[name] = json.load(f)

    close = gold['ht_dcperiod']['input']['close']
    n = len(close)
    print(f"n={n}, close[0]={close[0]:.4f}")

    # Group 1: warmup_i=9 (lookback=32)
    print("=== Group 1 (warmup=9, lookback=32) ===")
    r9 = run_ht(close, 9)
    dc_ta = gold['ht_dcperiod']['output']['values']
    cmp({b: r['smooth_period'] for b, r in r9.items()}, dc_ta, "DCPeriod")

    ph_ta_ip = gold['ht_phasor']['output']['inphase']
    ph_ta_q  = gold['ht_phasor']['output']['quadrature']
    cmp({b: r['I1'] for b, r in r9.items()}, ph_ta_ip, "Phasor_I1")
    cmp({b: r['Q1'] for b, r in r9.items()}, ph_ta_q,  "Phasor_Q1")

    # Group 2: warmup_i=34 (lookback=63)
    print("=== Group 2 (warmup=34, lookback=63) ===")
    r34 = run_ht(close, 34)
    phase_ta = gold['ht_dcphase']['output']['values']
    cmp({b: r['dc_phase'] for b, r in r34.items()}, phase_ta, "DCPhase")
    sine_ta   = gold['ht_sine']['output']['sine']
    lead_ta   = gold['ht_sine']['output']['leadsine']
    cmp({b: r['sine'] for b, r in r34.items()}, sine_ta, "Sine")
    cmp({b: r['lead_sine'] for b, r in r34.items()}, lead_ta, "LeadSine")
    tl_ta = gold['ht_trendline']['output']['values']
    cmp({b: r['trendline'] for b, r in r34.items()}, tl_ta, "Trendline")

    # Trendmode
    print("=== Trendmode ===")
    tm_ta = gold['ht_trendmode']['output']['values']
    # trendmode needs its own state machine
    days_in_trend = 0
    prev_sine = prev_lead_sine = 0.0
    tm_ours = {}
    for bar in range(n):
        r = r34.get(bar)
        if r is None:
            tm_ours[bar] = 0.0
            continue
        sp = r['smooth_period']
        sine = r['sine']; lead_sine = r['lead_sine']
        dc_phase = r['dc_phase']; prev_dc_phase = r['prev_dc_phase']
        trendline = r['trendline']; smooth_cur = r['smooth_price_cur']

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
        tm_ours[bar] = float(trend)

    cmp(tm_ours, tm_ta, "Trendmode", tol=0.5)
