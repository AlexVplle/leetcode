use std::collections::HashMap;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut frequency_needed: [i32; 26] = [0; 26];
        for word2 in words2 {
            let mut frequency2: [i32; 26] = [0; 26];
            for c in word2.chars() {
                frequency2[(c as u8 - b'a') as usize] += 1;
            }
            for i in 0..26 {
                frequency_needed[i] = frequency_needed[i].max(frequency2[i]);
            }
        }
        let mut result: Vec<String> = Vec::with_capacity(words1.len());
        'outer: for word1 in words1 {
            let mut frequency1: [i32; 26] = [0; 26];
            for c in word1.chars() {
                frequency1[(c as u8 - b'a') as usize] += 1;
            }
            for i in 0..26 {
                if frequency1[i] < frequency_needed[i] {
                    continue 'outer;
                }
            }
            result.push(word1);
        }

        result
    }
}
