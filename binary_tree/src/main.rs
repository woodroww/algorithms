// https://youtu.be/QkuNmL7tz08?t=2785

// maybe a revalation https://youtu.be/QkuNmL7tz08?t=4308

use std::fmt::Display;
use std::fmt::Debug;
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

fn print_recursive<T: Display>(root: &NodeRef<T>, level: usize) {
    if let Some(node) = root {
        print_recursive(&node.left, level + 1);
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", node.value);
        print_recursive(&node.right, level + 1);
    }
}


fn print_iterative<T: Display>(root: &NodeRef<T>) {
    inorder_fun(root, |node, level| {
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", node);
    });
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

fn inorder_fun<T, F>(root: &NodeRef<T>, call_me: F)
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

fn inorder_iterative<T>(root: &NodeRef<T>)
where T: Display
{
    if root.is_none() {
        return;
    }
    let mut stack: Vec<&Box<Node<T>>> = Vec::new();
    let mut current: Option<&Box<Node<T>>> = root.as_ref();

    loop {
        if current.is_some() {
            let node = current.unwrap();
            stack.push(node);
            current = node.left.as_ref();
        } else {
            match stack.pop() {
                Some(node) => {
                    print!("{} ", node.value);
                    current = node.right.as_ref();
                }
                None => {
                    break;
                }
            }
        }
    }
    println!();
}

fn preorder_iterative<T: Display>(root: &NodeRef<T>) {
    if root.is_none() {
        return;
    }
    let mut stack: Vec<&Box<Node<T>>> = Vec::new();
    stack.push(root.as_ref().unwrap());

    while let Some(node) = stack.pop() {
        print!("{} ", node.value);
        if let Some(right) = node.right.as_ref() {
            stack.push(right);
        }
        if let Some(left) = node.left.as_ref() {
            stack.push(left);
        }
    }
    println!();
}

// ok this is going to be a little more complicated
/*
1. Push root to first stack.
2. Loop while first stack is not empty
   2.1 Pop a node from first stack and push it to second stack
   2.2 Push left and right children of the popped node to first stack
3. Print contents of second stack
*/

fn postorder_iterative<T>(root: &NodeRef<T>)
where T: Display
{
    if root.is_none() {
        return;
    }
    let mut stack: Vec<&Box<Node<T>>> = Vec::new();
    stack.push(root.as_ref().unwrap());

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
        print!("{} ", node.value);
    }
    println!();
}

fn levelorder_iterative<T: Display>(root: &NodeRef<T>) {
    let mut queue: VecDeque<&Node<T>> = VecDeque::new();
    queue.push_back(root.as_ref().unwrap());

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        // action here
        print!("{} ", node.value);
        if let Some(left) = &node.left {
            queue.push_back(left);
        }
        if let Some(right) = &node.right {
            queue.push_back(right);
        }
    }
    println!();
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
    println!();
}



fn main() {
    let mut counter = 1;
    let tree = generate_tree(3, &mut counter);
    //print_tree(&tree, 0);
    let inverted = invert_tree(&tree);
    println!("----print-recursive------------");
    print_recursive(&inverted, 0);
    println!("----print-iterative------------");
    print_iterative(&inverted);
    println!("----inorder--------------------");
    inorder_iterative(&tree);
    println!("----preorder-------------------");
    preorder_iterative(&tree);
    println!("----postorder------------------");
    postorder_iterative(&tree);
    println!("----levelorder-recursive-------");
    levelorder_recursive(&tree);
    println!("----levelorder-iterative-------");
    levelorder_iterative(&tree);

    println!("{} - height", tree.as_ref().unwrap().height_recursive().unwrap());
    println!("{} - height iterative", tree.as_ref().unwrap().height_iterative());
}






