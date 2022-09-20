use std::fmt::Display;
use std::collections::VecDeque;

type NodeRef<T> = Box<Node<T>>;

/// A tree implementation

#[derive(Default)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<NodeRef<T>>,
    pub right: Option<NodeRef<T>>,
}

impl<T> std::fmt::Debug for Node<T>
where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Node<T> {
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn left(&self) -> Option<&Box<Node<T>>> {
        self.left.as_ref()
    }

    pub fn right(&self) -> Option<&Box<Node<T>>> {
        self.right.as_ref()
    }

    pub fn height(&self) -> Option<usize> {
        self.height_recursive()
    }

    pub fn height_recursive(&self) -> Option<usize> {
        let l_depth = match &self.left {
            Some(left) => match left.height_recursive() {
                Some(h) => h,
                None => 0,
            }
            None => {
                0
            }
        };
        let r_depth = match &self.right {
            Some(right) => match right.height_recursive() {
                Some(h) => h,
                None => 0,
            }
            None => {
                0
            }
        };
        if l_depth > r_depth {
            Some(l_depth + 1)
        } else {
            Some(r_depth + 1)
        }
    }

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

impl<T: Display> Node<T> {

    pub fn print_level(&self, level: usize) {
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

pub fn tree_contains<T>(root: &NodeRef<T>, item: T) -> bool
where T: std::cmp::PartialEq
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
pub fn inorder_iterative<T, F>(root: &Option<NodeRef<T>>, mut call_me: F)
where T: Display, F: FnMut(&T, usize)
{
    if root.is_none() {
        return;
    }
    let mut stack: Vec<&Box<Node<T>>> = Vec::new();
    let mut current: Option<&Box<Node<T>>> = root.as_ref();
    let root_height = root.as_ref().unwrap().height().unwrap_or(0);
    loop {
        if current.is_some() {
            let node = current.unwrap();
            stack.push(node);
            current = node.left.as_ref();
        } else {
            match stack.pop() {
                Some(node) => {
                    let h = node.height().unwrap_or(0);
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

pub fn preorder_iterative<T, F>(root: &Option<NodeRef<T>>, mut call_me: F)
where T: Display, F: FnMut(&T, usize)
{
    if root.is_none() {
        return;
    }
    let mut stack: Vec<&Box<Node<T>>> = Vec::new();
    stack.push(root.as_ref().unwrap());
    let root_height = root.as_ref().unwrap().height().unwrap_or(0);

    while let Some(node) = stack.pop() {
        let h = node.height().unwrap_or(0);
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
        let mut i = Self {
            stack: Vec::new(),
        };
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
where T: Display
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

pub fn postorder_iterative<T, F>(root: &Option<NodeRef<T>>, mut call_me: F)
where T: Display, F: FnMut(&T, usize)
{
    if root.is_none() {
        return;
    }
    let mut stack: Vec<&Box<Node<T>>> = Vec::new();
    stack.push(root.as_ref().unwrap());
    let root_height = root.as_ref().unwrap().height().unwrap_or(0);
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
        let h = node.height().unwrap_or(0);
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

pub fn levelorder_recursive<T: Display>(node: &Option<NodeRef<T>>) {
    if node.is_none() {
        return;
    }
    let node = node.as_ref().unwrap();
    let h = node.height().unwrap_or(0);
    for i in 1..=h {
        node.print_level(i);
    }
}

pub fn levelorder_iterative<T, F>(root: &Option<NodeRef<T>>, mut call_me: F)
where F: FnMut(&T)
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

pub fn invert_tree<T: Clone>(root: &Option<NodeRef<T>>) -> Option<NodeRef<T>> {
    match root {
        Some(node) => Some(Box::new(Node {
            value: node.value.clone(),
            left: invert_tree(&node.right),
            right: invert_tree(&node.left),
        })),
        None => None,
    }
}

// sum the tree
pub fn tree_sum<T>(root: &Option<NodeRef<T>>) -> Option<T>
where T: std::ops::Add<Output = T> + Clone
{
    if root.is_none() {
        return None;
    }
    let root = root.as_ref().unwrap();
    if root.left().is_none() && root.right.is_none() {
        return Some(root.value().clone());
    }
    if root.left().is_some() && root.right.is_some() {
        let left = tree_sum(&root.left).unwrap();
        let right = tree_sum(&root.right).unwrap();
        return Some(root.value().clone() + left + right);
    }
    if root.left().is_some() {
        let left = tree_sum(&root.left).unwrap();
        return Some(root.value().clone() + left);
    } else {
        let right = tree_sum(&root.right).unwrap();
        return Some(root.value().clone() + right);
    }
}

pub fn tree_sum_iterative<T>(root: &Option<NodeRef<T>>) -> Option<T>
where T: std::ops::AddAssign + Default + Copy
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

// generate_tree with (3, 1) will generate this:
//     1
//  2     5
// 3 4   6 7

#[test]
fn tree_sum_1() {
    let root = generate_tree(3, &mut 1);
    let sum = tree_sum(&root);
    assert_eq!(sum.unwrap(), 28);
    let sum = tree_sum_iterative(&root);
    assert_eq!(sum.unwrap(), 28);
}

// generate_tree with (4, 1) will generate this:
//               1
//         2           9
//      3     6    10     13
//     4 5   7 8  11 12  14 15 

#[test]
fn tree_sum_2() {
    let root = generate_tree(4, &mut 1);
    let sum = tree_sum(&root);
    assert_eq!(sum.unwrap(), 120);
    let sum = tree_sum_iterative(&root);
    assert_eq!(sum.unwrap(), 120);
}

fn min2<T>(x: T, y: T) -> T
where T: std::cmp::PartialOrd
{
    if x < y { x } else { y }
}

// n = number of nodes
// time O(n)
// space O(n)
pub fn minimum<T>(root: &Option<NodeRef<T>>) -> Option<T>
where T: Default + Clone + std::cmp::Ord 
{
    if root.is_none() {
        return None;
    }
    let root = root.as_ref().unwrap();
    if root.left().is_none() && root.right.is_none() {
        return Some(root.value().clone());
    }
    if root.left().is_some() && root.right.is_some() {
        //let left = root.left.as_ref().unwrap().value.clone();
        let left = minimum(&root.left).unwrap();
        //let right = root.right.as_ref().unwrap().value.clone();
        let right = minimum(&root.right).unwrap();
        let value = root.value.clone(); 
        return Some(min2(min2(left, right), value));
    }
    if root.left().is_some() {
        //let left = root.left.as_ref().unwrap().value.clone();
        let left = minimum(&root.left).unwrap();
        return Some(min2(root.value.clone(), left));
    } else {
        //let right = root.right.as_ref().unwrap().value.clone();
        let right = minimum(&root.right).unwrap();
        return Some(min2(root.value.clone(), right));
    }
}

#[test]
fn minimum_1() {
    let root = Box::new(make_num_tree_1());
    let result = minimum(&Some(root));
    assert_eq!(result.unwrap(), -2);
}

#[test]
fn minimum_2() {
    let root = Box::new(make_num_tree_2());
    let result = minimum(&Some(root));
    assert_eq!(result.unwrap(), 3);
}

#[test]
fn minimum_3() {
    let root = Box::new(make_num_tree_3());
    let result = minimum(&Some(root));
    assert_eq!(result.unwrap(), -13);
}


// type NodeRef<T> = Option<Box<Node<T>>>;
pub fn max_path_sum<T>(root: &Option<NodeRef<T>>) -> Option<T>
where T: Ord + std::ops::Add<Output = T> + Clone
{
    if root.is_none() {
        return None;
    }
    let root = root.as_ref().unwrap();
    if root.left().is_none() && root.right.is_none() {
        return Some(root.value().clone());
    }
    if root.left().is_some() && root.right.is_some() {
        let left = max_path_sum(&root.left);
        let right = max_path_sum(&root.right);
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

//       3
//    /    \
//   11     4
//  / \      \
// 4   -2     1

pub fn make_num_tree_1() -> Node<i32> {

    let n_4 = Node {
        value: 4,
        left: None,
        right: None,
    };
    let n_neg2 = Node {
        value: -2,
        left: None,
        right: None,
    };
    let n_1 = Node {
        value: 1,
        left: None,
        right: None,
    };
    let n_4_2 = Node {
        value: 4,
        left: None,
        right: Some(Box::new(n_1)),
    };
    let n_11 = Node {
        value: 11,
        left: Some(Box::new(n_4)),
        right: Some(Box::new(n_neg2)),
    };
    Node {
        value: 3,
        left: Some(Box::new(n_11)),
        right: Some(Box::new(n_4_2)),
    }
}

//       5
//    /    \
//   11     3
//  / \      \
// 4   14     12

pub fn make_num_tree_2() -> Node<i32> {

    let n_4 = Node {
        value: 4,
        left: None,
        right: None,
    };
    let n_14 = Node {
        value: 14,
        left: None,
        right: None,
    };
    let n_12 = Node {
        value: 12,
        left: None,
        right: None,
    };
    let n_11 = Node {
        value: 11,
        left: Some(Box::new(n_4)),
        right: Some(Box::new(n_14)),
    };
    let n_3 = Node {
        value: 3,
        left: None,
        right: Some(Box::new(n_12)),
    };
    Node {
        value: 5,
        left: Some(Box::new(n_11)),
        right: Some(Box::new(n_3)),
    }
}

//        -1
//      /   \
//    -6    -5
//   /  \     \
// -3   -4   -13
//     /       \
//    -2       -2

pub fn make_num_tree_3() -> Node<i32> {

    let n_neg2_l = Node {
        value: -2,
        left: None,
        right: None,
    };
    let n_neg2_r = Node {
        value: -2,
        left: None,
        right: None,
    };

    let n_neg3 = Node {
        value: -3,
        left: None,
        right: None,
    };
    let n_neg4 = Node {
        value: -4,
        left: Some(Box::new(n_neg2_l)),
        right: None,
    };
    let n_neg13 = Node {
        value: -13,
        left: None,
        right: Some(Box::new(n_neg2_r)),
    };

    let n_neg6 = Node {
        value: -6,
        left: Some(Box::new(n_neg3)),
        right: Some(Box::new(n_neg4)),
    };
    let n_neg5 = Node {
        value: -6,
        left: None,
        right: Some(Box::new(n_neg13)),
    };
    Node {
        value: -1,
        left: Some(Box::new(n_neg6)),
        right: Some(Box::new(n_neg5)),
    }
}

pub fn make_char_tree_3() -> Node<char> {
//      a
//       \
//        b
//       /
//      c
//    /  \
//   x    d
//         \
//          e

    let e = Node {
        value: 'e',
        left: None,
        right: None,
    };
    let d = Node {
        value: 'd',
        left: None,
        right: Some(Box::new(e)),
    };
    let x = Node {
        value: 'x',
        left: None,
        right: None,
    };
    let c = Node {
        value: 'c',
        left: Some(Box::new(x)),
        right: Some(Box::new(d)),
    };
    let b = Node {
        value: 'b',
        left: Some(Box::new(c)),
        right: None,
    };
    let a = Node {
        value: 'a',
        left: None,
        right: Some(Box::new(b)),
    };
    a
}

pub fn make_char_tree_2() -> Node<char> {
//      a
//    /   \
//   b     c
//  / \     \
// d   e     f
//    /       \
//   g         h

    let g = Node {
        value: 'g',
        left: None,
        right: None,
    };
    let h = Node {
        value: 'h',
        left: None,
        right: None,
    };
    let f = Node {
        value: 'f',
        left: None,
        right: Some(Box::new(h)),
    };
    let e = Node {
        value: 'e',
        left: Some(Box::new(g)),
        right: None,
    };
    let d = Node {
        value: 'd',
        left: None,
        right: None,
    };

    let c = Node {
        value: 'c',
        left: None,
        right: Some(Box::new(f)), 
    };
    let b = Node {
        value: 'b',
        left: Some(Box::new(d)),
        right: Some(Box::new(e)), 
    };

    let a = Node {
        value: 'a',
        left: Some(Box::new(b)),
        right: Some(Box::new(c)),
    };
    a
}

pub fn make_char_tree_1() -> Node<char> {
//      a
//    /   \
//   b     c
//  / \     \
// d   e     f
    let f = Node {
        value: 'f',
        left: None,
        right: None,
    };
    let e = Node {
        value: 'e',
        left: None,
        right: None,
    };
    let d = Node {
        value: 'd',
        left: None,
        right: None,
    };

    let c = Node {
        value: 'c',
        left: None,
        right: Some(Box::new(f)), 
    };
    let b = Node {
        value: 'b',
        left: Some(Box::new(d)),
        right: Some(Box::new(e)), 
    };

    let a = Node {
        value: 'a',
        left: Some(Box::new(b)),
        right: Some(Box::new(c)),
    };
    a
}

#[test]
fn contains_a() {
    let root = Box::new(make_char_tree_1());
    let result = tree_contains(&root, 'a');
    assert!(result == true);
}

#[test]
fn contains_e() {
    let root = Box::new(make_char_tree_1());
    let result = tree_contains(&root, 'g');
    assert!(result == false);
}



#[test]
fn level_order_char_tree_1() {
    let root = make_char_tree_1();
    let output: Vec<char> = LevelOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['a', 'b', 'c', 'd', 'e', 'f']);
}

#[test]
fn level_order_char_tree_2() {
    let root = make_char_tree_2();
    let output: Vec<char> = LevelOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
}

#[test]
fn level_order_char_tree_3() {
    let root = make_char_tree_3();
    let output: Vec<char> = LevelOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['a', 'b', 'c', 'x', 'd', 'e']);
}

#[test]
fn pre_order_char_tree_1() {
    let root = make_char_tree_1();
    let output: Vec<char> = PreOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['a', 'b', 'd', 'e', 'c', 'f']);
}

#[test]
fn pre_order_char_tree_2() {
    let root = make_char_tree_2();
    let output: Vec<char> = PreOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['a', 'b', 'd', 'e', 'g', 'c', 'f', 'h']);
}

#[test]
fn pre_order_char_tree_3() {
    let root = make_char_tree_3();
    let output: Vec<char> = PreOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['a', 'b', 'c', 'x', 'd', 'e']);
}

#[test]
fn post_order_char_tree_1() {
    let root = make_char_tree_1();
    let output: Vec<char> = PostOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['d', 'e', 'b', 'f', 'c', 'a']);
}

#[test]
fn post_order_char_tree_2() {
    let root = make_char_tree_2();
    let output: Vec<char> = PostOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['d', 'g', 'e', 'b', 'h', 'f', 'c', 'a']);
}

#[test]
fn post_order_char_tree_3() {
    let root = make_char_tree_3();
    let output: Vec<char> = PostOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['x', 'e', 'd', 'c', 'b', 'a']);
}

#[test]
fn in_order_char_tree_1() {
    let root = make_char_tree_1();
    let output: Vec<char> = InOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['d', 'b', 'e', 'a', 'c', 'f']);
}

#[test]
fn in_order_char_tree_2() {
    let root = make_char_tree_2();
    let output: Vec<char> = InOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['d', 'b', 'g', 'e', 'a', 'c', 'f', 'h']);
}

#[test]
fn in_order_char_tree_3() {
    let root = make_char_tree_3();
    let output: Vec<char> = InOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec!['a', 'x', 'c', 'd', 'e', 'b']);
}

// ---------------------------------------------------------------------------------------
// iterative tests
// ---------------------------------------------------------------------------------------

#[test]
fn test_in_order_iterative() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let mut output: Vec<i32> = Vec::new();
    inorder_iterative(&tree, |node_value, _level| {
        output.push(*node_value);
    });
    assert_eq!(output, vec![3, 2, 4, 1, 6, 5, 7]);
}

#[test]
fn test_pre_order_iterative() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let mut output: Vec<i32> = Vec::new();
    preorder_iterative(&tree, |node_value, _level| {
        output.push(*node_value);
    });
    assert_eq!(output, vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_post_order_iterative() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let mut output: Vec<i32> = Vec::new();
    postorder_iterative(&tree, |node_value, _level| {
        output.push(*node_value);
    });
    assert_eq!(output, vec![3, 4, 2, 6, 7, 5, 1]);
}

#[test]
fn test_level_order_iterative() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let mut output: Vec<i32> = Vec::new();
    levelorder_iterative(&tree, |node_value| {
        output.push(*node_value);
    });
    assert_eq!(output, vec![1, 2, 5, 3, 4, 6, 7]);
}

