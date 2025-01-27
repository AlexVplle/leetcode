impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();
        nums.into_iter().cycle().take(n * 2).collect()
    }
}
