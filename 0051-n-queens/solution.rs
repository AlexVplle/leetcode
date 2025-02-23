impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n: u32 = n as u32;
        let full: u32 = (1 << n) - 1;
        let mut result: Vec<Vec<String>> = vec![];
        let mut solution: Vec<String> = vec![];
        let mut rows: Vec<String> = Vec::with_capacity(n as usize);
        for i in 0..n {
            let mut row = String::with_capacity(n as usize);
            for j in 0..n {
                if i == j {
                    row.push('Q');
                } else {
                    row.push('.');
                }
            }
            rows.push(row);
        }
        Self::backtrack(0, 0, 0, n, full, &rows, &mut result, &mut solution);
        result
    }

    fn backtrack(
        cols: u32,
        left_diagonals: u32,
        right_diagonals: u32,
        n: u32,
        full: u32,
        rows: &Vec<String>,
        result: &mut Vec<Vec<String>>,
        solution: &mut Vec<String>,
    ) {
        if cols == full {
            result.push(solution.clone());
            return;
        }
        let mut available: u32 = (!(cols | left_diagonals | right_diagonals)) & full;
        while available != 0 {
            let position: u32 = available & (!available + 1);
            available &= available - 1;
            let col_index: u32 = position.trailing_zeros();
            solution.push(rows[col_index as usize].clone());
            Self::backtrack(
                cols | position,
                (left_diagonals | position) << 1,
                (right_diagonals | position) >> 1,
                n,
                full,
                rows,
                result,
                solution,
            );
            solution.pop();
        }
    }
}
