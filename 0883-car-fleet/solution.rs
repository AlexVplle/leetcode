impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = position.len();
        let mut cars: Vec<(i32, i32)> = Vec::with_capacity(n);
        
        for i in 0..n {
            cars.push((position[i], speed[i]));
        }
        
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));  // Sort in descending order of position
        
        let mut stack: Vec<f64> = Vec::with_capacity(n);
        
        for (pos, speed) in cars {
            let time = (target - pos) as f64 / speed as f64;
            if stack.is_empty() || time > *stack.last().unwrap() {
                stack.push(time);
            }
        }
        
        stack.len() as i32
    }
}
