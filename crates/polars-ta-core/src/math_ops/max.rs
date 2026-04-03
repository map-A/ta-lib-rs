//! MAX — Rolling Maximum
//!
//! Exactly replicates ta-lib's `TA_MAX` NaN behavior:
//! - Uses the conditional-rescan trick for O(1) amortised performance.
//! - When a rescan is needed (previous max left the window):
//!   - If oldest is NaN: output NaN (scan seeds with NaN, nothing beats it).
//!   - If oldest is valid: scan from oldest, skip NaN via `NaN > x = false`.
//! - When incremental (previous max still in window):
//!   - Check newest element only; oldest NaN is irrelevant.

pub fn max(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let out_len = n - period + 1;
    let mut out = vec![0.0_f64; out_len];

    let mut highest = f64::NAN;
    let mut highest_idx: isize = -1;

    for i in 0..out_len {
        let newest = i + period - 1;

        if highest_idx < i as isize {
            // Previous max left window — rescan from oldest.
            // IEEE 754: NaN > anything = false, so NaN seeds stop propagation.
            highest_idx = i as isize;
            highest = data[i];
            for (offset, &v) in data[(i + 1)..=newest].iter().enumerate() {
                if v > highest {
                    highest = v;
                    highest_idx = (i + 1 + offset) as isize;
                }
            }
        } else {
            // Previous max still in window — only check newest element.
            if data[newest] > highest {
                highest = data[newest];
                highest_idx = newest as isize;
            }
        }
        out[i] = highest;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_basic() {
        let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];
        let result = max(&data, 3);
        assert_eq!(result, vec![3.0, 5.0, 5.0]);
    }

    #[test]
    fn max_boundary_short() {
        assert!(max(&[1.0, 2.0], 3).is_empty());
    }

    #[test]
    fn max_nan_oldest_rescan_forces_nan() {
        // NaN at oldest, previous max has just left → rescan seeds with NaN → NaN output
        let data = vec![10.0, f64::NAN, 3.0, 2.0];
        let r = max(&data, 2);
        // out[0] = max(10, NaN): prev_max_idx=-1 < 0, rescan: seed=10, NaN skipped → 10
        // out[1] = max(NaN, 3):  prev_max_idx=0 < 1, rescan: seed=NaN, 3 not > NaN → NaN
        // out[2] = max(3, 2):    prev_max_idx=-1 < 2, rescan: seed=3, 2 not > 3 → 3
        assert!(!r[0].is_nan(), "r[0]={}", r[0]);
        assert!(r[1].is_nan(), "r[1]={}", r[1]);
        assert!(!r[2].is_nan(), "r[2]={}", r[2]);
    }

    #[test]
    fn max_nan_oldest_incremental_valid() {
        // NaN at oldest but prev max is still in window → incremental → valid
        let data = vec![5.0, f64::NAN, 10.0, 3.0, 2.0];
        let r = max(&data, 3);
        // out[0] = max(5, NaN, 10): rescan, seed=5, 10>5 → 10 at idx=2
        // out[1] = max(NaN, 10, 3): prev_max_idx=2 >= 1 → incremental, 3 not>10 → 10
        // out[2] = max(10, 3, 2): prev_max_idx=2 >= 2 → incremental, 2 not>10 → 10
        assert!((r[0] - 10.0).abs() < 1e-10);
        assert!(!r[1].is_nan());
        assert!((r[1] - 10.0).abs() < 1e-10);
    }
}
