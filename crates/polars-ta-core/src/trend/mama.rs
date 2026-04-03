//! Mesa Adaptive Moving Average (MAMA) and Following Adaptive Moving Average (FAMA)
//!
//! 完全匹配 ta-lib 的 `TA_MAMA`，基于 ta-lib C 源码逐行翻译。
//!
//! # Algorithm
//!
//! 使用偶奇（Even/Odd）双路 Hilbert Transform 内核，与 ta-lib `DO_HILBERT_EVEN` /
//! `DO_HILBERT_ODD` 宏完全一致。lookback = 32。
//!
//! # Parameters
//!
//! - `data`       — 输入价格序列（通常为 close）
//! - `fast_limit` — 快速限制，默认 0.5
//! - `slow_limit` — 慢速限制，默认 0.05
//!
//! # Output
//!
//! [`MamaOutput`]，包含两个等长的 `Vec<f64>`：
//! - `mama` — MAMA 线
//! - `fama` — FAMA 线

const A: f64 = 0.0962;
const B: f64 = 0.5769;
const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;

/// ta-lib `DO_HILBERT_TRANSFORM` 宏的 Rust 等价实现。
///
/// 更新 `buf[hidx]`、`prev`、`prev_input`，返回变换值。
#[inline(always)]
fn do_hilbert(
    buf: &mut [f64; 3],
    input: f64,
    prev: &mut f64,
    prev_input: &mut f64,
    adj: f64,
    hidx: usize,
) -> f64 {
    let ht_tmp = A * input;
    let mut var = -buf[hidx];
    buf[hidx] = ht_tmp;
    var += ht_tmp;
    var -= *prev;
    *prev = B * *prev_input;
    var += *prev;
    *prev_input = input;
    var * adj
}

/// MAMA/FAMA 的输出结构体。
pub struct MamaOutput {
    /// Mesa Adaptive Moving Average。
    pub mama: Vec<f64>,
    /// Following Adaptive Moving Average。
    pub fama: Vec<f64>,
}

