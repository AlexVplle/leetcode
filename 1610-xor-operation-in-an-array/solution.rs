impl Solution {
    pub fn xor_operation(n: i32, mut start: i32) -> i32 {
        let mut result: i32 = start;
        for i in 1..n {
            result ^= start + 2 * i;
        }
        result
    }
}
