impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut result: i32 = 0;
        for i in 0..32 {
            result |= ((n >> i) & 1) << (31 - i);
        }
        result
    }
}
