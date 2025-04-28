class Solution:
    def numSubarrayProductLessThanK(self, nums: List[int], k: int) -> int:
        if k == 0 or k == 1:
            return 0
        result = 0
        sum = 1
        left = 0
        for right in range(len(nums)):
            sum *= nums[right]
            while sum >= k:
                sum /= nums[left]
                left += 1
            result += right - left + 1
        return result
