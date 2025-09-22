from collections import Counter

class Solution:
    def maxFrequencyElements(self, nums: List[int]) -> int:
        frequencies = list(Counter(nums).values())
        max_value = max(frequencies)
        return max_value * frequencies.count(max_value)
