impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut i = 0;
        let mut state = None;
        while i < nums.len() - 1 {
            let difference = nums[i + 1] - nums[i];
            if difference != 0 {
                match state {
                    Some(previous_difference) => {
                        if previous_difference * difference < 0 {
                            return false;
                        }
                    },
                    None => state = Some(difference)
                }
            }
            i += 1
        }
        true
    }
}
