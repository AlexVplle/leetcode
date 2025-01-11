
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if k > s.len() as i32 {
            return false;
        }
        let mut frequence_map: HashMap<char, usize> = HashMap::with_capacity(26);
        for c in s.chars() {
            *frequence_map.entry(c).or_insert(0) += 1;
        }
        frequence_map
            .values()
            .filter(|&&count: &&usize| count % 2 != 0)
            .count() as i32
            <= k
    }
}
