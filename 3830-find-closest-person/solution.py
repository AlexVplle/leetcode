class Solution:
    def findClosest(self, x: int, y: int, z: int) -> int:
        xz, yz = abs(x - z), abs(y - z)
        if xz == yz:
            return 0
        if xz < yz:
            return 1
        return 2
