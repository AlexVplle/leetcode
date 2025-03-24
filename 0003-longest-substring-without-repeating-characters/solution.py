class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        last_seen = [None] * 128  # Assuming ASCII
        result = 0
        start = 0

        for index, char in enumerate(s):
            prev_index = last_seen[ord(char)]
            if prev_index is not None and prev_index >= start:
                start = prev_index + 1
            last_seen[ord(char)] = index
            result = max(result, index - start + 1)

        return result


