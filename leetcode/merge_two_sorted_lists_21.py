# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def mergeTwoLists(self, list1, list2):
        cur = dummy = ListNode()
        while list1 and list2:
            if list1.val < list2.val:
                cur.next = list1
                list1, cur = list1.next, list1
            else:
                cur.next = list2
                list2, cur = list2.next, list2

        if list1 or list2:
            cur.next = list1 if list1 else list2

        return dummy.next

def create_linked_list(python_list):
    node = None
    for n in python_list[::-1]:
        node = ListNode(n, node)
    return node

def print_linked_list(node):
    while node is not None:
        print(node.val, end=" ")
        node = node.next
    print()

a = [1,2,4]
b = [1,3,4]

ll_a = create_linked_list(a)
ll_b = create_linked_list(b)

print_linked_list(ll_a)
print_linked_list(ll_b)

jam = Solution()
result = jam.mergeTwoLists(ll_a, ll_b)

print_linked_list(result)



