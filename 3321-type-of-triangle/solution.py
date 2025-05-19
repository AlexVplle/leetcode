class Solution:
    def triangleType(self, nums: List[int]) -> str:
        max_value = max(nums)
        if sum(nums) - max_value <= max_value:
            return "none"
        return ["equilateral", "isosceles", "scalene"][len(set(nums)) - 1]
