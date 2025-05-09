from collections import Counter
from math import comb
from functools import cache
class Solution:
    def countBalancedPermutations(self, num: str) -> int:
        n = len(num)
        total = sum(int(x) for x in num)
        if total % 2 == 1:
            return 0

        counter = Counter(int(x) for x in num)

        @cache
        def dfs(digit: int, odd: int, even: int, balance: int) -> int:
            if odd == 0 and even == 0 and balance == 0:
                return 1
            
            if digit < 0 or odd < 0 or even < 0 or balance < 0:
                return 0

            count = counter[digit]
            res= 0
            for k in range(count + 1):
                res += comb(odd, k) * comb(even, count - k) * dfs(digit - 1, odd - k, even - (count - k), balance - k * digit)

            return res % 1000000007

        return dfs(9, n - n // 2, n // 2, total // 2)
