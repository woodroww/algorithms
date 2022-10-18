use std::collections::VecDeque;
use std::fmt::Display;

pub type NodeRef<T> = Box<Node<T>>;

/// A tree implementation

#[derive(Default)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<NodeRef<T>>,
    pub right: Option<NodeRef<T>>,
}

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

    pub fn left(&self) -> Option<&Box<Node<T>>> {
        self.left.as_ref()
    }

    pub fn right(&self) -> Option<&Box<Node<T>>> {
        self.right.as_ref()
    }

    // idk if this works no tests either
    pub fn height_iterative(&self) -> usize {
        let mut queue: VecDeque<&Node<T>> = VecDeque::new();
        queue.push_back(&self);
        let mut height = 0;
        while !queue.is_empty() {
            let mut node_count = queue.len();
            while node_count > 0 {
                let node = queue.pop_front().unwrap();
                if let Some(left) = &node.left {
                    queue.push_back(left);
                }
                if let Some(right) = &node.right {
                    queue.push_back(right);
                }
                node_count -= 1;
            }
            height += 1;
        }
        height
    }
}

impl<T> Node<T>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd,
{
    pub fn insert(&mut self, value: T) {
        if value < self.value {
            match &mut self.left {
                Some(l) => l.insert(value),
                None => {
                    self.left = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }))
                }
            }
        } else {
            match &mut self.right {
                Some(r) => r.insert(value),
                None => {
                    self.right = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }))
                }
            }
        }
    }
}

pub fn height<T>(root: Option<&NodeRef<T>>) -> isize {
    if root.is_none() {
        return -1;
    }
    let root = root.unwrap();
    let l_height = height(root.left.as_ref());
    let r_height = height(root.right.as_ref());
    std::cmp::max(l_height, r_height) + 1
}

pub fn depth<T>(root: Option<&NodeRef<T>>, node: &NodeRef<T>) -> isize {
    if root.is_none() {
        return -1;
    }
    let root = root.unwrap();

    let mut dist = -1;
    if std::ptr::eq(root, node) {
        return dist + 1;
    }
    dist = depth(root.left.as_ref(), node);
    if dist >= 0 {
        return dist + 1;
    }
    dist = depth(root.right.as_ref(), node);
    if dist >= 0 {
        return dist + 1;
    }

    dist
}

impl<T: Display> Node<T> {
    pub fn print_level(&self, level: isize) {
        if level == 1 {
            print!("{} ", self.value);
        } else if level > 1 {
            if let Some(left) = self.left.as_ref() {
                left.print_level(level - 1);
            }
            if let Some(right) = self.right.as_ref() {
                right.print_level(level - 1);
            }
        }
    }

    pub fn print_recursive(&self, level: usize) {
        if let Some(left) = &self.left {
            left.print_recursive(level + 1);
        }
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", self.value);
        if let Some(right) = &self.right {
            right.print_recursive(level + 1);
        }
    }
}

// does tree contain value ?
pub fn tree_contains<T>(root: &NodeRef<T>, item: T) -> bool
where
    T: std::cmp::PartialEq,
{
    let mut i = LevelOrderIterator::new(root);
    i.find(|&x| x == &item).is_some()
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
pub fn inorder_recursive<T: Display>(node: &Box<Node<T>>) {
    if let Some(left) = &node.left {
        inorder_recursive(&left);
    }
    print!("{} ", node.value);
    if let Some(right) = &node.right {
        inorder_recursive(&right);
    }
}

/// iterate in order with function
pub fn inorder_iterative<T, F>(root: Option<&NodeRef<T>>, mut call_me: F)
where
    T: Display,
    F: FnMut(&T, isize),
{
    if root.is_none() {
        return;
    }
    let mut stack: Vec<&Box<Node<T>>> = Vec::new();
    let mut current: Option<&Box<Node<T>>> = root;
    let root_height = height(Some(root.unwrap()));
    loop {
        if current.is_some() {
            let node = current.unwrap();
            stack.push(node);
            current = node.left.as_ref();
        } else {
            match stack.pop() {
                Some(node) => {
                    let h = height(Some(node));
                    call_me(&node.value, root_height - h);
                    current = node.right.as_ref();
                }
                None => {
                    break;
                }
            }
        }
    }
}

/// An inorder iterator
pub struct InOrderIterator<'a, T> {
    stack: Vec<&'a Node<T>>,
    current: Option<&'a Node<T>>,
}

impl<'a, T> InOrderIterator<'a, T> {
    /// return a new iterator
    pub fn new(root: &'a Node<T>) -> Self {
        Self {
            stack: Vec::new(),
            current: Some(root),
        }
    }
}

