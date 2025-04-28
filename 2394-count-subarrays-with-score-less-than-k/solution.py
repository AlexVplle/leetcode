class Solution:
    def countSubarrays(self, nums: List[int], k: int) -> int:
        result = 0
        sum = 0
        left = 0
        for right in range(len(nums)):
            sum += nums[right]
            while sum * (right - left + 1) >= k:
                sum -= nums[left]
                left += 1
            result += (right - left + 1)
        return result

