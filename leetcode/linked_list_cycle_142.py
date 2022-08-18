class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class NaiveSolution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        visited = set()
        node = head
        while node is not None:
            if node in visited:
                return node
            else:
                visited.add(node)
            node = node.next
        return None

# https://www.youtube.com/watch?v=gBTe7lFR3vc

class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        slow, fast = head, head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
            if slow == fast:
                # if looking for true or false (is there a cycle) return true here
                # we're looking for the first node in the cycle
                # it seems like finding the last node in the cycle might be better
                # but that is not what leetcode wants
                entry = head
                while slow != entry:
                    slow = slow.next
                    entry = entry.next
                return slow
        # else return false (no cycle)
        return None
