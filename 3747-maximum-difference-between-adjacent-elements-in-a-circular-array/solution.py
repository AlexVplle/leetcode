class Solution(object):
    def maxAdjacentDistance(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        max_val = 0
        n = len(nums)
        for i in range(n):
            max_val = max(max_val, abs(nums[i] - nums[(i + 1) % n]))
        return max_val
