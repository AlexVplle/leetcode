use std::mem::swap;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut index: usize = 0;
        while index < n {
            let value: i32 = nums[index];
            if value >= 1 && value <= n as i32 && value != nums[value as usize - 1] {
                nums.swap(index, value as usize - 1);
            } else {
                index += 1;
            }
        }
        for index in 0..n {
            if nums[index] != index as i32 + 1 {
                return index as i32 + 1;
            }
        }
        n as i32 + 1
    }
}
