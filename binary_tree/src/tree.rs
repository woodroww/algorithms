use std::collections::VecDeque;
use std::fmt::Display;
use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::RefCell;


/// A tree implementation
// Leetcode Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
pub struct Node<T> {
    pub value: T,
    pub left: Option<Rc<RefCell<Node<T>>>>,
    pub right: Option<Rc<RefCell<Node<T>>>>,
}

pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

impl<T> std::fmt::Debug for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Node<T> {

    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn left(&self) -> Option<NodeRef<T>> {
        match &self.left {
            Some(l) => { Some(Rc::clone(l)) },
            None => { None },
        }
    }

    pub fn right(&self) -> Option<NodeRef<T>> {
        match &self.right {
            Some(r) => { Some(Rc::clone(r)) },
            None => { None },
        }
    }

    pub fn has_children(&self) -> bool {
        self.left.is_some() || self.right.is_some()
    }


    pub fn left_mut(&mut self) -> &mut Option<Rc<RefCell<Node<T>>>> {
        &mut self.left
    }
}

// idk if this works no tests either
pub fn height_iterative<T>(root: NodeRef<T>) -> usize {
    let mut queue: VecDeque<NodeRef<T>> = VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut height = 0;
    while !queue.is_empty() {
        let mut node_count = queue.len();
        while node_count > 0 {
            let node = queue.pop_front().unwrap();
            let cell: &RefCell<Node<T>> = node.borrow();
            let node = cell.borrow();
            if let Some(left) = &node.left {
                queue.push_back(Rc::clone(left));
            }
            if let Some(right) = &node.right {
                queue.push_back(Rc::clone(right));
            }
            node_count -= 1;
        }
        height += 1;
    }
    height
}

impl<T> Node<T>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd,
{
    pub fn insert(&mut self, value: T) {
        if value < self.value {
            match &self.left {
                Some(l) => {
                    let cell: &RefCell<Node<T>> = l.borrow();
                    let mut node = cell.borrow_mut();
                    node.insert(value);
                }
                None => {
                    self.left = Some(Rc::new(RefCell::new(Node {
                        value,
                        left: None,
                        right: None,
                    })))
                }
            }
        } else {
            match &self.right {
                Some(r) => {
                    let cell: &RefCell<Node<T>> = r.borrow();
                    let mut node = cell.borrow_mut();
                    node.insert(value);
                }
                None => {
                    self.right = Some(Rc::new(RefCell::new(Node {
                        value,
                        left: None,
                        right: None,
                    })))
                }
            }
        }
    }
}

pub fn height<T>(root: Option<NodeRef<T>>) -> isize {
    if root.is_none() {
        return -1;
    }
    let root = root.unwrap();
    let cell: &RefCell<Node<T>> = root.borrow();
    let node = cell.borrow();

    let l_height = height(node.left());
    let r_height = height(node.right());
    std::cmp::max(l_height, r_height) + 1
}

/*
pub fn depth<T>(root: Option<NodeRef<T>>, node: &NodeRef<T>) -> isize {
    if root.is_none() {
        return -1;
    }
    let root = root.unwrap();

    let cell: &RefCell<Node<T>> = root.borrow();
    let root = cell.borrow();

    let mut dist = -1;
    if std::ptr::eq(Rc::as_ptr(cell), node) {
        return dist + 1;
    }
    dist = depth(root.left, node);
    if dist >= 0 {
        return dist + 1;
    }
    dist = depth(root.right, node);
    if dist >= 0 {
        return dist + 1;
    }

    dist
}
*/

impl<T: Display> Node<T> {
    pub fn print_level(&self, level: isize) {
        if level == 1 {
            print!("{} ", self.value);
        } else if level > 1 {
            if let Some(left) = &self.left {
                let cell: &RefCell<Node<T>> = left.borrow();
                let node = cell.borrow();
                node.print_level(level - 1);
            }
            if let Some(right) = &self.right {
                let cell: &RefCell<Node<T>> = right.borrow();
                let node = cell.borrow();
                node.print_level(level - 1);
            }
        }
    }

    pub fn print_recursive(&self, level: usize) {
        if let Some(left) = &self.left {
            let cell: &RefCell<Node<T>> = left.borrow();
            let node = cell.borrow();
            node.print_recursive(level + 1);
        }
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", self.value);
        if let Some(right) = &self.right {
            let cell: &RefCell<Node<T>> = right.borrow();
            let node = cell.borrow();
            node.print_recursive(level + 1);
        }
    }
}

