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

fn main() {
    let mut counter = 1;
    let tree = generate_tree(3, &mut counter);
    print_tree(&tree, 0);
    println!("-----------------------");
    print_tree(&invert_tree(&tree), 0);
}






