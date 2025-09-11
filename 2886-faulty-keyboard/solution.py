from collections import deque

class Solution:
    def finalString(self, s: str) -> str:
        result = deque()
        reverse = False
        for c in s:
            if c == 'i':
                reverse = not reverse
            else:
                if reverse:
                    result.appendleft(c)
                else:
                    result.append(c)
        return ''.join(result if not reverse else reversed(result))
