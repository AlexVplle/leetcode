class Solution:
    def findNumbers(self, nums: List[int]) -> int:
        result = 0
        for val in nums:
            count_digits = 0
            while val != 0:
                count_digits += 1
                val //= 10
            if count_digits % 2 == 0:
                result += 1
        return result
        
