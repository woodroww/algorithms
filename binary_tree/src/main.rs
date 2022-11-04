// https://youtu.be/QkuNmL7tz08?t=2785

// maybe a revalation https://youtu.be/QkuNmL7tz08?t=4308

use binary_tree::tree::*;
use binary_tree::tree::height;
use binary_tree::tree_creation::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
//{InOrderIterator, Node, PreOrderIterator, PostOrderIterator, LevelOrderIterator};



fn main() {
    println!("Hello Trees");
    let tree = make_num_tree_7();
    let tree = Some(Rc::new(RefCell::new(tree)));
    //tree.print_recursive(0);
    levelorder_recursive(tree);
    /*
    let mut counter = 1;
    let tree = generate_tree(4, &mut counter).unwrap();
    tree.print_recursive(0);

    print_everything();
    let max = max_path_sum(&Some(Box::new(make_tree())));
    println!("max path sum {}", max.unwrap());
    let letters = make_char_tree_2();
    letters.print_recursive(0);
    */
}







fn print_everything() {
    let mut counter = 1;
    let tree = generate_tree(3, &mut counter).unwrap();
    //let inverted = invert_tree(&tree);

    let cell: &RefCell<Node<i32>> = tree.borrow();
    let root = cell.borrow();

    let left: NodeRef<i32> = root.left().unwrap();
    let cell: &RefCell<Node<i32>> = left.borrow();
    let left = cell.borrow();
    println!("left {}", left.value);

    println!("----print-recursive------------");
    root.print_recursive(0);

    println!("----print-iterative------------");
    inorder_iterative(Some(Rc::clone(&tree)), |node, level| {
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", node);
    });

    println!("----inorder-recursive----------");
    inorder_recursive(Rc::clone(&tree));
    println!();
    println!("----inorder-iterative----------");
    inorder_iterative(Some(Rc::clone(&tree)), |node, _level| {
        print!("{} ", node);
    });
    println!();
    println!("----InOrderIterator------------");
    let it = InOrderIterator::new(Rc::clone(&tree));
    for i in it {
        print!("{} ", i);
    }
    println!();
    println!();

    println!("----preorder-recursive---------");
    preorder_recursive(Rc::clone(&tree));
    println!();
    println!("----preorder-iterative---------");
    preorder_iterative(Some(Rc::clone(&tree)), |node, _level| {
        print!("{} ", node);
    });
    println!();
    println!("----PreOrderIterator-----------");
    let it = PreOrderIterator::new(Rc::clone(&tree));
    for i in it {
        print!("{} ", i);
    }
    println!();
    println!();

    println!("----postorder-recursive--------");
    postorder_recursive(Rc::clone(&tree));
    println!();
    println!("----postorder-iterative--------");
    postorder_iterative(Some(Rc::clone(&tree)), |node, _level| {
        print!("{} ", node);
    });
    println!();
    println!("----PostOrderIterator----------");
    let it = PostOrderIterator::new(Rc::clone(&tree));
    for i in it {
        print!("{} ", i);
    }
    println!();
    println!();

    println!("----levelorder-recursive-------");
    levelorder_recursive(Some(Rc::clone(&tree)));
    println!();
    println!("----levelorder-iterative-------");
    levelorder_iterative(Some(Rc::clone(&tree)), |node| {
        print!("{} ", node);
    });
    println!();
    println!("----LevelOrderIterator---------");
    let it = LevelOrderIterator::new(Rc::clone(&tree));
    for i in it {
        print!("{} ", i);
    }
    println!();

    println!();
    println!(
        "{} - height",
        height(Some(Rc::clone(&tree)))
    );
    println!(
        "{} - height iterative",
        height_iterative(Rc::clone(&tree))
    );
}
