impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut results = Vec::with_capacity((1..nums.len()).product());
        Self::backtrack(&mut nums, 0, &mut results);
        results
    }

    fn backtrack(nums: &mut Vec<i32>, start: usize, results: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            results.push(nums.clone());
            return;
        }
        for i in start..nums.len() {
            nums.swap(start, i);
            Self::backtrack(nums, start + 1, results);
            nums.swap(start, i);
        }
    }
}
