//! MINMAXINDEX — indices of rolling minimum and maximum simultaneously.
//!
//! Returns 0-based absolute indices as `f64`.
//! Output length = `n - period + 1` (lookback = period - 1).

pub struct MinMaxIndexOutput {
    pub minidx: Vec<f64>,
    pub maxidx: Vec<f64>,
}

pub fn minmaxindex(data: &[f64], period: usize) -> MinMaxIndexOutput {
    let n = data.len();
    if period == 0 || n < period {
        return MinMaxIndexOutput { minidx: vec![], maxidx: vec![] };
    }
    let out_len = n - period + 1;

    let cap = period.next_power_of_two().max(4);
    let mask = cap - 1;
    let mut min_buf = vec![0usize; cap];
    let mut max_buf = vec![0usize; cap];
    let mut min_front = 0usize;
    let mut min_back = 0usize;
    let mut max_front = 0usize;
    let mut max_back = 0usize;

    let mut minidx_out = vec![0.0f64; out_len];
    let mut maxidx_out = vec![0.0f64; out_len];

    unsafe {
        let data_ptr = data.as_ptr();
        let minidx_ptr = minidx_out.as_mut_ptr();
        let maxidx_ptr = maxidx_out.as_mut_ptr();

        for i in 0..n {
            if i >= period {
                let ws = i - period + 1;
                while min_front != min_back && *min_buf.get_unchecked(min_front & mask) < ws {
                    min_front = min_front.wrapping_add(1);
                }
                while max_front != max_back && *max_buf.get_unchecked(max_front & mask) < ws {
                    max_front = max_front.wrapping_add(1);
                }
            }

            let val = *data_ptr.add(i);

            while min_front != min_back
                && *data_ptr.add(*min_buf.get_unchecked(min_back.wrapping_sub(1) & mask)) > val
            {
                min_back = min_back.wrapping_sub(1);
            }
            *min_buf.get_unchecked_mut(min_back & mask) = i;
            min_back = min_back.wrapping_add(1);

            while max_front != max_back
                && *data_ptr.add(*max_buf.get_unchecked(max_back.wrapping_sub(1) & mask)) < val
            {
                max_back = max_back.wrapping_sub(1);
            }
            *max_buf.get_unchecked_mut(max_back & mask) = i;
            max_back = max_back.wrapping_add(1);

            if i >= period - 1 {
                let out_i = i + 1 - period;
                *minidx_ptr.add(out_i) = *min_buf.get_unchecked(min_front & mask) as f64;
                *maxidx_ptr.add(out_i) = *max_buf.get_unchecked(max_front & mask) as f64;
            }
        }
    }
    MinMaxIndexOutput { minidx: minidx_out, maxidx: maxidx_out }
}
