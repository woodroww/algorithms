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

impl<T: Display> Node<T> {

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

fn inorder_iterative<T, F>(root: &NodeRef<T>, call_me: F)
where T: Display, F: Fn(&T, usize)
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

fn preorder_iterative<T, F>(root: &NodeRef<T>, call_me: F)
where T: Display, F: Fn(&T, usize)
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

/*
Algorithm Postorder(tree)
   1. Traverse the left subtree, i.e., call Postorder(left-subtree)
   2. Traverse the right subtree, i.e., call Postorder(right-subtree)
   3. Visit the root.
*/

fn postorder_resursive<T: Display>(node: &Box<Node<T>>) {
    if let Some(left) = &node.left {
        postorder_resursive(&left);
    }
    if let Some(right) = &node.right {
        postorder_resursive(&right);
    }
    print!("{} ", node.value);
}

// ok this is going to be a little more complicated
/*
1. Push root to first stack.
2. Loop while first stack is not empty
   2.1 Pop a node from first stack and push it to second stack
   2.2 Push left and right children of the popped node to first stack
3. Print contents of second stack
*/

fn postorder_iterative<T, F>(root: &NodeRef<T>, call_me: F)
where T: Display, F: Fn(&T, usize)
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

fn levelorder_iterative<T, F>(root: &NodeRef<T>, call_me: F)
where T: Display, F: Fn(&T, usize)
{
    let mut queue: VecDeque<&Node<T>> = VecDeque::new();
    queue.push_back(root.as_ref().unwrap());
    let root_height = root.as_ref().unwrap().height().unwrap_or(0);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        let h = node.height().unwrap_or(0);
        call_me(&node.value, root_height - h);

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



fn main() {
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

    println!("----preorder-recursive---------");
    preorder_recursive(tree.as_ref().unwrap());
    println!();
    println!("----preorder-iterative---------");
    preorder_iterative(&tree, |node, _level| {
        print!("{} ", node);
    });
    println!();

    println!("----postorder-recursive--------");
    postorder_resursive(tree.as_ref().unwrap());
    println!();
    println!("----postorder-iterative--------");
    postorder_iterative(&tree, |node, _level| {
        print!("{} ", node);
    });
    println!();

    println!("----levelorder-recursive-------");
    levelorder_recursive(&tree);
    println!();
    println!("----levelorder-iterative-------");
    levelorder_iterative(&tree, |node, _level| {
        print!("{} ", node);
    });
    println!();

    println!();
    println!("{} - height", tree.as_ref().unwrap().height_recursive().unwrap());
    println!("{} - height iterative", tree.as_ref().unwrap().height_iterative());
}






