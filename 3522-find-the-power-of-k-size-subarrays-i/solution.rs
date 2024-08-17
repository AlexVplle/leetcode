impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k_usize: usize = k as usize;
        let len_results: usize = nums.len() - k_usize + 1;
        let mut results: Vec<i32> = vec![0; len_results];
        for i in 0..len_results {
            let mut subarray = &nums[i..i + k_usize];
            let mut contigious: bool = true;
            for j in 1..k_usize {
                if subarray[j] != subarray[j - 1] + 1 {
                    contigious = false;
                }
            }
            if (contigious) {
                results[i] = subarray[k_usize - 1];
            } else {
                results[i] = -1;
            }
        }
        results
    }
}
