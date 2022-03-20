class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        setNums = sorted(set(nums))
        setNumsLength = len(setNums)
        for index, value in enumerate(setNums) : nums[index] = value
        return setNumsLength
        
