impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let xor: i32 = n ^ (n >> 1);
        (xor & (xor + 1)) == 0
    }
}
