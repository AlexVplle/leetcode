impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let len_s: usize = s.len();
        if len_s < 2 || len_s % 2 == 1 {
            return false;
        }
        let s: Vec<char> = s.chars().collect();
        let locked: Vec<char> = locked.chars().collect();
        let mut balance: i32 = 0;
        for index in 0..len_s {
            if locked[index] == '0' || s[index] == '(' {
                balance += 1;
            } else {
                balance -= 1;
                if balance < 0 {
                    return false;
                }
            }
        }
        balance = 0;
        for index in (0..len_s).rev() {
            if locked[index] == '0' || s[index] == ')' {
                balance += 1;
            } else {
                balance -= 1;
                if balance < 0 {
                    return false;
                }
            }
        }
        true
    }
}
