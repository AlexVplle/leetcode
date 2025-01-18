impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        if nums2.len() & 1 == 1 {
            for &num in &nums1 {
                result ^= num;
            }
        }
        if nums1.len() & 1 == 1 {
            for num in nums2 {
                result ^= num;
            }
        }
        result
    }
}
