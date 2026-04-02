//! MINMAXINDEX — Indices of Rolling Minimum and Maximum.
//! Never outputs NaN for either index array.

pub struct MinMaxIndexOutput {
    pub min_idx: Vec<f64>,
    pub max_idx: Vec<f64>,
}

pub fn minmaxindex(data: &[f64], period: usize) -> MinMaxIndexOutput {
    let n = data.len();
    if period == 0 || n < period {
        return MinMaxIndexOutput { min_idx: vec![], max_idx: vec![] };
    }
    let out_len = n - period + 1;
    let mut out_max_idx = vec![0.0_f64; out_len];
    let mut out_min_idx = vec![0.0_f64; out_len];

    let mut highest = f64::NAN;
    let mut highest_idx: isize = -1;
    let mut lowest = f64::NAN;
    let mut lowest_idx: isize = -1;

    for i in 0..out_len {
        let newest = i + period - 1;

        if highest_idx < i as isize {
            highest_idx = i as isize; highest = data[i];
            for j in (i+1)..=newest { if data[j] > highest { highest = data[j]; highest_idx = j as isize; } }
        } else {
            if data[newest] > highest { highest = data[newest]; highest_idx = newest as isize; }
        }

        if lowest_idx < i as isize {
            lowest_idx = i as isize; lowest = data[i];
            for j in (i+1)..=newest { if data[j] < lowest { lowest = data[j]; lowest_idx = j as isize; } }
        } else {
            if data[newest] < lowest { lowest = data[newest]; lowest_idx = newest as isize; }
        }

        out_max_idx[i] = highest_idx as f64;
        out_min_idx[i] = lowest_idx as f64;
    }
    MinMaxIndexOutput { min_idx: out_min_idx, max_idx: out_max_idx }
}
