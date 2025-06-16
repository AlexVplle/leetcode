class Solution:
    def maximumDifference(self, nums: List[int]) -> int:
        min_so_far = nums[0]
        result = -1
        for i in range(1, len(nums)):
            if nums[i] > min_so_far:
                result = max(result, nums[i] - min_so_far)
            min_so_far = min(min_so_far, nums[i])
        return result

