import "fmt"

func singleNumber(nums []int) int {
    var result int32 = 0
    for i := 0; i < 32; i++ {
        number_ones := 0
        for _, value := range nums {
            if value & (1 << i) != 0 {
                number_ones += 1
            }
        }
        if number_ones % 3 != 0 {
            result |= 1 << i
        }
    }
    return int(result)
}
