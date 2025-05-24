class Solution:
    def findDisappearedNumbers(self, nums: List[int]) -> List[int]:
        result = [i + 1 for i in range(len(nums))]
        for num in nums:
            result[num - 1] = 0
        return [x for x in result if x != 0]
