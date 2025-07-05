from collections import Counter

class Solution:
    def findLucky(self, arr: List[int]) -> int:
        counter = Counter(arr)
        return max((num for num, freq in counter.items() if num == freq), default=-1)
