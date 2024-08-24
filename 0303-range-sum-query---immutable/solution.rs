struct NumArray {
    prefix_sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix_sum: Vec<i32> = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i].saturating_add(nums[i]);
        }
        Self { prefix_sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix_sum[right as usize + 1] - self.prefix_sum[left as usize]
    }
}

