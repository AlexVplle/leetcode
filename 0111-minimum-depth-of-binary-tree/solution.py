from collections import deque

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def minDepth(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0
        queue = deque([(1, root)])
        while queue:
            level, node = queue.popleft()
            if not node.left and not node.right:
                return level
            if node.left:
                queue.append((level + 1, node.left))
            if node.right:
                queue.append((level + 1, node.right))
