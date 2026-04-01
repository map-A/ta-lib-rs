//! SUM — rolling sum using a sliding window (O(n)).
//!
//! Output length = `n - period + 1` (lookback = period - 1).

pub fn sum(data: &[f64], period: usize) -> Vec<f64> {
    let n = data.len();
    if period == 0 || n < period {
        return vec![];
    }
    let out_len = n - period + 1;
    let mut out = Vec::with_capacity(out_len);
    let mut s: f64 = data[..period].iter().sum();
    out.push(s);
    for i in period..n {
        s += data[i] - data[i - period];
        out.push(s);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sum(&data, 3);
        assert_eq!(result, vec![6.0, 9.0, 12.0]);
    }

    #[test]
    fn sum_boundary_short() {
        assert!(sum(&[1.0, 2.0], 3).is_empty());
    }
}
