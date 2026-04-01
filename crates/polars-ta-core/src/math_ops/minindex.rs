//! MININDEX — index of rolling minimum using O(n) ring-buffer monotone deque.
//!
//! Returns the 0-based absolute index in the original array of the minimum
//! value within each window, as `f64`.
//! Output length = `n - period + 1` (lookback = period - 1).

pub fn minindex(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let out_len = n - period + 1;

    let cap = period.next_power_of_two().max(4);
    let mask = cap - 1;
    let mut buf = vec![0usize; cap];
    let mut front = 0usize;
    let mut back = 0usize;

    let mut out = vec![0.0f64; out_len];

    unsafe {
        let data_ptr = data.as_ptr();
        let out_ptr = out.as_mut_ptr();

        for i in 0..n {
            if i >= period {
                let ws = i - period + 1;
                while front != back && *buf.get_unchecked(front & mask) < ws {
                    front = front.wrapping_add(1);
                }
            }
            while front != back
                && *data_ptr.add(*buf.get_unchecked(back.wrapping_sub(1) & mask))
                    > *data_ptr.add(i)
            {
                back = back.wrapping_sub(1);
            }
            *buf.get_unchecked_mut(back & mask) = i;
            back = back.wrapping_add(1);

            if i >= period - 1 {
                *out_ptr.add(i + 1 - period) =
                    *buf.get_unchecked(front & mask) as f64;
            }
        }
    }
    out
}
