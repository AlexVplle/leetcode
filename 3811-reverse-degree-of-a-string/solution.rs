impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.char_indices()
            .map(|(index, value): (usize, char)| (value as i32 - 123).abs() * (index as i32 + 1))
            .sum()
    }
}
