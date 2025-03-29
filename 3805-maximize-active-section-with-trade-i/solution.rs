impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let original = s.as_bytes();
        let b: i32 = original.iter().filter(|&&c| c == b'1').count() as i32;
        let n = original.len();
        let mut v: Vec<u8> = Vec::with_capacity(n + 2);
        v.push(b'1');
        v.extend_from_slice(original);
        v.push(b'1');
        let m = v.len();
        let mut left_zeros = vec![0; m];
        for i in 0..m {
            if v[i] == b'0' {
                left_zeros[i] = if i == 0 { 1 } else { left_zeros[i - 1] + 1 };
            }
        }
        let mut right_zeros = vec![0; m];
        for i in (0..m).rev() {
            if v[i] == b'0' {
                right_zeros[i] = if i == m - 1 {
                    1
                } else {
                    right_zeros[i + 1] + 1
                };
            }
        }
        let mut res = b;
        let mut i = 1;
        while i < m - 1 {
            if v[i] == b'1' {
                let mut j = i;
                while j < m && v[j] == b'1' {
                    j += 1;
                }
                if j < m - 1 && v[i - 1] == b'0' && v[j] == b'0' {
                    let left_count = left_zeros[i - 1];
                    let right_count = right_zeros[j];
                    res = res.max(b + left_count + right_count);
                }
                i = j;
            } else {
                i += 1;
            }
        }
        res
    }
}
