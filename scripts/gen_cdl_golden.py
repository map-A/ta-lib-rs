#!/usr/bin/env python3
"""Generate golden JSON files for all 36 CDL patterns."""
import talib, numpy as np, json
from pathlib import Path
from datetime import datetime, timezone

rng = np.random.default_rng(seed=42)
close = 100.0 + rng.normal(0, 2, 1000).cumsum()
close = np.abs(close)
high = close * (1 + rng.uniform(0, 0.02, 1000))
low = close * (1 - rng.uniform(0, 0.02, 1000))
open_ = close * (1 + rng.normal(0, 0.005, 1000))

cdl_funcs = [
    'CDL2CROWS','CDL3BLACKCROWS','CDL3INSIDE','CDL3LINESTRIKE','CDL3OUTSIDE',
    'CDL3STARSINSOUTH','CDL3WHITESOLDIERS','CDLABANDONEDBABY','CDLADVANCEBLOCK',
    'CDLBELTHOLD','CDLBREAKAWAY','CDLCLOSINGMARUBOZU','CDLCONCEALBABYSWALL',
    'CDLCOUNTERATTACK','CDLDARKCLOUDCOVER','CDLDOJI','CDLDOJISTAR','CDLDRAGONFLYDOJI',
    'CDLENGULFING','CDLEVENINGDOJISTAR','CDLEVENINGSTAR','CDLGAPSIDESIDEWHITE',
    'CDLGRAVESTONEDOJI','CDLHAMMER','CDLHANGINGMAN','CDLHARAMI','CDLHARAMICROSS',
    'CDLHIGHWAVE','CDLHIKKAKE','CDLHIKKAKEMOD','CDLHOMINGPIGEON','CDLIDENTICAL3CROWS',
    'CDLINNECK','CDLINVERTEDHAMMER','CDLKICKING','CDLKICKINGBYLENGTH',
]

output_dir = Path('tests/golden')
output_dir.mkdir(exist_ok=True)

def arr_to_json(arr):
    return [None if np.isnan(v) else float(v) for v in arr]

now = datetime.now(timezone.utc).isoformat()
talib_version = talib.__version__

for fn in cdl_funcs:
    f = getattr(talib, fn)
    result = f(open_, high, low, close)
    result_float = result.astype(float)
    
    fn_lower = fn.lower()
    fname = output_dir / f"{fn_lower}__normal_1000.json"
    
    data = {
        "meta": {
            "indicator": fn_lower,
            "params": {},
            "talib_version": talib_version,
            "generated_at": now,
            "dataset": "normal_1000",
        },
        "input": {
            "open": arr_to_json(open_),
            "high": arr_to_json(high),
            "low": arr_to_json(low),
            "close": arr_to_json(close),
        },
        "output": {
            "values": arr_to_json(result_float),
        },
        "lookback": 0,
        "output_length": int(len(result)),
    }
    
    with open(fname, 'w') as f_out:
        json.dump(data, f_out, separators=(',', ':'))
    print(f"Generated {fname} (nonzero: {int(np.sum(result != 0))})")
