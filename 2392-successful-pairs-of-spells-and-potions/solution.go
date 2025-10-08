func successfulPairs(spells []int, potions []int, success int64) []int {
	sort.Slice(potions, func(i, j int) bool { return potions[i] < potions[j] })
	var result []int
	for _, spell := range spells {
		result = append(result, len(potions)-sort.Search(len(potions), func(i int) bool {
			return potions[i] >= int(math.Ceil(float64(success)/float64(spell)))
		}))
	}
	return result
}
