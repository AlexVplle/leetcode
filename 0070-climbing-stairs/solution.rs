use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut hashmap: HashMap<i32, i32> = HashMap::with_capacity(45);
        Self::backtrack(&mut hashmap, n)
    }

    pub fn backtrack(hashmap: &mut HashMap<i32, i32>, n: i32) -> i32 {
        if n < 0 {
            return 0;
        }
        if n == 0 {
            return 1;
        }
        if hashmap.contains_key(&n) {
            return hashmap[&n];
        }
        let count: i32 = Self::backtrack(hashmap, n - 2) + Self::backtrack(hashmap, n - 1);
        hashmap.insert(n, count);
        count
    }
}
