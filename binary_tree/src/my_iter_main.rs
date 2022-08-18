
#[derive(Debug, Default)]
struct Node<T>
where T: std::fmt::Display {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn generate_tree<T>(level: usize, value: &mut dyn Iterator<Item = T>) -> Option<Box<Node<T>>>
where T: std::fmt::Display
{
    if level == 0 {
        return None;
    }

    let node = Node {
        value: value.next().unwrap(),
        left: generate_tree(level - 1, value),
        right: generate_tree(level - 1, value),
    };

    Some(Box::new(node))
}

fn print_tree<T>(root: Option<Box<Node<T>>>, level: usize)
where T: std::fmt::Display
{
    if let Some(node) = root {
        print_tree(node.left, level + 1);
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", node.value);
        print_tree(node.right, level + 1);
    }
}

fn main() {
    let mut value_iter = 1..;
    let tree = generate_tree(3, &mut value_iter);
    print_tree(tree, 0);
}
