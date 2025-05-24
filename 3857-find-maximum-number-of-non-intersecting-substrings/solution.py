class Solution:
    def maxSubstrings(self, word: str) -> int:
        n = len(word)
        intervals = []
        for i in range(n):
            for j in range(i + 3, n):
                if word[i] == word[j]:
                    intervals.append((i, j))
                    break
        intervals.sort(key=lambda x: x[1])
        result = 0
        previous_end = -1
        for start, end in intervals:
            if start > previous_end:
                result += 1
                previous_end = end
        return result
