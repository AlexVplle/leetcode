func singleNumber(nums []int) []int {
    xor_combination := 0
    for _, value := range nums {
        xor_combination ^= value
    }
    lb := xor_combination & -xor_combination
    a := 0
    for _, value := range nums {
        if value & lb != 0 {
            a ^= value
        }
    }
    return []int{a, xor_combination ^ a}
}
