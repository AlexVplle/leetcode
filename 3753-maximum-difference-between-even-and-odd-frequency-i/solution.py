from collections import Counter

class Solution:
    def maxDifference(self, s: str) -> int:
        counter = Counter(s)
        counter_values = counter.values()
        return max([i for i in counter_values if i & 1 == 1]) - min([i for i in counter_values if i & 1 == 0])
