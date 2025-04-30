impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        for mut value in nums {
            let mut count_digits = 0;
            while value != 0 {
                value /= 10;
                count_digits += 1;
            }
            if count_digits % 2 == 0 {
                result += 1;
            }
        }
        result
    }
}
