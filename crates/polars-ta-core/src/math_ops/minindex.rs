//! MININDEX — Index of Rolling Minimum
//!
//! Returns the absolute (0-based) index of the minimum, as `f64`.
//! Never outputs NaN — same behavior as MAXINDEX.

pub fn minindex(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let out_len = n - period + 1;
    let mut out = vec![0.0_f64; out_len];

    let mut lowest = f64::NAN;
    let mut lowest_idx: isize = -1;

    for i in 0..out_len {
        let newest = i + period - 1;

        if lowest_idx < i as isize {
            lowest_idx = i as isize;
            lowest = data[i];
            for (offset, &v) in data[(i + 1)..=newest].iter().enumerate() {
                if v < lowest {
                    lowest = v;
                    lowest_idx = (i + 1 + offset) as isize;
                }
            }
        } else if data[newest] < lowest {
            lowest = data[newest];
            lowest_idx = newest as isize;
        }
        out[i] = lowest_idx as f64;
    }
    out
}
