//! BETA — 滚动回归斜率（收益率序列）
//!
//! 计算 real1 收益率对 real0 收益率的线性回归斜率，窗口大小为 period。
//! 与 ta-lib 一致：分母使用 Σr0²（real0 的方差），而非 Σr1²。
//!
//! slope = (n * Σr0r1 - Σr0 * Σr1) / (n * Σr0² - (Σr0)²)
//!
//! lookback = period（需要 period+1 个价格点才能产生 period 个收益率）

pub fn beta(real0: &[f64], real1: &[f64], period: usize) -> Vec<f64> {
    let n = real0.len().min(real1.len());
    if period == 0 || n <= period {
        return vec![];
    }

    let p = period as f64;
    let out_len = n - period;
    let mut out = Vec::with_capacity(out_len);

    let mut sum_r0 = 0.0_f64;
    let mut sum_r1 = 0.0_f64;
    let mut sum_r0r1 = 0.0_f64;
    let mut sum_r0sq = 0.0_f64;

    // 初始化第一个窗口
    for i in 0..period {
        let r0 = (real0[i + 1] - real0[i]) / real0[i];
        let r1 = (real1[i + 1] - real1[i]) / real1[i];
        sum_r0 += r0;
        sum_r1 += r1;
        sum_r0r1 += r0 * r1;
        sum_r0sq += r0 * r0;
    }

    // 分母使用 r0 的方差（与 ta-lib 一致）
    let calc = |sr0: f64, sr1: f64, sr0r1: f64, sr0sq: f64| -> f64 {
        let denom = p * sr0sq - sr0 * sr0;
        if denom == 0.0 { 0.0 } else { (p * sr0r1 - sr0 * sr1) / denom }
    };

    out.push(calc(sum_r0, sum_r1, sum_r0r1, sum_r0sq));

    for i in period..n - 1 {
        let old_r0 = (real0[i - period + 1] - real0[i - period]) / real0[i - period];
        let old_r1 = (real1[i - period + 1] - real1[i - period]) / real1[i - period];
        let new_r0 = (real0[i + 1] - real0[i]) / real0[i];
        let new_r1 = (real1[i + 1] - real1[i]) / real1[i];

        sum_r0 += new_r0 - old_r0;
        sum_r1 += new_r1 - old_r1;
        sum_r0r1 += new_r0 * new_r1 - old_r0 * old_r1;
        sum_r0sq += new_r0 * new_r0 - old_r0 * old_r0;

        out.push(calc(sum_r0, sum_r1, sum_r0r1, sum_r0sq));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beta_identical_series() {
        let data: Vec<f64> = (1..=10).map(|x| x as f64).collect();
        let result = beta(&data, &data, 5);
        assert_eq!(result.len(), 5);
        for v in &result {
            assert!((v - 1.0).abs() < 1e-10, "identical series beta=1, got {}", v);
        }
    }

    #[test]
    fn beta_boundary_short() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(beta(&data, &data, 3).is_empty());
    }

    #[test]
    fn beta_period_zero() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(beta(&data, &data, 0).is_empty());
    }
}
