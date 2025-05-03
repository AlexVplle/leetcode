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
use std::collections::HashSet;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut hashset: HashSet<i32> = HashSet::new();
        Self::dfs(root, k, &mut hashset)
    }

    pub fn dfs(node_option: Option<Rc<RefCell<TreeNode>>>, k: i32, mut hashset: &mut HashSet<i32>) -> bool {
        if let Some(node) = node_option {
            let borrowed_node = node.borrow();
            if hashset.contains(&(k - borrowed_node.val)) {
                return true;
            }
            hashset.insert(borrowed_node.val);
            return Self::dfs(borrowed_node.left.clone(), k, &mut hashset) || Self::dfs(borrowed_node.right.clone(), k, &mut hashset);
        }
        false
    }
}
