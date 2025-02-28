// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head_result = None;
        let mut current = &mut head_result;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum = carry
                + l1.as_ref().map_or(0, |node| node.val)
                + l2.as_ref().map_or(0, |node| node.val);

            carry = sum / 10;
            let new_node = Some(Box::new(ListNode::new(sum % 10)));
            *current = new_node;
            if let Some(ref mut node) = current {
                current = &mut node.next;
            }
            if let Some(node) = l1 {
                l1 = node.next;
            } else {
                l1 = None;
            }
            if let Some(node) = l2 {
                l2 = node.next;
            } else {
                l2 = None;
            }
        }
        head_result
    }
}
