impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, mut k: i32) -> i32 {
        let n: usize = arr.len();
        if k > arr[n - 1] {
            return k + n as i32;
        }
        let mut index: usize = 0;
        let mut i: i32 = 1;
        while index < n {
            if arr[index] != i {
                k -= 1;
                if k == 0 {
                    return i;
                }
            } else {
                index += 1;
            }
            i += 1;
        }
        arr[n - 1] + k
    }
}
