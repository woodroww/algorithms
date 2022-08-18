// https://youtu.be/QkuNmL7tz08?t=2785

// maybe a revalation https://youtu.be/QkuNmL7tz08?t=4308

use std::fmt::{Display, Debug};

type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

impl<T: Display> Node<T> {
    fn height(&self) -> Option<usize> {
        let l_depth = match &self.left {
            Some(left) => match left.height() {
                Some(h) => h,
                None => 0,
            }
            None => {
                0
            }
        };
        let r_depth = match &self.right {
            Some(right) => match right.height() {
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

    fn print_level(&self, level: usize) {
        if level == 1 {
            print!("{} ", self.value);
        } else if level > 1 {
            if let Some(right) = self.right.as_ref() {
                right.print_level(level - 1);
            }
            if let Some(left) = self.left.as_ref() {
                left.print_level(level - 1);
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

fn print_tree<T>(root: &NodeRef<T>, level: usize)
where T: Display
{
    let switch = false;

    if switch {
        if let Some(node) = root {
            print_tree(&node.right, level + 1);
            for _ in 0..level {
                print!("  ");
            }
            println!("{}", node.value);
            print_tree(&node.left, level + 1);
        }
    } else {
        if let Some(node) = root {
            print_tree(&node.left, level + 1);
            for _ in 0..level {
                print!("  ");
            }
            println!("{}", node.value);
            print_tree(&node.right, level + 1);
        }
    }
}


fn print_tree_iterative<T: Display>(root: &NodeRef<T>) {
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
/*
1. Push root to first stack.
2. Loop while first stack is not empty
   2.1 Pop a node from first stack and push it to second stack
   2.2 Push left and right children of the popped node to first stack
3. Print contents of second stack
*/

fn postorder_traversal<T>(root: &NodeRef<T>)
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


fn levelorder_traversal<T: Display>(node: &NodeRef<T>) {
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
    println!("-----------------------");
    print_tree(&inverted, 0);
    println!("-----------------------");
    print_tree_iterative(&inverted);
    println!("----inorder--------------------");
    inorder_traversal(&tree);
    println!("----preorder-------------------");
    preorder_traversal(&tree);
    println!("----postorder------------------");
    postorder_traversal(&tree);
    println!("----levelorder-----------------");
    levelorder_traversal(&tree);

}






