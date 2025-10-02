func findTheDifference(s string, t string) byte {
    var result byte
    for i := range s {
        result ^= s[i] ^ t[i]
    }
    result ^= t[len(t) - 1]
    return result
}
