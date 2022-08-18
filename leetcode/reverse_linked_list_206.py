class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def reverseList(self, head):
        node = head
        values = []
        while node is not None:
            values.append(node.val)
            node = node.next
        if len(values) == 0:
            return None
        values = values[::-1]
        first = prev_node = ListNode(values[0])
        for v in values[1:]:
            node = ListNode(v)
            prev_node.next = node
            prev_node = node
        return first

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

solution = Solution()
print_linked_list(solution.reverseList(ll_a))




