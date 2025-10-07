func isBalanced(root *TreeNode) bool {
    return height(root) != -1
}

func height(node *TreeNode) int {
    if node == nil {
        return 0
    }
    
    leftHeight := height(node.Left)
    if leftHeight == -1 {
        return -1
    }
    
    rightHeight := height(node.Right)
    if rightHeight == -1 {
        return -1
    }
    
    diff := leftHeight - rightHeight
    if diff > 1 || diff < -1 {
        return -1
    }
    
    return max(leftHeight, rightHeight) + 1
}
