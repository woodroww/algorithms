# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def print_result(self, result):
        node = result
        while node is not None:
            #print(f"{node.val}", end=" ")
            node = node.next
        #print()
            
    def mergeTwoLists(self, list1, list2):
        node1 = list1
        node2 = list2
        result = None
        first = None
        if node1 is not None:
            if node2 is not None:
                if node1.val <= node2.val:
                    first = result = node1
                    node1 = result.next = node1.next
                else:
                    first = result = node2
                    node2 = result.next = node2.next
            else:
                first = result = node1
                result = result.next
                node1 = node1.next
        else:
            if node2 is not None:
                first = result = node2
                result = result.next
                node2 = node2.next

        while node1 is not None or node2 is not None: 
            if node1 is not None:
                if node2 is not None:
                    #print(f"two node available {node1.val} {node2.val}")
                    if node1.val <= node2.val:
                        result.next = node1
                        result = result.next
                        node1 = node1.next
                    else:
                        result.next = node2
                        result = result.next
                        node2 = node2.next
                else:
                    #print(f"only node1 available {node1.val}")
                    result.next = node1
                    result = result.next
                    node1 = node1.next
            else: # no node1 do we have only node2 left
                if node2 is not None:
                    #print(f"only node2 left")
                    result.next = node2
                    result = result.next
                    node2 = node2.next
            #self.print_result(first)
        return first

# jam

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



