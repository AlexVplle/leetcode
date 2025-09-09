class Solution:
    def peopleAwareOfSecret(self, n: int, delay: int, forget: int) -> int:
        MOD = 10**9 + 7
        dp = [0] * (n + 1)
        dp[1] = 1
        for day in range(2, n + 1):
            spreaders = 0
            for learn_day in range(max(1, day - forget + 1), day - delay + 1):
                spreaders = (spreaders + dp[learn_day]) % MOD
            dp[day] = spreaders
        total = 0
        for learn_day in range(max(1, n - forget + 1), n + 1):
            total = (total + dp[learn_day]) % MOD
        return total
