// https://youtu.be/QkuNmL7tz08?t=2785
use std::fmt::Display;

type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
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

fn print_tree<T>(root: &NodeRef<T>, level: usize)
where T: Display
{
    // this here left and right are switched from main_recursive.rs
    // now instead of mentally transposing the tree to see a top down view
    // mentally rotate it instead
    if let Some(node) = root {
        print_tree(&node.left, level + 1);
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", node.value);
        print_tree(&node.right, level + 1);
    }
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

fn inorder_traversal<T>(root: &NodeRef<T>)
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

fn preorder_traversal<T: Display>(root: &NodeRef<T>) {
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
fn postorder_traversal<T: Display>(root: &NodeRef<T>) {
    if root.is_none() {
        return;
    }
    let mut stack: Vec<&Box<Node<T>>> = Vec::new();
    stack.push(root.as_ref().unwrap());

    while let Some(node) = stack.pop() {
        if let Some(left) = node.left.as_ref() {
            stack.push(left);
        }
        if let Some(right) = node.right.as_ref() {
            stack.push(right);
        }
        print!("{} ", node.value);
    }
    println!();
}




fn main() {
    let mut counter = 1;
    let tree = generate_tree(3, &mut counter);
    //print_tree(&tree, 0);
    println!("-----------------------");
    print_tree(&invert_tree(&tree), 0);
    println!("----inorder--------------------");
    inorder_traversal(&tree);
    println!("----preorder-------------------");
    preorder_traversal(&tree);
    println!("----postorder------------------");
    postorder_traversal(&tree);
}






