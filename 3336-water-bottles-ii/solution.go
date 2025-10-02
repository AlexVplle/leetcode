import "fmt"

func maxBottlesDrunk(numBottles int, numExchange int) int {
    emptyBottles := numBottles
    bottlesDrunk := numBottles
    for emptyBottles >= numExchange {
        emptyBottles -= numExchange
        numExchange += 1
        bottlesDrunk += 1
        emptyBottles += 1
    }
    return bottlesDrunk
}
