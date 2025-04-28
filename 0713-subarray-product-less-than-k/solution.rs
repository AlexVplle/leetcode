impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k < 2 {
            return 0;
        }
        let mut result: i32 = 0;
        let mut sum: i32 = 1;
        let mut left: usize = 0;
        for right in 0..nums.len() {
            sum *= nums[right];
            while sum >= k {
                sum /= nums[left];
                left += 1;
            }
            result += (right - left + 1) as i32;
        }
        result
    }
}
