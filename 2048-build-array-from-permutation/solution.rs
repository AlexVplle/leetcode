impl Solution {
    pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();
        for i in 0..n {
            nums[i] += n as i32 * (nums[nums[i] as usize] % n as i32);
        }
        for i in 0..n {
            nums[i] /= n as i32;
        }
        nums
    }
}
