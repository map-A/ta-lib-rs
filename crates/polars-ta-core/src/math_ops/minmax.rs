//! MINMAX — Rolling Minimum and Maximum in one pass.

pub struct MinMaxOutput {
    pub min: Vec<f64>,
    pub max: Vec<f64>,
}

pub fn minmax(data: &[f64], period: usize) -> MinMaxOutput {
    let n = data.len();
    if period == 0 || n < period {
        return MinMaxOutput { min: vec![], max: vec![] };
    }
    let out_len = n - period + 1;
    let mut out_min = vec![0.0_f64; out_len];
    let mut out_max = vec![0.0_f64; out_len];

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

        out_max[i] = highest;
        out_min[i] = lowest;
    }
    MinMaxOutput { min: out_min, max: out_max }
}
