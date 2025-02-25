// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root, 0)
    }

    pub fn dfs(node_option: Option<Rc<RefCell<TreeNode>>>, level: i32) -> i32 {
        if let Some(node) = node_option {
            let node_borrowed = node.borrow();
            return Self::dfs(node_borrowed.left.clone(), level + 1).max(Self::dfs(node_borrowed.right.clone(), level + 1))
        }
        level
    }
}
