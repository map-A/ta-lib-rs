//! MIN — Rolling Minimum
//!
//! Exactly replicates ta-lib's `TA_MIN` NaN behavior using conditional-rescan.

pub fn min(data: &[f64], period: usize) -> Vec<f64> {
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
            for j in (i + 1)..=newest {
                if data[j] < lowest {
                    lowest = data[j];
                    lowest_idx = j as isize;
                }
            }
        } else {
            if data[newest] < lowest {
                lowest = data[newest];
                lowest_idx = newest as isize;
            }
        }
        out[i] = lowest;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_basic() {
        let data = vec![3.0, 1.0, 2.0, 5.0, 4.0];
        let result = min(&data, 3);
        assert_eq!(result, vec![1.0, 1.0, 2.0]);
    }

    #[test]
    fn min_boundary_short() {
        assert!(min(&[1.0, 2.0], 3).is_empty());
    }
}
