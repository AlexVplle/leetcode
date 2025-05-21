class Solution:
    def setZeroes(self, matrix: List[List[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """
        self.recurrence(matrix, (0, 0), (len(matrix), len(matrix[0])))

    def recurrence(self, matrix, indexes, lengths):
        i, j = indexes
        m, n = lengths
        if i == m:
            return
        value = matrix[i][j]
        if value == 0:
            for k in range(j):
                matrix[i][k] = 0
            for k in range(i):
                matrix[k][j] = 0
        if j + 1 == n:
            self.recurrence(matrix, (i + 1, 0), (m, n))
        else:
            self.recurrence(matrix, (i, j + 1), (m, n))
        if value == 0:
            for k in range(j + 1, n):
                matrix[i][k] = 0
            for k in range(i + 1, m):
                matrix[k][j] = 0
        
