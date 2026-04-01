//! Percentage Price Oscillator (PPO)
//!
//! `PPO[i] = (SMA_fast[i] - SMA_slow[i]) / SMA_slow[i] * 100`
//!
//! ta-lib PPO with matype=0 uses sliding SMA (not EMA). This matches ta-lib exactly.
//!
//! lookback = slow_period - 1

pub fn ppo(data: &[f64], fast_period: usize, slow_period: usize) -> Vec<f64> {
    let n = data.len();
    if fast_period == 0 || slow_period == 0 || fast_period >= slow_period || n < slow_period {
        return vec![];
    }

    let out_len = n - (slow_period - 1);
    let mut out = Vec::with_capacity(out_len);

    let inv_fast = 1.0 / fast_period as f64;
    let inv_slow = 1.0 / slow_period as f64;

    // 初始化慢窗口和快窗口的滑动和
    let mut slow_sum: f64 = data[..slow_period].iter().sum();
    let mut fast_sum: f64 = data[slow_period - fast_period..slow_period].iter().sum();
    let s = slow_sum * inv_slow;
    out.push(if s == 0.0 {
        0.0
    } else {
        (fast_sum * inv_fast - s) / s * 100.0
    });

    for i in slow_period..n {
        slow_sum += data[i] - data[i - slow_period];
        fast_sum += data[i] - data[i - fast_period];
        let s = slow_sum * inv_slow;
        out.push(if s == 0.0 {
            0.0
        } else {
            (fast_sum * inv_fast - s) / s * 100.0
        });
    }

    out
}
