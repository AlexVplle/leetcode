impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut right = 0;
        let mut result = nums.len();
        nums.sort_unstable();
        for left in 0..nums.len() {
            while right < nums.len() && (k as i64) * (nums[left] as i64) >= nums[right] as i64 {
                right += 1;
            }
            result = result.min(nums.len() - (right - left));
        }
        result as i32
    }
}
