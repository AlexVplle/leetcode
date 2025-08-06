class Solution:
    def numOfUnplacedFruits(self, fruits: List[int], baskets: List[int]) -> int:
        n = len(fruits)
        result = 0
        array = [False for i in range(n)]
        for i in range(n):
            found = False
            for j in range(n):
                if not array[j] and fruits[i] <= baskets[j]:
                    array[j] = True
                    found = True
                    break
            if not found:
                result += 1
        return result