/// Mesa Adaptive Moving Average。
///
/// 详见 [模块文档](self)。
pub fn mama(data: &[f64], fast_limit: f64, slow_limit: f64) -> MamaOutput {
    const LOOKBACK: usize = 32;

    let n = data.len();
    if n <= LOOKBACK {
        return MamaOutput {
            mama: vec![],
            fama: vec![],
        };
    }

    let out_len = n - LOOKBACK;
    let mut out_mama = Vec::with_capacity(out_len);
    let mut out_fama = Vec::with_capacity(out_len);

    // ── Hilbert 缓冲区（偶/奇双路） ──────────────────────────────────────
    let mut det_b_odd = [0.0f64; 3];
    let mut det_b_even = [0.0f64; 3];
    let mut q1_b_odd = [0.0f64; 3];
    let mut q1_b_even = [0.0f64; 3];
    let mut ji_b_odd = [0.0f64; 3];
    let mut ji_b_even = [0.0f64; 3];
    let mut jq_b_odd = [0.0f64; 3];
    let mut jq_b_even = [0.0f64; 3];

    let mut det_p_odd = 0.0f64;
    let mut det_p_even = 0.0f64;
    let mut det_pi_odd = 0.0f64;
    let mut det_pi_even = 0.0f64;
    let mut q1_p_odd = 0.0f64;
    let mut q1_p_even = 0.0f64;
    let mut q1_pi_odd = 0.0f64;
    let mut q1_pi_even = 0.0f64;
    let mut ji_p_odd = 0.0f64;
    let mut ji_p_even = 0.0f64;
    let mut ji_pi_odd = 0.0f64;
    let mut ji_pi_even = 0.0f64;
    let mut jq_p_odd = 0.0f64;
    let mut jq_p_even = 0.0f64;
    let mut jq_pi_odd = 0.0f64;
    let mut jq_pi_even = 0.0f64;

    let mut hilbert_idx: usize = 0;

    // I1 延迟线（偶/奇各一套）
    let mut i1_odd_prev2 = 0.0f64;
    let mut i1_odd_prev3 = 0.0f64;
    let mut i1_even_prev2 = 0.0f64;
    let mut i1_even_prev3 = 0.0f64;

    let mut prev_i2 = 0.0f64;
    let mut prev_q2 = 0.0f64;
    let mut re = 0.0f64;
    let mut im = 0.0f64;
    let mut period = 0.0f64;
    let mut mama_v = 0.0f64;
    let mut fama_v = 0.0f64;
    let mut prev_phase = 0.0f64;

    // ── WMA 初始化（与 ta-lib C 完全一致） ────────────────────────────────
    // ta-lib 从 startIdx - lookbackTotal = 0 开始
    let mut trailing_wma_idx: usize = 0;
    let mut today: usize = 0;

    let mut period_wma_sub = data[today];
    today += 1;
    let mut period_wma_sum = period_wma_sub;
    period_wma_sub += data[today];
    period_wma_sum += data[today] * 2.0;
    today += 1;
    period_wma_sub += data[today];
    period_wma_sum += data[today] * 3.0;
    today += 1;

    let mut trailing_wma_value = 0.0f64;
    // 9 次 WMA 预热（到 today=12）——仅更新 WMA 内部状态，不保存 smoothed
    for _ in 0..9 {
        let p = data[today];
        today += 1;
        period_wma_sub += p;
        period_wma_sub -= trailing_wma_value;
        period_wma_sum += p * 4.0;
        trailing_wma_value = data[trailing_wma_idx];
        trailing_wma_idx += 1;
        period_wma_sum -= period_wma_sub;
    }

    let mut smoothed;

    // ── 主循环 ────────────────────────────────────────────────────────────
    while today < n {
        let adj = 0.075 * period + 0.54;
        let today_value = data[today];

        // DO_PRICE_WMA
        period_wma_sub += today_value;
        period_wma_sub -= trailing_wma_value;
        period_wma_sum += today_value * 4.0;
        trailing_wma_value = data[trailing_wma_idx];
        trailing_wma_idx += 1;
        smoothed = period_wma_sum * 0.1;
        period_wma_sum -= period_wma_sub;

        let phase;

        if today.is_multiple_of(2) {
            // 偶数 bar
            let detrender = do_hilbert(
                &mut det_b_even,
                smoothed,
                &mut det_p_even,
                &mut det_pi_even,
                adj,
                hilbert_idx,
            );
            let q1 = do_hilbert(
                &mut q1_b_even,
                detrender,
                &mut q1_p_even,
                &mut q1_pi_even,
                adj,
                hilbert_idx,
            );
            let ji = do_hilbert(
                &mut ji_b_even,
                i1_even_prev3,
                &mut ji_p_even,
                &mut ji_pi_even,
                adj,
                hilbert_idx,
            );
            let jq = do_hilbert(
                &mut jq_b_even,
                q1,
                &mut jq_p_even,
                &mut jq_pi_even,
                adj,
                hilbert_idx,
            );

            hilbert_idx += 1;
            if hilbert_idx == 3 {
                hilbert_idx = 0;
            }

            let q2 = 0.2 * (q1 + ji) + 0.8 * prev_q2;
            let i2 = 0.2 * (i1_even_prev3 - jq) + 0.8 * prev_i2;

            // 更新奇路延迟线
            i1_odd_prev3 = i1_odd_prev2;
            i1_odd_prev2 = detrender;

            phase = if i1_even_prev3 != 0.0 {
                (q1 / i1_even_prev3).atan() * RAD2DEG
            } else {
                0.0
            };

            // 更新 I2/Q2（供 period 计算）
            re = 0.2 * (i2 * prev_i2 + q2 * prev_q2) + 0.8 * re;
            im = 0.2 * (i2 * prev_q2 - q2 * prev_i2) + 0.8 * im;
            prev_q2 = q2;
            prev_i2 = i2;
        } else {
            // 奇数 bar
            let detrender = do_hilbert(
                &mut det_b_odd,
                smoothed,
                &mut det_p_odd,
                &mut det_pi_odd,
                adj,
                hilbert_idx,
            );
            let q1 = do_hilbert(
                &mut q1_b_odd,
                detrender,
                &mut q1_p_odd,
                &mut q1_pi_odd,
                adj,
                hilbert_idx,
            );
            let ji = do_hilbert(
                &mut ji_b_odd,
                i1_odd_prev3,
                &mut ji_p_odd,
                &mut ji_pi_odd,
                adj,
                hilbert_idx,
            );
            let jq = do_hilbert(
                &mut jq_b_odd,
                q1,
                &mut jq_p_odd,
                &mut jq_pi_odd,
                adj,
                hilbert_idx,
            );

            let q2 = 0.2 * (q1 + ji) + 0.8 * prev_q2;
            let i2 = 0.2 * (i1_odd_prev3 - jq) + 0.8 * prev_i2;

            // 更新偶路延迟线
            i1_even_prev3 = i1_even_prev2;
            i1_even_prev2 = detrender;

            phase = if i1_odd_prev3 != 0.0 {
                (q1 / i1_odd_prev3).atan() * RAD2DEG
            } else {
                0.0
            };

            // 更新 I2/Q2
            re = 0.2 * (i2 * prev_i2 + q2 * prev_q2) + 0.8 * re;
            im = 0.2 * (i2 * prev_q2 - q2 * prev_i2) + 0.8 * im;
            prev_q2 = q2;
            prev_i2 = i2;
        }

        // delta phase → alpha
        let mut delta_phase = prev_phase - phase;
        prev_phase = phase;
        if delta_phase < 1.0 {
            delta_phase = 1.0;
        }

        let alpha = if delta_phase > 1.0 {
            let a2 = fast_limit / delta_phase;
            if a2 < slow_limit {
                slow_limit
            } else {
                a2
            }
        } else {
            fast_limit
        };

        mama_v = alpha * today_value + (1.0 - alpha) * mama_v;
        let alpha2 = alpha * 0.5;
        fama_v = alpha2 * mama_v + (1.0 - alpha2) * fama_v;

        if today >= LOOKBACK {
            out_mama.push(mama_v);
            out_fama.push(fama_v);
        }

        // period 更新（在 mama/fama 之后，与 C 代码一致）
        let prev_period = period;
        if im != 0.0 && re != 0.0 {
            period = 360.0 / ((im / re).atan() * RAD2DEG);
        }
        let tmp2 = 1.5 * prev_period;
        if period > tmp2 {
            period = tmp2;
        }
        let tmp2 = 0.67 * prev_period;
        if period < tmp2 {
            period = tmp2;
        }
        period = period.clamp(6.0, 50.0);
        period = 0.2 * period + 0.8 * prev_period;

        today += 1;
    }

    MamaOutput {
        mama: out_mama,
        fama: out_fama,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mama_output_length() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let out = mama(&data, 0.5, 0.05);
        assert_eq!(out.mama.len(), 100 - 32);
        assert_eq!(out.fama.len(), out.mama.len());
    }

    #[test]
    fn mama_too_short() {
        let data = vec![1.0f64; 32];
        let out = mama(&data, 0.5, 0.05);
        assert!(out.mama.is_empty());
    }

    #[test]
    fn mama_boundary_exact() {
        let data = vec![100.0f64; 33];
        let out = mama(&data, 0.5, 0.05);
        assert_eq!(out.mama.len(), 1);
        assert_eq!(out.fama.len(), 1);
    }

    #[test]
    fn mama_constant_series() {
        // 常数序列：MAMA 和 FAMA 应当收敛到该常数
        let data = vec![50.0f64; 200];
        let out = mama(&data, 0.5, 0.05);
        assert_eq!(out.mama.len(), 200 - 32);
        let last = *out.mama.last().unwrap();
        assert!((last - 50.0).abs() < 1.0, "last mama={last}");
    }
}
