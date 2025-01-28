use std::collections::HashSet;

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n: usize = matrix.len();
        for i in 0..n {
            let mut hashset: HashSet<&i32> = HashSet::from_iter(matrix[i].iter());
            if hashset.len() != n {
                return false;
            }
            hashset.clear();
            for j in 0..n {
                if !hashset.insert(&matrix[j][i]) {
                    return false;
                }
            }
        }
        true
    }
}
