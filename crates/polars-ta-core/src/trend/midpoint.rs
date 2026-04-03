//! Midpoint Over Period (MIDPOINT)
//!
//! midpoint[i] = (max + min) / 2 over the rolling window.
//!
//! NaN rule: output NaN iff oldest element is NaN (ta-lib always checks oldest).

pub fn midpoint(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let out_len = n - period + 1;
    let mut out = Vec::with_capacity(out_len);

    let mut highest = f64::NAN;
    let mut highest_idx: isize = -1;
    let mut lowest = f64::NAN;
    let mut lowest_idx: isize = -1;

    for i in 0..out_len {
        let newest = i + period - 1;

        // ta-lib MIDPOINT explicitly checks oldest; NaN oldest → NaN output.
        if data[i].is_nan() {
            // Invalidate cache so next window rescans correctly.
            highest_idx = -1;
            lowest_idx = -1;
            out.push(f64::NAN);
            continue;
        }

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

        out.push((highest + lowest) / 2.0);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midpoint_output_length() {
        let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        assert_eq!(midpoint(&data, 5).len(), 16);
    }

    #[test]
    fn midpoint_basic() {
        let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];
        let r = midpoint(&data, 3);
        assert_eq!(r.len(), 3);
        assert!((r[0] - 2.0).abs() < 1e-10);
        assert!((r[1] - 3.5).abs() < 1e-10);
        assert!((r[2] - 3.5).abs() < 1e-10);
    }

    #[test]
    fn midpoint_boundary_short() {
        assert!(midpoint(&[1.0, 2.0, 3.0], 5).is_empty());
    }
}
