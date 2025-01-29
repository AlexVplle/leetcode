use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n): (usize, usize) = (mat.len(), mat[0].len());
        let mut result: Vec<Vec<i32>> = vec![vec![-1; n]; m];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(n * m);
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    result[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }
        while let Some((x, y)) = queue.pop_front() {
            for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (x_prime, y_prime): (i32, i32) = (x as i32 + i, y as i32 + j);
                if x_prime >= 0
                    && x_prime < m as i32
                    && y_prime >= 0
                    && y_prime < n as i32
                    && result[x_prime as usize][y_prime as usize] == -1
                {
                    let (x_prime, y_prime): (usize, usize) = (x_prime as usize, y_prime as usize);
                    result[x_prime][y_prime] = result[x][y] + 1;
                    queue.push_back((x_prime, y_prime));
                }
            }
        }
        result
    }
}
