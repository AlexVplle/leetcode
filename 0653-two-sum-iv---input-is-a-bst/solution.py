# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def dfs(self, node: Optional[TreeNode], set_value, k):
        if node:
            if k - node.val in set_value:
                return True
            set_value.add(node.val)
            return self.dfs(node.left, set_value, k) or self.dfs(node.right, set_value, k)
        return False
    
    def findTarget(self, root: Optional[TreeNode], k: int) -> bool:
        set_value = set()
        return self.dfs(root, set_value, k)
    
    

