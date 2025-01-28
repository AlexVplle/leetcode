impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n): (usize, usize) = (grid.len(), grid[0].len());
        let mut row_count_one: Vec<i32> = vec![0; m];
        let mut column_count_one: Vec<i32> = vec![0; n];
        let mut result: Vec<Vec<i32>> = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    row_count_one[i] += 2;
                    column_count_one[j] += 2;
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                result[i][j] = row_count_one[i] + column_count_one[j] - n as i32 - m as i32;
            }
        }
        result
    }
}
