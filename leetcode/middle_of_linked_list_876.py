# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        node_count = 0
        node = head
        while node is not None:
            node_count += 1
            node = node.next
        middle = math.floor(node_count / 2)
        
        node_count = 0
        node = head
        while node.next is not None and node_count < middle:
            node_count += 1
            node = node.next
        return node
