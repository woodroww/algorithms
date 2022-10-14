# Binary Tree in Python


import os
from IPython.core.display import display, HTML
from pyvis.network import Network
import webbrowser

chrome_path = 'open -a /Applications/Google\ Chrome.app %s'
safari_path = 'open -a /Applications/Safari.app %s'
base_url = "/Users/matt/Documents/Programming/trees/python/"

def saveShow(net, file_name, browser_path=safari_path):
    net.show(os.path.join(base_url, file_name))
    webbrowser.get(browser_path).open(os.path.join(base_url, file_name))


class Node:
    def __init__(self, key):
        self.left = None
        self.right = None
        self.val = key

    # Traverse preorder
    def traversePreOrder(self):
        print(self.val, end=' ')
        if self.left:
            self.left.traversePreOrder()
        if self.right:
            self.right.traversePreOrder()

    # Traverse inorder
    def traverseInOrder(self):
        if self.left:
            self.left.traverseInOrder()
        print(self.val, end=' ')
        if self.right:
            self.right.traverseInOrder()

    # Traverse postorder
    def traversePostOrder(self):
        if self.left:
            self.left.traversePostOrder()
        if self.right:
            self.right.traversePostOrder()
        print(self.val, end=' ')

    def height(self):
        lheight = 0
        rheight = 0
        if self.left is not None:
            lheight = self.left.height()
        if self.right is not None:
            rheight = self.right.height()
        if lheight > rheight:
            return lheight + 1
        else:
            return rheight + 1

    def printLevelOrder(self):
        h = self.height()
        for i in range(1, h + 1):
            self.printCurrentLevel(i)

    def printCurrentLevel(self, level):
        if level == 1:
            print(self.val, end=" ")
        elif level > 1:
            if self.left is not None:
                self.left.printCurrentLevel(level - 1)
            if self.right is not None:
                self.right.printCurrentLevel(level - 1)

def isFullTree(root):
    # Tree empty case
    if root is None:
        return True
    # Checking whether child is present
    if root.left is None and root.right is None:
        return True
    if root.left is not None and root.right is not None:
        return (isFullTree(root.left) and isFullTree(root.right))
    return False


root = Node("a")
root.left = Node("b")
root.right = Node("c")
root.left.left = Node("d")
root.left.right = Node("e")


root = Node(1)
root.left = Node(2)
root.right = Node(3)
root.left.left = Node(4)
root.left.right = Node(5)


print("Pre order Traversal: ", end="")
root.traversePreOrder()
print("\nIn order Traversal: ", end="")
root.traverseInOrder()
print("\nPost order Traversal: ", end="")
root.traversePostOrder()
print("\nLevel order Traversal: ", end="")
root.printLevelOrder()
print()
if isFullTree(root):
    print("Full")
else:
    print("Not full")


net = Network()
net.add_node(1, label="1")
net.add_node(2, label="2")
net.add_node(3, label="3")
net.add_node(4, label="4")
net.add_edges([(1, 2), (1, 3), (2, 4)])
saveShow(net, "nodes.html")




