from collections import Counter

class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        s1_len = len(s1)
        s2_len = len(s2)
        if s1_len > s2_len:
            return False
        s1_count = Counter(s1)
        window_count = Counter()
        for i in range(s1_len):
            window_count[s2[i]] += 1
        if window_count == s1_count:
            return True
        for i in range(s1_len, s2_len):
            window_count[s2[i]] += 1
            left_char = s2[i - s1_len]
            window_count[left_char] -= 1
            if window_count[left_char] == 0:
                del window_count[left_char]
            if window_count == s1_count:
                return True
        return False
