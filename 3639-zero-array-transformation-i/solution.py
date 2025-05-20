class Solution:
    def isZeroArray(self, nums: List[int], queries: List[List[int]]) -> bool:
        n = len(nums)
        decrement_count = [0] * n
        for left_bound, right_bound in queries:
            decrement_count[left_bound] += 1
            if right_bound + 1 < n:
                decrement_count[right_bound + 1] -= 1
        for i in range(1, n):
            decrement_count[i] += decrement_count[i - 1]
        for i in range(n):
            if nums[i] > decrement_count[i]:
                return False
        return True

