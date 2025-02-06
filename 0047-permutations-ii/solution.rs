use std::collections::HashSet;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n: usize = nums.len();
        let mut result: Vec<Vec<i32>> = Vec::with_capacity((1..n).product());
        Self::backtrack(&mut nums, 0, &mut result);
        result
    }

    pub fn backtrack(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
        let n: usize = nums.len();
        if start == n {
            result.push(nums.clone());
            return;
        }
        let mut hashset: HashSet<i32> = HashSet::with_capacity(n - start);
        for index in start..n {
            if hashset.insert(nums[index]) {
                nums.swap(start, index);
                Self::backtrack(nums, start + 1, result);
                nums.swap(start, index);
            }
        }
    }
}
