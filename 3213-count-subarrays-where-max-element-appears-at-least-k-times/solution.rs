impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut result: i64 = 0;
        let mut count_max: i32 = 0;
        let mut left: usize = 0;
        let max_element: i32 = *nums.iter().max().unwrap();
        for right in 0..nums.len() {
            if nums[right] == max_element {
                count_max += 1;
            }
            while count_max >= k {
                result += (nums.len() - right) as i64;
                if nums[left] == max_element {
                    count_max -= 1;
                }
                left += 1;
            }
        }
        result
    }
}
