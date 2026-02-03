impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut p: usize = 0;
        while p < nums.len() - 1 && nums[p] < nums[p + 1] {
            p += 1;
        }
        if p == 0 || p >= nums.len() - 2 {
            return false;
        }
        let mut q: usize = p;
        while q < nums.len() - 1 && nums[q] > nums[q + 1] {
            q += 1;
        }
        if q == p || q >= nums.len() - 1 {
            return false;
        }
        for i in q..nums.len() - 1 {
            if nums[i] >= nums[i + 1] {
                return false;
            }
        }
        true
    }
}
