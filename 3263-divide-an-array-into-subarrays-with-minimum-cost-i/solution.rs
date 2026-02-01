impl Solution {
    pub fn minimum_cost(mut nums: Vec<i32>) -> i32 {
        let result: i32 = nums[0];
        let nums_slice: &mut [i32] = &mut nums[1..];
        nums_slice.select_nth_unstable(1);
        result + nums_slice[0] + nums_slice[1]
    }
}