// does tree contain value ?
pub fn tree_contains<T>(root: NodeRef<T>, item: T) -> bool
where
    T: std::cmp::PartialEq + Copy,
{
    let mut i = LevelOrderIterator::new(root);
    i.find(|x| x == &item).is_some()
}

// ---------------------------------------------------------------------------------------
// In order
// ---------------------------------------------------------------------------------------
/*
Algorithm Inorder(tree)
   1. Traverse the left subtree, i.e., call Inorder(left-subtree)
   2. Visit the root.
   3. Traverse the right subtree, i.e., call Inorder(right-subtree)
*/

/// print value using in order recursive function
pub fn inorder_recursive<T: Display>(node: NodeRef<T>) {
    let cell: &RefCell<Node<T>> = node.borrow();
    let node = cell.borrow();
    if let Some(left) = &node.left {
        inorder_recursive(Rc::clone(left));
    }
    print!("{} ", node.value);
    if let Some(right) = &node.right {
        inorder_recursive(Rc::clone(right));
    }
}

/// iterate in order with function
pub fn inorder_iterative<T, F>(root: Option<NodeRef<T>>, mut call_me: F)
where
    T: Display,
    F: FnMut(&T, isize),
{
    if root.is_none() {
        return;
    }
    let root = root.unwrap();
    let root_height = height(Some(Rc::clone(&root)));
    let mut stack: Vec<NodeRef<T>> = Vec::new();
    let mut current: Option<NodeRef<T>> = Some(root);
    loop {
        if current.is_some() {
            let node = current.unwrap();
            stack.push(Rc::clone(&node));
            let cell: &RefCell<Node<T>> = node.borrow();
            let node = cell.borrow();
            current = node.left;
        } else {
            match stack.pop() {
                Some(node) => {
                    let h = height(Some(node));
                    let cell: &RefCell<Node<T>> = node.borrow();
                    let node = cell.borrow();
                    call_me(&node.value, root_height - h);
                    current = node.right;
                }
                None => {
                    break;
                }
            }
        }
    }
}

/// An inorder iterator
pub struct InOrderIterator<T> {
    stack: Vec<NodeRef<T>>,
    current: Option<NodeRef<T>>,
}

impl<T> InOrderIterator<T> {
    /// return a new iterator
    pub fn new(root: NodeRef<T>) -> Self {
        Self {
            stack: Vec::new(),
            current: Some(root),
        }
    }
}

