class Solution:
    def maxSum(self, nums: List[int]) -> int:
        max_negative = -101
        result = 0
        positive_seen = [False] * 100
        for num in nums:
            if num > 0:
                if not positive_seen[num - 1]:
                    positive_seen[num - 1] = True
                    result += num
            else:
                max_negative = max(max_negative, num)
        return result if result > 0 else max_negative

        
