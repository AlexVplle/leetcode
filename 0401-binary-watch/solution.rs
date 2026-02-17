impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut result = vec![];
        for hour in 0u32..12 {
            for minutes in 0u32..60 {
                if (hour.count_ones() + minutes.count_ones()) == turned_on as u32 {
                    result.push(format!("{}:{:02}", hour, minutes));
                }
            }
        }
        result
    }
}
