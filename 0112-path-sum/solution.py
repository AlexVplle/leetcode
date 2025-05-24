# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if not root:
            return False
        def dfs(node, sum):
            sum += node.val
            if sum == targetSum and not node.left and not node.right:
                return True
            result = False
            if node.left:
                result = result or dfs(node.left, sum)
            if node.right:
                result = result or dfs(node.right, sum)
            return result
        return dfs(root, 0)
