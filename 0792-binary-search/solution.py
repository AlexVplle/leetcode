class Solution:
    def search(self, nums: List[int], target: int) -> int:
        if target not in nums : return -1
        end = len(nums) - 1
        mid = end // 2
        valueMid = nums[mid]
        if valueMid == target: return mid
        if valueMid > target : return self.search(nums[:mid], target)
        else : return mid + 1 + self.search(nums[mid+1:], target)
