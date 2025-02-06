impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n: usize = nums.len();
        let mut result: Vec<Vec<i32>> = vec![];
        for bitmask in 0..(1 << n) {
            let mut subset: Vec<i32> = vec![];
            for bit in 0..n {
                if bitmask >> bit & 1 == 1 {
                    subset.push(nums[bit]);
                }
            }
            result.push(subset);
        }
        result
    }
}