impl<T> Iterator for InOrderIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current.is_some() {
                let node = self.current.unwrap();
                self.stack.push(node);
                let cell: &RefCell<Node<T>> = node.borrow();
                let node = cell.borrow();
                match node.left {
                    Some(left) => {
                        self.current = Some(left);
                    }
                    None => {
                        self.current = None;
                    }
                }
            } else {
                match self.stack.pop() {
                    Some(node) => {
                        let cell: &RefCell<Node<T>> = node.borrow();
                        let node = cell.borrow();
                        match node.right {
                            Some(right) => {
                                self.current = Some(right);
                            }
                            None => {
                                self.current = None;
                            }
                        }
                        return Some(node.value);
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        None
    }
}

// ---------------------------------------------------------------------------------------
// Pre order
// ---------------------------------------------------------------------------------------
/*
Algorithm Preorder(tree)
   1. Visit the root.
   2. Traverse the left subtree, i.e., call Preorder(left-subtree)
   3. Traverse the right subtree, i.e., call Preorder(right-subtree)
*/

pub fn preorder_recursive<T: Display>(node: NodeRef<T>) {
    let cell: &RefCell<Node<T>> = node.borrow();
    let node = cell.borrow();
    print!("{} ", node.value);

    if let Some(left) = node.left {
        preorder_recursive(left);
    }
    if let Some(right) = node.right {
        preorder_recursive(right);
    }
}

pub fn preorder_iterative<T, F>(root: Option<NodeRef<T>>, mut call_me: F)
where
    T: Display,
    F: FnMut(&T, isize),
{
    if root.is_none() {
        return;
    }
    let mut stack: Vec<NodeRef<T>> = Vec::new();
    stack.push(root.unwrap());
    let root_height = height(root);

    while let Some(node) = stack.pop() {
        let h = height(Some(node));
        let cell: &RefCell<Node<T>> = node.borrow();
        let node = cell.borrow();
        call_me(&node.value, root_height - h);
        if let Some(right) = node.right {
            stack.push(right);
        }
        if let Some(left) = node.left {
            stack.push(left);
        }
    }
}

/// A pre order iterator
pub struct PreOrderIterator<T> {
    stack: Vec<NodeRef<T>>,
}

impl<T> PreOrderIterator<T> {
    /// return a new iterator starting at root
    pub fn new(root: NodeRef<T>) -> Self {
        let mut i = Self { stack: Vec::new() };
        i.stack.push(root);
        i
    }
}

impl<T: Copy> Iterator for PreOrderIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let cell: &RefCell<Node<T>> = node.borrow();
            let node = cell.borrow();
            if let Some(right) = node.right {
                self.stack.push(right);
            }
            if let Some(left) = node.left() {
                self.stack.push(left);
            }
            Some(node.value)
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------------------
// Post order
// ---------------------------------------------------------------------------------------
/*
Algorithm Postorder(tree)
   1. Traverse the left subtree, i.e., call Postorder(left-subtree)
   2. Traverse the right subtree, i.e., call Postorder(right-subtree)
   3. Visit the root.
*/

pub fn postorder_recursive<T>(node: NodeRef<T>)
where
    T: Display,
{
    let cell: &RefCell<Node<T>> = node.borrow();
    let node = cell.borrow();
    if let Some(left) = node.left {
        postorder_recursive(left);
    }
    if let Some(right) = node.right {
        postorder_recursive(right);
    }
    print!("{} ", &node.value);
}

/*
1. Push root to first stack.
2. Loop while first stack is not empty
   2.1 Pop a node from first stack and push it to second stack
   2.2 Push left and right children of the popped node to first stack
3. Print contents of second stack
*/

pub fn postorder_iterative<T, F>(root: Option<NodeRef<T>>, mut call_me: F)
where
    T: Display,
    F: FnMut(T, isize),
{
    if root.is_none() {
        return;
    }
    let mut stack: Vec<NodeRef<T>> = Vec::new();
    stack.push(root.unwrap());
    let root_height = height(root);
    let mut stack_two: Vec<NodeRef<T>> = Vec::new();

    while let Some(node) = stack.pop() {
        stack_two.push(node);
        let cell: &RefCell<Node<T>> = node.borrow();
        let node = cell.borrow();

        if let Some(left) = node.left {
            stack.push(left);
        }
        if let Some(right) = node.right {
            stack.push(right);
        }
    }

    while let Some(node) = stack_two.pop() {
        let h = height(Some(node));
        let cell: &RefCell<Node<T>> = node.borrow();
        let node = cell.borrow();
        call_me(node.value, root_height - h);
    }
}

pub struct PostOrderIterator<T> {
    stack: Vec<NodeRef<T>>,
    stack_two: Vec<NodeRef<T>>,
}

impl<T> PostOrderIterator<T> {
    pub fn new(root: NodeRef<T>) -> Self {
        let mut i = Self {
            stack: Vec::new(),
            stack_two: Vec::new(),
        };
        i.stack.push(root);
        while let Some(node) = i.stack.pop() {
            i.stack_two.push(node);
            let cell: &RefCell<Node<T>> = node.borrow();
            let node = cell.borrow();
            if let Some(left) = node.left {
                i.stack.push(left);
            }
            if let Some(right) = node.right {
                i.stack.push(right);
            }
        }
        i
    }
}

impl<T: Copy> Iterator for PostOrderIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack_two.pop() {
            Some(node) => {
                let cell: &RefCell<Node<T>> = node.borrow();
                let node = cell.borrow();
                Some(node.value)
            }
            None => None,
        }
    }
}

// ---------------------------------------------------------------------------------------
// Level order
// ---------------------------------------------------------------------------------------
/*
Commentary
*/

pub fn levelorder_recursive<T: Display>(node: Option<NodeRef<T>>) {
    if node.is_none() {
        return;
    }
    let node = node.unwrap();
    let h = height(Some(node));
    for i in 1..=h {
        print!("l:{} ", i);
        let cell: &RefCell<Node<T>> = node.borrow();
        let node = cell.borrow();
        node.print_level(i);
        println!("");
    }
}

pub fn levelorder_iterative<T, F>(root: Option<NodeRef<T>>, mut call_me: F)
where
    F: FnMut(T),
{
    if root.is_none() {
        return;
    }
    let mut queue: VecDeque<NodeRef<T>> = VecDeque::new();
    queue.push_back(root.unwrap());
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let cell: &RefCell<Node<T>> = node.borrow();
        let node = cell.borrow();
        call_me(node.value);
        if let Some(left) = node.left {
            queue.push_back(left);
        }
        if let Some(right) = node.right {
            queue.push_back(right);
        }
    }
}