// ---------------------------------------------------------------------------------------
// iterator tests
// ---------------------------------------------------------------------------------------

#[test]
fn test_in_order_iterator() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let root = tree.as_ref().unwrap();
    let output: Vec<i32> = InOrderIterator::new(root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec![3, 2, 4, 1, 6, 5, 7]);
}

#[test]
fn test_pre_order_iterator() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let root = tree.as_ref().unwrap();
    let output: Vec<i32> = PreOrderIterator::new(root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_post_order_iterator() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let root = tree.as_ref().unwrap();
    let output: Vec<i32> = PostOrderIterator::new(root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec![3, 4, 2, 6, 7, 5, 1]);
}

#[test]
fn test_level_order_iterator() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let root = tree.as_ref().unwrap();
    let output: Vec<i32> = LevelOrderIterator::new(root)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(output, vec![1, 2, 5, 3, 4, 6, 7]);
}

// ---------------------------------------------------------------------------------------
// recursive tests
// ---------------------------------------------------------------------------------------

/*
#[test]
fn test_in_order_recursive() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter).unwrap();
    let mut output: Vec<i32> = Vec::new();
    inorder_recursive(&tree, |node_value| {
        output.push(*node_value);
    });
    assert_eq!(output, vec![3, 2, 4, 1, 6, 5, 7]);
}

#[test]
fn test_pre_order_recursive() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter).unwrap();
    let mut output: Vec<i32> = Vec::new();
    preorder_recursive(&tree, |node_value, _level| {
        output.push(*node_value);
    });
    assert_eq!(output, vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_post_order_recursive() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter).unwrap();
    let mut output: Vec<i32> = Vec::new();
    postorder_recursive(&tree, |node_value, _level| {
        output.push(*node_value);
    });
    assert_eq!(output, vec![3, 4, 2, 6, 7, 5, 1]);
}

#[test]
fn test_level_order_recursive() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let mut output: Vec<i32> = Vec::new();
    levelorder_recursive(&tree, |node_value, _level| {
        output.push(*node_value);
    });
    assert_eq!(output, vec![1, 2, 5, 3, 4, 6, 7]);
}
*/



