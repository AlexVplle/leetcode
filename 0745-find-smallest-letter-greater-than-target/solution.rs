impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        match letters.binary_search(&target) {
            Ok(index) => Self::check_bounding(letters, index + 1),
            Err(index) => Self::check_bounding(letters, index)
        }
    }

    pub fn check_bounding(letters: Vec<char>, index: usize) -> char {
        if index < letters.len() {
                letters[index]
            } else {
                letters[0]
            }
    }
}