impl<'a, T> Iterator for InOrderIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current.is_some() {
                let node = self.current.unwrap();
                self.stack.push(node);
                match &node.left {
                    Some(left) => {
                        self.current = Some(left.as_ref());
                    }
                    None => {
                        self.current = None;
                    }
                }
            } else {
                match self.stack.pop() {
                    Some(node) => {
                        match &node.right {
                            Some(right) => {
                                self.current = Some(right.as_ref());
                            }
                            None => {
                                self.current = None;
                            }
                        }
                        return Some(&node.value);
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

pub fn preorder_recursive<T: Display>(node: &Box<Node<T>>) {
    print!("{} ", node.value);
    if let Some(left) = &node.left {
        preorder_recursive(left);
    }
    if let Some(right) = &node.right {
        preorder_recursive(right);
    }
}

pub fn preorder_iterative<T, F>(root: Option<&NodeRef<T>>, mut call_me: F)
where
    T: Display,
    F: FnMut(&T, isize),
{
    if root.is_none() {
        return;
    }
    let mut stack: Vec<&Box<Node<T>>> = Vec::new();
    stack.push(root.as_ref().unwrap());
    let root_height = height(root);

    while let Some(node) = stack.pop() {
        let h = height(Some(node));
        call_me(&node.value, root_height - h);
        if let Some(right) = node.right.as_ref() {
            stack.push(right);
        }
        if let Some(left) = node.left.as_ref() {
            stack.push(left);
        }
    }
}

/// A pre order iterator
pub struct PreOrderIterator<'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> PreOrderIterator<'a, T> {
    /// return a new iterator starting at root
    pub fn new(root: &'a Node<T>) -> Self {
        let mut i = Self { stack: Vec::new() };
        i.stack.push(root);
        i
    }
}

impl<'a, T> Iterator for PreOrderIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if let Some(right) = node.right.as_ref() {
                self.stack.push(right);
            }
            if let Some(left) = node.left.as_ref() {
                self.stack.push(left);
            }
            Some(&node.value)
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

pub fn postorder_recursive<T>(node: &Box<Node<T>>)
where
    T: Display,
{
    if let Some(left) = &node.left {
        postorder_recursive(&left);
    }
    if let Some(right) = &node.right {
        postorder_recursive(&right);
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

pub fn postorder_iterative<T, F>(root: Option<&NodeRef<T>>, mut call_me: F)
where
    T: Display,
    F: FnMut(&T, isize),
{
    if root.is_none() {
        return;
    }
    let mut stack: Vec<&Box<Node<T>>> = Vec::new();
    stack.push(root.as_ref().unwrap());
    let root_height = height(root);
    let mut stack_two: Vec<&Box<Node<T>>> = Vec::new();

    while let Some(node) = stack.pop() {
        stack_two.push(node);

        if let Some(left) = &node.left {
            stack.push(left);
        }
        if let Some(right) = &node.right {
            stack.push(right);
        }
    }

    while let Some(node) = stack_two.pop() {
        let h = height(Some(node));
        call_me(&node.value, root_height - h);
    }
}

pub struct PostOrderIterator<'a, T> {
    stack: Vec<&'a Node<T>>,
    stack_two: Vec<&'a Node<T>>,
}

impl<'a, T> PostOrderIterator<'a, T> {
    pub fn new(root: &'a Node<T>) -> Self {
        let mut i = Self {
            stack: Vec::new(),
            stack_two: Vec::new(),
        };
        i.stack.push(root);
        while let Some(node) = i.stack.pop() {
            i.stack_two.push(node);
            if let Some(left) = &node.left {
                i.stack.push(left);
            }
            if let Some(right) = &node.right {
                i.stack.push(right);
            }
        }
        i
    }
}

impl<'a, T> Iterator for PostOrderIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack_two.pop() {
            Some(n) => Some(&n.value),
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

pub fn levelorder_recursive<T: Display>(node: Option<&NodeRef<T>>) {
    if node.is_none() {
        return;
    }
    let node = node.as_ref().unwrap();
    let h = height(Some(node));
    for i in 1..=h {
        print!("l:{} ", i);
        node.print_level(i);
        println!("");
    }
}

pub fn levelorder_iterative<T, F>(root: Option<&NodeRef<T>>, mut call_me: F)
where
    F: FnMut(&T),
{
    let mut queue: VecDeque<&Node<T>> = VecDeque::new();
    queue.push_back(root.as_ref().unwrap());
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        call_me(&node.value);
        if let Some(left) = &node.left {
            queue.push_back(left);
        }
        if let Some(right) = &node.right {
            queue.push_back(right);
        }
    }
}

// breadth first
pub struct LevelOrderIterator<'a, T> {
    queue: VecDeque<&'a Node<T>>,
}

impl<'a, T> LevelOrderIterator<'a, T> {
    pub fn new(root: &'a Node<T>) -> Self {
        let mut i = Self {
            queue: VecDeque::new(),
        };
        i.queue.push_back(root);
        i
    }
}

impl<'a, T> Iterator for LevelOrderIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.queue.is_empty() {
            let node = self.queue.pop_front().unwrap();
            if let Some(left) = &node.left {
                self.queue.push_back(left);
            }
            if let Some(right) = &node.right {
                self.queue.push_back(right);
            }
            Some(&node.value)
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

    Some(Box::new(node))
}

// ---------------------------------------------------------------------------------------
// invert_tree
// ---------------------------------------------------------------------------------------
// basically mirror reverse

