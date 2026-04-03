//! MAXINDEX — Index of Rolling Maximum
//!
//! Returns the absolute (0-based) index of the maximum, as `f64`.
//! Never outputs NaN — even when oldest is NaN, the index (pointing to the NaN
//! element) is returned, matching ta-lib's C behavior exactly.

pub fn maxindex(data: &[f64], period: usize) -> Vec<f64> {
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
            highest_idx = i as isize;
            highest = data[i];
            for (offset, &v) in data[(i + 1)..=newest].iter().enumerate() {
                if v > highest {
                    highest = v;
                    highest_idx = (i + 1 + offset) as isize;
                }
            }
        } else if data[newest] > highest {
            highest = data[newest];
            highest_idx = newest as isize;
        }
        // Always output the index (even when highest is NaN — index points to oldest NaN element)
        out[i] = highest_idx as f64;
    }
    out
}
