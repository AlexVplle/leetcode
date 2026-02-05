impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        
        for i in 0..n {
            let step = nums[i];
            let new_index = ((i as i32 + step).rem_euclid(n as i32)) as usize;
            result[i] = nums[new_index];
        }
        result
    }
}
