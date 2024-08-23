// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut head_mut: Option<Box<ListNode>> = head;
        let mut vector: Vec<i32> = Vec::new();
        let mut stack: Vec<i32> = Vec::new();
        while let Some(node) = head_mut {
            vector.push(node.val);
            head_mut = node.next;
        }
        let mut result: Vec<i32> = vec![0; vector.len()];
        vector.into_iter().enumerate().rev().for_each(|(index, value): (usize, i32)| {
            while let Some(&top) = stack.last() {
                if top > value {
                    result[index] = top;
                    break;
                } else {
                    stack.pop();
                }
            }
            stack.push(value);
        });
        result
    }
}
