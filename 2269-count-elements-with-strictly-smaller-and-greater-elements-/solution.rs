impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let (min, max) = nums.iter()
            .fold((i32::MAX, i32::MIN), |(min, max), &num| {
                (min.min(num), max.max(num))
            });
        
        nums.iter()
            .filter(|&&num| num > min && num < max)
            .count() as i32
    }
}
