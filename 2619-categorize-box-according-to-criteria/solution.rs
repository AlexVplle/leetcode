impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        match (
            mass >= 100,
            length >= 10_000
                || width >= 10_000
                || height >= 10_000
                || length >= 1_000_000_000 / width / height,
        ) {
            (true, true) => "Both".to_string(),
            (true, false) => "Heavy".to_string(),
            (false, true) => "Bulky".to_string(),
            _ => "Neither".to_string(),
        }
    }
}
