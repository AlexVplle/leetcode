impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n: u16 = n as u16;
        let mut total: i32 = 0;
        let half: u16 = n / 2;
        for x in 0..half {
            let mut cols: u16 = 0;
            let mut right_diagonals: u16 = 0;
            let mut left_diagonals: u16 = 0;
            cols |= 1 << x;
            left_diagonals |= 1 << x;
            right_diagonals |= 1 << (n - x - 1);
            let solutions = Self::backtrack(1, &mut cols, &mut right_diagonals, &mut left_diagonals, n);
            total += solutions * 2;
        }

        if n % 2 == 1 {
            let x = half;
            let mut cols: u16 = 0;
            let mut right_diagonals: u16 = 0;
            let mut left_diagonals: u16 = 0;
            cols |= 1 << x;
            left_diagonals |= 1 << (x + 0);
            right_diagonals |= 1 << (n - x - 1 + 0);
            total += Self::backtrack(1, &mut cols, &mut right_diagonals, &mut left_diagonals, n);
        }
        total
    }

    pub fn backtrack(
        y: u16,
        cols: &mut u16,
        right_diagonals: &mut u16,
        left_diagonals: &mut u16,
        n: u16,
    ) -> i32 {
        if y == n {
            return 1;
        }
        let mut sum: i32 = 0;
        for x in 0..n {
            if (*cols & (1 << x))
                + (*left_diagonals & (1 << (x + y)))
                + (*right_diagonals & (1 << (n - x - 1 + y)))
                == 0
            {
                *cols |= 1 << x;
                *left_diagonals |= 1 << (x + y);
                *right_diagonals |= 1 << (n - x - 1 + y);
                sum += Self::backtrack(y + 1, cols, right_diagonals, left_diagonals, n);
                *cols &= !(1 << x);
                *left_diagonals &= !(1 << (x + y));
                *right_diagonals &= !(1 << (n - x - 1 + y));
            }
        }
        sum
    }
}
