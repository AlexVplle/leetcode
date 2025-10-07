func isBalanced(num string) bool {
    sum := 0
    for i, digit := range num {
        if i % 2 == 0 {
            sum += int(digit - '0')
        } else {
            sum -= int(digit - '0')
        }
    }
    return sum == 0
}
