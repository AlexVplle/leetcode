impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let n: usize = nums.len();
        for index in 0..n {
            result ^= index as i32;
            result ^= nums[index];
        }
        result ^ n as i32
    }
}
