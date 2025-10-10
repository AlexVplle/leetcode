import "slices"

func maximumEnergy(energy []int, k int) int {
    maxEnergyFrom := make([]int, len(energy))
    for i := len(energy) - 1; i >= 0; i-- {
        if i + k < len(energy) {
            maxEnergyFrom[i] = maxEnergyFrom[i + k] + energy[i]
        } else {
            maxEnergyFrom[i] = energy[i]
        }
    }
    return slices.Max(maxEnergyFrom)
}