// breadth first
pub struct LevelOrderIterator<T> {
    queue: VecDeque<NodeRef<T>>,
}

impl<T> LevelOrderIterator<T> {
    pub fn new(root: NodeRef<T>) -> Self {
        let mut i = Self {
            queue: VecDeque::new(),
        };
        i.queue.push_back(root);
        i
    }
}

impl<T: Copy> Iterator for LevelOrderIterator<T> {
    type Item = T;
    //type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.queue.is_empty() {
            let node = self.queue.pop_front().unwrap();
            let cell: &RefCell<Node<T>> = node.borrow();
            let node = cell.borrow();

            if let Some(left) = node.left {
                self.queue.push_back(left);
            }
            if let Some(right) = node.right {
                self.queue.push_back(right);
            }
            Some(node.value)
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------------------
// generate_tree
// ---------------------------------------------------------------------------------------
// generate_tree with (3, 1) will generate this:
//     1
//  2     5
// 3 4   6 7

// generate_tree with (4, 1) will generate this:
//               1
//         2           9
//      3     6    10     13
//     4 5   7 8  11 12  14 15

pub fn generate_tree<T>(level: usize, counter: &mut T) -> Option<NodeRef<T>>
where
    T: std::ops::AddAssign<i32> + Copy,
{
    if level == 0 {
        return None;
    }

    let mut node = Node {
        value: *counter,
        left: None,
        right: None,
    };
    *counter += 1;
    node.left = generate_tree(level - 1, counter);
    node.right = generate_tree(level - 1, counter);

    Some(Rc::new(RefCell::new(node)))
}

// ---------------------------------------------------------------------------------------
// invert_tree
// ---------------------------------------------------------------------------------------
// basically mirror reverse

pub fn invert_tree<T: Clone>(root: Option<NodeRef<T>>) -> Option<NodeRef<T>> {
    match root {
        Some(node) => {
            let cell: &RefCell<Node<T>> = node.borrow();
            let node = cell.borrow();
            Some(Rc::new(RefCell::new(Node {
                value: node.value.clone(),
                left: invert_tree(node.right),
                right: invert_tree(node.left),
            })))
        },
        None => None,
    }
}

// ---------------------------------------------------------------------------------------
// max_depth
// ---------------------------------------------------------------------------------------

pub fn max_depth<T>(root: Option<NodeRef<T>>) -> isize {
    if root.is_none() {
        return -1;
    }
    let root = root.unwrap();
    let cell: &RefCell<Node<T>> = root.borrow();
    let node = cell.borrow();

    let l_depth = max_depth(node.left);
    let r_depth = max_depth(node.right);
    std::cmp::max(l_depth, r_depth) + 1
}

// ---------------------------------------------------------------------------------------
// max_width
// ---------------------------------------------------------------------------------------

pub fn max_width<T>(root: Option<NodeRef<T>>) -> usize {
    if root.is_none() {
        return 0;
    }
    let mut max = 0;
    let mut queue: VecDeque<NodeRef<T>> = VecDeque::new();
    let root = root.unwrap();

    queue.push_back(root);

    while !queue.is_empty() {
        let mut nodes_in_level = queue.len();
        max = std::cmp::max(max, nodes_in_level);

        while nodes_in_level > 0 {
            let current = queue.pop_front().unwrap();

            let cell: &RefCell<Node<T>> = current.borrow();
            let current = cell.borrow();

            if current.left.is_some() {
                queue.push_back(current.left.unwrap());
            }
            if current.right.is_some() {
                queue.push_back(current.right.unwrap());
            }
            nodes_in_level -= 1;
        }
    }
    max
}

// ---------------------------------------------------------------------------------------
// diameter
// ---------------------------------------------------------------------------------------
// The diameter/width of a tree is defined as the number of nodes on the longest path between two
// end nodes.
// the diameter of T’s left subtree.
// the diameter of T’s right subtree.
// the longest path between leaves that goes through the root of T (this can be computed from the
// heights of the subtrees of T)

fn traverse<T>(root: Option<NodeRef<T>>, max: &mut isize) -> isize {
    if root.is_none() {
        return 0;
    }
    let root = root.unwrap();
    let cell: &RefCell<Node<T>> = root.borrow();
    let root = cell.borrow();

    let left = traverse(root.left, max);
    let right = traverse(root.right, max);
    if left + right > *max {
        *max = left + right;
    }
    std::cmp::max(left, right) + 1
}

pub fn diameter<T>(root: Option<NodeRef<T>>) -> isize {
    if root.is_none() {
        return 0;
    }
    let mut max = 0;
    traverse(root, &mut max);
    max
}


//     10
//   11    12
//  2  31
// ---------------------------------------------------------------------------------------
// tree_sum
// ---------------------------------------------------------------------------------------

/// Get sum of all tree nodes recursively
pub fn tree_sum<T>(root: Option<NodeRef<T>>) -> Option<T>
where
    T: std::ops::Add<Output = T> + Clone,
{
    if root.is_none() {
        return None;
    }
    let root = root.unwrap();
    let cell: &RefCell<Node<T>> = root.borrow();
    let root = cell.borrow();

    if root.left().is_none() && root.right.is_none() {
        return Some(root.value().clone());
    }
    if root.left().is_some() && root.right.is_some() {
        let left = tree_sum(root.left).unwrap();
        let right = tree_sum(root.right).unwrap();
        return Some(root.value().clone() + left + right);
    }
    if root.left().is_some() {
        let left = tree_sum(root.left).unwrap();
        return Some(root.value().clone() + left);
    } else {
        let right = tree_sum(root.right).unwrap();
        return Some(root.value().clone() + right);
    }
}

/// Get sum of all tree nodes iteratively
pub fn tree_sum_iterative<T>(root: Option<NodeRef<T>>) -> Option<T>
where
    T: std::ops::AddAssign + Default + Copy,
{
    if root.is_none() {
        return None;
    }

    let root = root.unwrap();
    let iter = LevelOrderIterator::new(root);

    let mut result: T = T::default();
    for i in iter.into_iter() {
        result += i;
    }
    Some(result)
}

// ---------------------------------------------------------------------------------------
// minimum
// ---------------------------------------------------------------------------------------

fn min2<T>(x: T, y: T) -> T
where
    T: std::cmp::PartialOrd,
{
    if x < y {
        x
    } else {
        y
    }
}

// n = number of nodes
// time O(n)
// space O(n)
/// Find the minimum value in tree
pub fn minimum<T>(root: Option<NodeRef<T>>) -> Option<T>
where
    T: Default + Clone + std::cmp::Ord,
{
    if root.is_none() {
        return None;
    }

    let root = root.unwrap();
    let cell: &RefCell<Node<T>> = root.borrow();
    let root = cell.borrow();

    if root.left().is_none() && root.right.is_none() {
        return Some(root.value().clone());
    }
    if root.left().is_some() && root.right.is_some() {
        let left = minimum(root.left).unwrap();
        let right = minimum(root.right).unwrap();
        let value = root.value.clone();
        return Some(min2(min2(left, right), value));
    }
    if root.left().is_some() {
        let left = minimum(root.left).unwrap();
        return Some(min2(root.value.clone(), left));
    } else {
        let right = minimum(root.right).unwrap();
        return Some(min2(root.value.clone(), right));
    }
}

// ---------------------------------------------------------------------------------------
// max_path_sum
// ---------------------------------------------------------------------------------------

/// Find the largest possible sum along a path from the root to a leaf
pub fn max_path_sum<T>(root: Option<NodeRef<T>>) -> Option<T>
where
    T: Ord + std::ops::Add<Output = T> + Clone,
{
    if root.is_none() {
        return None;
    }
    let root = root.unwrap();
    let cell: &RefCell<Node<T>> = root.borrow();
    let root = cell.borrow();

    if root.left().is_none() && root.right.is_none() {
        return Some(root.value().clone());
    }
    if root.left().is_some() && root.right.is_some() {
        let left = max_path_sum(root.left);
        let right = max_path_sum(root.right);
        let max_child: T = std::cmp::max(left.unwrap(), right.unwrap());
        let max_child_path_sum = root.value().clone();
        return Some(max_child + max_child_path_sum);
    }
    if root.left().is_some() {
        let cell: &RefCell<Node<T>> = root.left().unwrap().borrow();
        let left = cell.borrow();
        return Some(root.value().clone() + left.value().clone());
    } else {
        let cell: &RefCell<Node<T>> = root.right().unwrap().borrow();
        let right = cell.borrow();
        return Some(root.value().clone() + right.value().clone());
    }
}

