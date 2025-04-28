impl Solution {
    pub fn count_subarrays(mut nums: Vec<i32>, k: i64) -> i64 {
        let mut result: i64 = 0;
        let mut sum: i64 = 0;
        let mut left: usize = 0;
        for right in 0 ..nums.len() {
            sum += nums[right] as i64;
            while sum * (right - left + 1) as i64 >= k {
                sum -= nums[left] as i64;
                left += 1;
            }
            result += (right - left + 1) as i64;
        }
        result
    }
}
