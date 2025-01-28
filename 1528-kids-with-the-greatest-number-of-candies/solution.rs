impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let maximum_value: i32 = *candies.iter().max().unwrap() - extra_candies;
        candies
            .into_iter()
            .map(|candie: i32| candie >= maximum_value)
            .collect()
    }
}
