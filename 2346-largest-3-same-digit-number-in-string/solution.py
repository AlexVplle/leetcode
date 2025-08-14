class Solution:
    def largestGoodInteger(self, num: str) -> str:
        n = len(num)
        if n < 3:
            return ""
        max_digit = -1
        for i in range(n - 2):
            if num[i] == num[i + 1] == num[i + 2]:
                digit = int(num[i])
                if digit > max_digit:
                    max_digit = digit
        return str(max_digit) * 3 if max_digit != -1 else ""
