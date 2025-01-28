use std::collections::HashMap;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m: usize = mat.len();
        let n: usize = mat[0].len();
        let mut hashmap: HashMap<i32, (usize, usize)> = HashMap::with_capacity(m * n);
        let mut rows_counter: Vec<usize> = vec![0; m];
        let mut column_counter: Vec<usize> = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                hashmap.insert(mat[i][j], (i, j));
            }
        }
        for index in 0..(m * n) {
            let (i, j): (usize, usize) = hashmap[&arr[index]];
            rows_counter[i] += 1;
            column_counter[j] += 1;
            if rows_counter[i] == n || column_counter[j] == m {
                return index as i32;
            }
        }
        0
    }
}
