class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        m, n = len(board), len(board[0])
        for y in range(m):
            for x in range(n):
                visited = set()
                if self.dfs((x, y), word, (m, n), board, visited):
                    return True
        return False
    
    def dfs(self, indexes, word, lengths, board, visited):
        x, y = indexes
        if board[y][x] != word[0]:
            return False
        if len(word) == 1:
            return True
        m, n = lengths
        visited.add((x, y))
        directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        for delta_x, delta_y in directions:
            x_prime, y_prime = x + delta_x, y + delta_y
            if (x_prime, y_prime) in visited:
                continue
            if 0 <= y_prime < m and 0 <= x_prime < n and self.dfs((x_prime, y_prime), word[1:], lengths, board, visited):
                visited.remove((x, y))
                return True
        visited.remove((x, y))
        return False
