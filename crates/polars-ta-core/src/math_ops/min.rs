//! MIN — rolling minimum using O(n) ring-buffer monotone deque.
//!
//! Output length = `n - period + 1` (lookback = period - 1).

pub fn min(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let out_len = n - period + 1;

    // 幂次方容量的环形缓冲区，用位掩码替代取模
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
            // 移除滑出窗口的过期下标
            if i >= period {
                let ws = i - period + 1;
                while front != back && *buf.get_unchecked(front & mask) < ws {
                    front = front.wrapping_add(1);
                }
            }
            // 维护单调递增（移除所有大于当前值的尾部下标）
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
                    *data_ptr.add(*buf.get_unchecked(front & mask));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_basic() {
        let data = vec![5.0, 3.0, 4.0, 1.0, 2.0];
        let result = min(&data, 3);
        assert_eq!(result, vec![3.0, 1.0, 1.0]);
    }

    #[test]
    fn min_boundary_short() {
        assert!(min(&[1.0, 2.0], 3).is_empty());
    }
}
