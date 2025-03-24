# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next


class Solution:
    def addTwoNumbers(
        self, l1: Optional[ListNode], l2: Optional[ListNode]
    ) -> Optional[ListNode]:
        result = None
        current = None
        carry = 0
        while l1 or l2 or carry != 0:
            sum = carry
            if l1:
                sum += l1.val
                l1 = l1.next
            if l2:
                sum += l2.val
                l2 = l2.next
            carry = sum // 10
            new_node = ListNode(sum % 10)
            if not result:
                result = new_node
            else:
                current.next = new_node
            current = new_node
        return result


