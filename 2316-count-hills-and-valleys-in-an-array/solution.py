class Solution:
    def countHillValley(self, nums: List[int]) -> int:
        result = 0
        left = nums[0]
        for i in range(1, len(nums) - 1):
            middle = nums[i]
            right = nums[i + 1]
            if middle == right:
                continue
            if (left < middle and middle > right) or (left > middle and middle < right):
                result += 1
            left = middle
        return result