pub fn invert_tree<T: Clone>(root: Option<&NodeRef<T>>) -> Option<NodeRef<T>> {
    match root {
        Some(node) => Some(Box::new(Node {
            value: node.value.clone(),
            left: invert_tree(node.right.as_ref()),
            right: invert_tree(node.left.as_ref()),
        })),
        None => None,
    }
}

// ---------------------------------------------------------------------------------------
// max_depth
// ---------------------------------------------------------------------------------------

pub fn max_depth<T>(root: Option<&NodeRef<T>>) -> isize {
    if root.is_none() {
        return -1;
    }
    let root = root.unwrap();

    let l_depth = max_depth(root.left.as_ref());
    let r_depth = max_depth(root.right.as_ref());
    std::cmp::max(l_depth, r_depth) + 1
}

// ---------------------------------------------------------------------------------------
// max_width
// ---------------------------------------------------------------------------------------

pub fn max_width<T>(root: Option<&NodeRef<T>>) -> usize {
    let mut max = 0;
    let mut queue: VecDeque<&Node<T>> = VecDeque::new();

    if root.is_none() {
        return 0;
    }
    let root = root.unwrap();

    queue.push_back(root);

    while !queue.is_empty() {
        let mut nodes_in_level = queue.len();
        max = std::cmp::max(max, nodes_in_level);

        while nodes_in_level > 0 {
            let current = queue.pop_front().unwrap();
            if current.left.is_some() {
                queue.push_back(current.left.as_ref().unwrap());
            }
            if current.right.is_some() {
                queue.push_back(current.right.as_ref().unwrap());
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

fn traverse<T>(root: Option<&NodeRef<T>>, max: &mut isize) -> isize {
    if root.is_none() {
        return 0;
    }
    let root = root.unwrap();
    let left = traverse(root.left.as_ref(), max);
    let right = traverse(root.right.as_ref(), max);
    if left + right > *max {
        *max = left + right;
    }
    std::cmp::max(left, right) + 1
}

pub fn diameter<T>(root: Option<&NodeRef<T>>) -> isize {
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
pub fn tree_sum<T>(root: Option<&NodeRef<T>>) -> Option<T>
where
    T: std::ops::Add<Output = T> + Clone,
{
    if root.is_none() {
        return None;
    }
    let root = root.as_ref().unwrap();
    if root.left().is_none() && root.right.is_none() {
        return Some(root.value().clone());
    }
    if root.left().is_some() && root.right.is_some() {
        let left = tree_sum(root.left.as_ref()).unwrap();
        let right = tree_sum(root.right.as_ref()).unwrap();
        return Some(root.value().clone() + left + right);
    }
    if root.left().is_some() {
        let left = tree_sum(root.left.as_ref()).unwrap();
        return Some(root.value().clone() + left);
    } else {
        let right = tree_sum(root.right.as_ref()).unwrap();
        return Some(root.value().clone() + right);
    }
}

/// Get sum of all tree nodes iteratively
pub fn tree_sum_iterative<T>(root: Option<&NodeRef<T>>) -> Option<T>
where
    T: std::ops::AddAssign + Default + Copy,
{
    if root.is_none() {
        return None;
    }
    let iter = LevelOrderIterator::new(&root.as_ref().unwrap());

    let mut result: T = T::default();
    for i in iter.into_iter() {
        result += *i;
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
pub fn minimum<T>(root: Option<&NodeRef<T>>) -> Option<T>
where
    T: Default + Clone + std::cmp::Ord,
{
    if root.is_none() {
        return None;
    }
    let root = root.as_ref().unwrap();
    if root.left().is_none() && root.right.is_none() {
        return Some(root.value().clone());
    }
    if root.left().is_some() && root.right.is_some() {
        let left = minimum(root.left.as_ref()).unwrap();
        let right = minimum(root.right.as_ref()).unwrap();
        let value = root.value.clone();
        return Some(min2(min2(left, right), value));
    }
    if root.left().is_some() {
        let left = minimum(root.left.as_ref()).unwrap();
        return Some(min2(root.value.clone(), left));
    } else {
        let right = minimum(root.right.as_ref()).unwrap();
        return Some(min2(root.value.clone(), right));
    }
}

// ---------------------------------------------------------------------------------------
// max_path_sum
// ---------------------------------------------------------------------------------------

/// Find the largest possible sum along a path from the root to a leaf
pub fn max_path_sum<T>(root: Option<&NodeRef<T>>) -> Option<T>
where
    T: Ord + std::ops::Add<Output = T> + Clone,
{
    if root.is_none() {
        return None;
    }
    let root = root.as_ref().unwrap();
    if root.left().is_none() && root.right.is_none() {
        return Some(root.value().clone());
    }
    if root.left().is_some() && root.right.is_some() {
        let left = max_path_sum(root.left.as_ref());
        let right = max_path_sum(root.right.as_ref());
        let max_child: T = std::cmp::max(left.unwrap(), right.unwrap());
        let max_child_path_sum = root.value().clone();
        return Some(max_child + max_child_path_sum);
    }
    if root.left().is_some() {
        return Some(root.value().clone() + root.left().unwrap().value().clone());
    } else {
        return Some(root.value().clone() + root.right().unwrap().value().clone());
    }
}

