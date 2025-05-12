
class Solution:
    def findEvenNumbers(self, digits: List[int]) -> List[int]:
        result = []
        frequencies = [0] * 10
        for digit in digits:
            frequencies[digit] += 1
        for i in range(1, 10):
            if frequencies[i] == 0:
                continue
            frequencies[i] -= 1
            for j in range(10):
                if frequencies[j] == 0:
                    continue
                frequencies[j] -= 1
                for k in range(0, 10, 2):
                    if frequencies[k] == 0:
                        continue
                    result.append(i * 100 + j * 10 + k)
                frequencies[j] += 1
            frequencies[i] += 1
        return result
