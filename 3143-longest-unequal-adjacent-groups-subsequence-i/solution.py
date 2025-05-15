class Solution:
    def getLongestSubsequence(self, words: List[str], groups: List[int]) -> List[str]:
        if not words:
            return []
        n = len(words)
        dp = [[0, 0] for _ in range(n)]
        prev = [[-1, -1] for _ in range(n)]
        dp[0][groups[0]] = 1
        for i in range(1, n):
            dp[i][0] = dp[i - 1][0]
            dp[i][1] = dp[i - 1][1]
            prev[i][0] = prev[i - 1][0]
            prev[i][1] = prev[i - 1][1]
            current_group = groups[i]
            other_group = 1 - current_group
            if dp[i - 1][other_group] + 1 > dp[i][current_group]:
                dp[i][current_group] = dp[i - 1][other_group] + 1
                prev[i][current_group] = i - 1
        final_group = 0 if dp[n - 1][0] > dp[n - 1][1] else 1
        result = []
        current_index = n - 1
        current_group = final_group
        while current_index >= 0 and dp[current_index][current_group] > 0:
            if groups[current_index] == current_group:
                result.append(words[current_index])
                previous_index = prev[current_index][current_group]
                if previous_index != -1:
                    current_group = groups[previous_index]
                current_index = previous_index
            else:
                current_index = -1
        return result[::-1]
