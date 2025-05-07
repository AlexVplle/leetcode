import heapq

class Solution:
    def minTimeToReach(self, moveTime: List[List[int]]) -> int:
        row = len(moveTime)
        col = len(moveTime[0])
        distances = [[float('inf') for _ in range(col)] for _ in range(row)]
        directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        priority_queue = [(0, 0, 0)]
        while priority_queue:
            time, x, y  = heapq.heappop(priority_queue)
            if x == row - 1 and y == col - 1:
                return time
            if moveTime[x][y] == -1:
                continue
            moveTime[x][y] = -1
            for dx, dy in directions:
                new_x, new_y = x + dx, y + dy
                if 0 <= new_x < row and 0 <= new_y < col and moveTime[new_x][new_y] != -1:
                    time_n = max(moveTime[new_x][new_y], time)
                    if distances[new_x][new_y] > time_n + 1:
                        distances[new_x][new_y] = time_n + 1
                        heapq.heappush(priority_queue, (time_n + 1, new_x, new_y))
        return distances[row - 1][col - 1]
            
