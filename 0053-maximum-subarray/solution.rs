impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut best: i32 = nums[0];
        let mut sum: i32 = 0;
        for index in 0..nums.len() {
            sum = nums[index].max(sum + nums[index]);
            best = best.max(sum);
        }
        best
    }
}
