// https://youtu.be/QkuNmL7tz08?t=2785

// maybe a revalation https://youtu.be/QkuNmL7tz08?t=4308

use std::fmt::Display;
use std::collections::VecDeque;

type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Default)]
struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

impl<T> std::fmt::Debug for Node<T>
where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Node<T> {
    fn height(&self) -> Option<usize> {
        self.height_recursive()
    }

    fn height_recursive(&self) -> Option<usize> {
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

    fn height_iterative(&self) -> usize {
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

    fn print_level(&self, level: usize) {
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

    fn print_recursive(&self, level: usize) {
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

fn generate_tree<T>(level: usize, counter: &mut T) -> NodeRef<T> 
where T: std::ops::AddAssign<i32> + Copy
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


fn invert_tree<T: Clone>(root: &NodeRef<T>) -> NodeRef<T> {
    match root {
        Some(node) => Some(Box::new(Node {
            value: node.value.clone(),
            left: invert_tree(&node.right),
            right: invert_tree(&node.left),
        })),
        None => {
            None
        }
    }
}

/*
Algorithm Inorder(tree)
   1. Traverse the left subtree, i.e., call Inorder(left-subtree)
   2. Visit the root.
   3. Traverse the right subtree, i.e., call Inorder(right-subtree)
*/

fn inorder_recursive<T: Display>(node: &Box<Node<T>>) {
    if let Some(left) = &node.left {
        inorder_recursive(&left);
    }
    print!("{} ", node.value);
    if let Some(right) = &node.right {
        inorder_recursive(&right);
    }
}

fn inorder_iterative<T, F>(root: &NodeRef<T>, mut call_me: F)
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

struct InOrderIterator<'a, T> {
    stack: Vec<&'a Node<T>>,
    current: Option<&'a Node<T>>,
}

impl<'a, T> InOrderIterator<'a, T> {
    fn new(root: &'a Node<T>) -> Self {
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

/*
Algorithm Preorder(tree)
   1. Visit the root.
   2. Traverse the left subtree, i.e., call Preorder(left-subtree)
   3. Traverse the right subtree, i.e., call Preorder(right-subtree) 
*/
fn preorder_recursive<T: Display>(node: &Box<Node<T>>) {
    print!("{} ", node.value);
    if let Some(left) = &node.left {
        preorder_recursive(left);
    }
    if let Some(right) = &node.right {
        preorder_recursive(right);
    }
}

fn preorder_iterative<T, F>(root: &NodeRef<T>, mut call_me: F)
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

struct PreOrderIterator<'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> PreOrderIterator<'a, T> {
    fn new(root: &'a Node<T>) -> Self {
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


/*
Algorithm Postorder(tree)
   1. Traverse the left subtree, i.e., call Postorder(left-subtree)
   2. Traverse the right subtree, i.e., call Postorder(right-subtree)
   3. Visit the root.
*/

fn postorder_recursive<T, F>(node: &Box<Node<T>>, mut call_me: F)
where T: Display, F: FnMut(&T, usize)
{
    if let Some(left) = &node.left {
        postorder_recursive(&left, &mut call_me);
    }
    if let Some(right) = &node.right {
        postorder_recursive(&right, &mut call_me);
    }
    call_me(&node.value, 0);
}

// ok this is going to be a little more complicated
/*
1. Push root to first stack.
2. Loop while first stack is not empty
   2.1 Pop a node from first stack and push it to second stack
   2.2 Push left and right children of the popped node to first stack
3. Print contents of second stack
*/

fn postorder_iterative<T, F>(root: &NodeRef<T>, mut call_me: F)
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

struct PostOrderIterator<'a, T> {
    stack: Vec<&'a Node<T>>,
    stack_two: Vec<&'a Node<T>>,
}

impl<'a, T> PostOrderIterator<'a, T> {
    fn new(root: &'a Node<T>) -> Self {
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



fn levelorder_iterative<T, F>(root: &NodeRef<T>, mut call_me: F)
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

fn levelorder_recursive<T: Display>(node: &NodeRef<T>) {
    if node.is_none() {
        return;
    }
    let node = node.as_ref().unwrap();
    let h = node.height().unwrap_or(0);
    for i in 1..=h {
        node.print_level(i);
    }
}

struct LevelOrderIterator<'a, T> {
    queue: VecDeque<&'a Node<T>>,
}

impl<'a, T> LevelOrderIterator<'a, T> {
    fn new(root: &'a Node<T>) -> Self {
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


fn main() {
    println!("Hello Trees");
    print_everything();
}

// -----------------------------------------------
// iterative
// -----------------------------------------------

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

// ----------------------------------------------------------

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

// -----------------------------------------------
// recursive
// -----------------------------------------------

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

fn print_everything() {
    let mut counter = 1;
    let tree = generate_tree(3, &mut counter);
    //let inverted = invert_tree(&tree);
    let root = tree.as_ref().unwrap();
    println!("left {}", root.left.as_ref().unwrap().value);

    println!("----print-recursive------------");
    root.print_recursive(0);

    println!("----print-iterative------------");
    inorder_iterative(&tree, |node, level| {
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", node);
    });

    println!("----inorder-recursive----------");
    inorder_recursive(tree.as_ref().unwrap());
    println!();
    println!("----inorder-iterative----------");
    inorder_iterative(&tree, |node, _level| {
        print!("{} ", node);
    });
    println!();
    println!("----InOrderIterator------------");
    let it = InOrderIterator::new(root);
    for i in it {
        print!("{} ", *i);
    }
    println!();
    println!();

    println!("----preorder-recursive---------");
    preorder_recursive(tree.as_ref().unwrap());
    println!();
    println!("----preorder-iterative---------");
    preorder_iterative(&tree, |node, _level| {
        print!("{} ", node);
    });
    println!();
    println!("----PreOrderIterator-----------");
    let it = PreOrderIterator::new(root);
    for i in it {
        print!("{} ", *i);
    }
    println!();
    println!();

    println!("----postorder-recursive--------");
    postorder_recursive(tree.as_ref().unwrap(), |node, _level| {
        print!("{} ", node);
    });
    println!();
    println!("----postorder-iterative--------");
    postorder_iterative(&tree, |node, _level| {
        print!("{} ", node);
    });
    println!();
    println!("----PostOrderIterator----------");
    let it = PostOrderIterator::new(root);
    for i in it {
        print!("{} ", *i);
    }
    println!();
    println!();

    println!("----levelorder-recursive-------");
    levelorder_recursive(&tree);
    println!();
    println!("----levelorder-iterative-------");
    levelorder_iterative(&tree, |node| {
        print!("{} ", node);
    });
    println!();
    println!("----LevelOrderIterator---------");
    let it = LevelOrderIterator::new(root);
    for i in it {
        print!("{} ", *i);
    }
    println!();

    println!();
    println!("{} - height", tree.as_ref().unwrap().height_recursive().unwrap());
    println!("{} - height iterative", tree.as_ref().unwrap().height_iterative());
}






