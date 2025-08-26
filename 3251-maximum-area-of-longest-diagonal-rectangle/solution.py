from math import sqrt

class Solution:
    def areaOfMaxDiagonal(self, dimensions: List[List[int]]) -> int:
        max_diagonal, max_area = 0, 0
        for length, width in dimensions:
            value = length * length + width * width
            if max_diagonal < value:
                max_diagonal = value
                max_area = length * width
            elif max_diagonal == value:
                max_area = max(max_area, length * width)
        return max_area
