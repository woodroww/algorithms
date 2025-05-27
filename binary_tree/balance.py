# Node definition
class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None

# build returns a pointer to the root Node of the sub-tree
# lower is the lower index of the array
# upper is the upper index of the array
def build(arr, lower, upper):
    size = upper - lower + 1
    if size <= 0: return None
    middle = size // 2 + lower

    subtree_root = Node(arr[middle])
    subtree_root.left = build(arr, lower, middle - 1)
    subtree_root.right = build(arr, middle + 1, upper)
    return subtree_root

# For convenience, In-order printing a tree
def print_tree(root):
    if root is None: return
    print_tree(root.left)
    print(root.val, end=' ')
    print_tree(root.right)

# For convenience, we create a right skewed BST.
# This BST will later be balanced.
def array_to_bst(nums):
    if len(nums) == 0: return None

    root = Node(nums[0])

    curr = root
    for i in range(1, len(nums)):
        curr.right = Node(nums[i])
        curr = curr.right
    return root

# Convenience function for creating an array from BST in-order traversal
def create_array_from_tree(root, arr):
    if not root: return arr

    arr = create_array_from_tree(root.left, arr)
    arr.append(root.val)
    arr = create_array_from_tree(root.right, arr)

    return arr

# Build a right skewed tree
# root = array_to_bst([10, 11, 17, 19, 30, 31, 37, 38])
root = array_to_bst([37, 38, 10, 11, 19, 17, 30, 31])
# Get the array from the tree
new_arr = create_array_from_tree(root, [])
# Make the skewed tree balanced
new_root = build(new_arr, 0, 7)
# In-order printing to verify
print_tree(new_root)
