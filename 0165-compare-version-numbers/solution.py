from itertools import zip_longest


class Solution:
    def compareVersion(self, version1: str, version2: str) -> int:
        v1_parts, v2_parts = version1.split("."), version2.split(".")
        for part1, part2 in zip_longest(v1_parts, v2_parts, fillvalue="0"):
            val1, val2 = int(part1), int(part2)
            if val1 < val2:
                return -1
            elif val1 > val2:
                return 1
        return 0
