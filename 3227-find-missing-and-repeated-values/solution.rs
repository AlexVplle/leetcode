use std::{collections::HashSet, vec};

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n: usize = grid.len();
        let mut xor: i32 = 0;
        for i in 0..n {
            for j in 0..n {
                xor ^= grid[i][j] ^ ((i * n + j + 1) as i32);
            }
        }
        let set_bit: i32 = xor & -xor;
        let (mut x_bucket, mut y_bucket): (i32, i32) = (0, 0);
        for i in 0..n {
            for j in 0..n {
                let index_flatten: i32 = (i * n + j + 1) as i32;
                if grid[i][j] & set_bit != 0 {
                    x_bucket ^= grid[i][j]
                } else {
                    y_bucket ^= grid[i][j]
                }
                if index_flatten & set_bit != 0 {
                    x_bucket ^= index_flatten;
                } else {
                    y_bucket ^= index_flatten;
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == x_bucket {
                    return vec![x_bucket, y_bucket];
                }
            }
        }
        vec![y_bucket, x_bucket]
    }
}
