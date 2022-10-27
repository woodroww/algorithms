// https://youtu.be/QkuNmL7tz08?t=2785

// maybe a revalation https://youtu.be/QkuNmL7tz08?t=4308

use binary_tree::tree::*;
use binary_tree::tree::height;
use binary_tree::tree_creation::*;
//{InOrderIterator, Node, PreOrderIterator, PostOrderIterator, LevelOrderIterator};



fn main() {
    println!("Hello Trees");
    let tree = make_num_tree_7();
    let tree_box = Box::new(tree);

    levelorder_recursive_print(Some(&tree_box));
    let tree_2 = balanced(&tree_box);
    let tree_2_box = Box::new(tree_2);

    levelorder_recursive_print(Some(&tree_2_box));

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
    let tree = generate_tree(3, &mut counter);
    //let inverted = invert_tree(&tree);
    let root = tree.as_ref().unwrap();
    println!("left {}", root.left().unwrap().value());

    println!("----print-recursive------------");
    root.print_recursive(0);

    println!("----print-iterative------------");
    inorder_iterative(tree.as_ref(), |node, level| {
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", node);
    });

    println!("----inorder-recursive----------");
    inorder_recursive(tree.as_ref().unwrap());
    println!();
    println!("----inorder-iterative----------");
    inorder_iterative(tree.as_ref(), |node, _level| {
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
    preorder_iterative(tree.as_ref(), |node, _level| {
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
    postorder_recursive(tree.as_ref().unwrap());
    println!();
    println!("----postorder-iterative--------");
    postorder_iterative(tree.as_ref(), |node, _level| {
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
    levelorder_recursive_print(tree.as_ref());
    println!();
    println!("----levelorder-iterative-------");
    levelorder_iterative(tree.as_ref(), |node| {
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
    println!(
        "{} - height",
        height(Some(tree.as_ref().unwrap()))
    );
    println!(
        "{} - height iterative",
        tree.as_ref().unwrap().height_iterative()
    );
}
