class Solution:
    def fib(self, n: int) -> int:
        if n < 2:
            return n
        memoization = [-1] * (n + 1)
        memoization[0] = 0
        memoization[1] = 1
        return self.recursion(n, memoization)

    def recursion(self, n, memoization):
        if memoization[n] != -1:
            return memoization[n]
        memoization[n] = self.recursion(n - 1, memoization) + self.recursion(n - 2, memoization)
        return memoization[n]
