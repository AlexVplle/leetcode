impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;
        let n: usize = mat.len();
        let mid: usize = n / 2;
        for i in 0..mid {
            let other_i: usize = n - i - 1;
            result += mat[i][i] + mat[i][other_i] + mat[other_i][i] + mat[other_i][other_i];
        }
        if n & 1 != 0 {
            result += mat[mid][mid];
        }
        result
    }
}
