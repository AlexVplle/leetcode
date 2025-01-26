impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();
        let mut xor: i32 = 0;
        for index in 0..n {
            xor ^= nums[index] ^ (index as i32 + 1);
        }
        let set_bit: i32 = xor & -xor;
        let (mut x_xor, mut y_xor): (i32, i32) = (0, 0);
        for index in 0..n {
            let integer: i32 = index as i32 + 1;
            if nums[index] & set_bit == 0 {
                x_xor ^= nums[index];
            } else {
                y_xor ^= nums[index];
            }
            if integer & set_bit == 0 {
                x_xor ^= integer;
            } else {
                y_xor ^= integer;
            }
        }
        for value in nums {
            if value == x_xor {
                return vec![x_xor, y_xor];
            }
        }
        vec![y_xor, x_xor]
    }
}
