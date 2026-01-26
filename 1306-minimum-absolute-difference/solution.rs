impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let mut result: Vec<Vec<i32>> = vec![vec![]];
        let mut min_difference: i32 = i32::MAX;
        for i in 0..arr.len() - 1 {
            let difference: i32 = arr[i + 1] - arr[i];
            if difference < min_difference {
                min_difference = difference;
                result = vec![vec![arr[i], arr[i + 1]]];
            } else if difference == min_difference {
                result.push(vec![arr[i], arr[i + 1]])
            }
        }
        result
    }
}
