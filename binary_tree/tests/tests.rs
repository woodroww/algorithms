use binary_tree::tree::*; 
use binary_tree::tree_creation::*;

#[test]
fn test_balance_1() {
    let n_3 = Node {
        value: 10,
        left: None,
        right: None,
    };
    let n_2 = Node {
        value: 20,
        left: Some(Box::new(n_3)),
        right: None,
    };
    let root = Node {
        value: 30,
        left: Some(Box::new(n_2)),
        right: None,
    };

    let balanced_tree = balanced(&root);
    let result: Vec<i32> = LevelOrderIterator::new(&balanced_tree)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(result, vec![20, 10, 30]);
}

#[test]
fn test_balance_2() {
    let n_1 = Node {
        value: 1,
        left: None,
        right: None,
    };
    let n_2 = Node {
        value: 2,
        left: Some(Box::new(n_1)),
        right: None,
    };
    let n_3 = Node {
        value: 3,
        left: Some(Box::new(n_2)),
        right: None,
    };

    let n_7 = Node {
        value: 7,
        left: None,
        right: None,
    };
    let n_6 = Node {
        value: 6,
        left: None,
        right: Some(Box::new(n_7)),
    };
    let n_5 = Node {
        value: 5,
        left: None,
        right: Some(Box::new(n_6)),
    };

    let root = Node {
        value: 4,
        left: Some(Box::new(n_3)),
        right: Some(Box::new(n_5)),
    };

    let balanced_tree = balanced(&root);
    let result: Vec<i32> = LevelOrderIterator::new(&balanced_tree)
        .map(|x| x.to_owned())
        .collect();
    assert_eq!(result, vec![4, 2, 6, 1, 3, 5, 7]);
}

#[test]
fn test_order_1() {

    // this should work for all make_num_tree_*
    let root = make_num_tree_4();
    let mut arr: Vec<i32> = InOrderIterator::new(&root)
        .map(|x| x.to_owned())
        .collect();
    let ordered_tree = Node::from_array(&arr);
    let result: Vec<i32> = InOrderIterator::new(&ordered_tree)
        .map(|x| x.to_owned())
        .collect();
    arr.sort();
    assert_eq!(result, arr);
}


#[test]
fn test_max_depth_1() {
    let root = Box::new(make_num_tree_1());
    let depth = max_depth(Some(&root));
    assert_eq!(depth, 2);
}

#[test]
fn test_max_depth_2() {
    let root = Box::new(make_num_tree_3());
    let depth = max_depth(Some(&root));
    assert_eq!(depth, 3);
}

#[test]
fn test_max_depth_3() {
    let root = Box::new(make_char_tree_3());
    let depth = max_depth(Some(&root));
    assert_eq!(depth, 4);
}

#[test]
fn test_max_depth_4() {
    let root = Box::new(make_char_tree_3());
    let depth = max_depth(root.right.as_ref());
    assert_eq!(depth, 3);
}

#[cfg(test)]
mod max_width_tests {
    use super::*;
    #[test]
    fn max_width_1() {
        let root = Box::new(make_num_tree_1());
        let max = max_width(Some(&root));
        assert_eq!(max, 3);
    }

    #[test]
    fn max_width_2() {
        let root = Box::new(make_num_tree_2());
        let max = max_width(Some(&root));
        assert_eq!(max, 3);
    }

    #[test]
    fn max_width_3() {
        let root = generate_tree(3, &mut 1);
        let max = max_width(root.as_ref());
        assert_eq!(max, 4);
    }

    #[test]
    fn max_width_4() {
        let root = generate_tree(4, &mut 1);
        let max = max_width(root.as_ref());
        assert_eq!(max, 8);
    }
}

#[test]
fn test_insert_1() {

    //pub fn insert(&mut self, value: T) {
//pub fn make_num_tree_6() -> Node<i32> {
    //        5
    //     /    \
    //    11    54
    //  /   \
    // 20   15
    //      / \
    //     1  3

    let mut root = Box::new(make_num_tree_6());
    root.insert(13);
    root.print_recursive(0);
    assert_eq!(root.left.unwrap().right.unwrap().right.unwrap().value, 13);
}

#[test]
fn test_diameter_1() {
    let root = Box::new(make_num_tree_8());
    let diameter = diameter(Some(&root));
    assert_eq!(diameter, 3);
}

#[test]
fn test_diameter_2() {
    let root = Box::new(make_num_tree_9());
    let diameter = diameter(Some(&root));
    assert_eq!(diameter, 6);
}


// ---------------------------------------------------------------------------------------
// tree_sum tests
// ---------------------------------------------------------------------------------------

// generate_tree with (3, 1) will generate this:
//     1
//  2     5
// 3 4   6 7

#[test]
fn tree_sum_1() {
    let root = generate_tree(3, &mut 1);
    let sum = tree_sum(root.as_ref());
    assert_eq!(sum.unwrap(), 28);
    let sum = tree_sum_iterative(root.as_ref());
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
    let sum = tree_sum(root.as_ref());
    assert_eq!(sum.unwrap(), 120);
    let sum = tree_sum_iterative(root.as_ref());
    assert_eq!(sum.unwrap(), 120);
}

#[test]
fn tree_sum_3() {
    let root = Box::new(make_num_tree_1());
    let sum = tree_sum(Some(&root));
    assert_eq!(sum.unwrap(), 21);
    let sum = tree_sum_iterative(Some(&Box::new(make_num_tree_1())));
    assert_eq!(sum.unwrap(), 21);
}

#[test]
fn tree_sum_4() {
    let root = Box::new(make_num_tree_4());
    let sum = tree_sum(Some(&root));
    assert_eq!(sum.unwrap(), 10);
    let sum = tree_sum_iterative(Some(&Box::new(make_num_tree_4())));
    assert_eq!(sum.unwrap(), 10);
}


// ---------------------------------------------------------------------------------------
// minimum tests
// ---------------------------------------------------------------------------------------

#[test]
fn minimum_1() {
    let root = Box::new(make_num_tree_1());
    let result = minimum(Some(&root));
    assert_eq!(result.unwrap(), -2);
}

#[test]
fn minimum_2() {
    let root = Box::new(make_num_tree_2());
    let result = minimum(Some(&root));
    assert_eq!(result.unwrap(), 3);
}

#[test]
fn minimum_3() {
    let root = Box::new(make_num_tree_3());
    let result = minimum(Some(&root));
    assert_eq!(result.unwrap(), -13);
}

#[test]
fn minimum_4() {
    let root = Box::new(Node {
        value: 42,
        left: None,
        right: None,
    });
    let result = minimum(Some(&root));
    assert_eq!(result.unwrap(), 42);
}


// ---------------------------------------------------------------------------------------
// max_path_sum tests
// ---------------------------------------------------------------------------------------

#[test]
fn max_path_sum_test_1() {
    let root = Box::new(make_num_tree_1());
    let max_sum = max_path_sum(Some(&root));
    assert_eq!(max_sum.unwrap(), 18);
}

#[test]
fn max_path_sum_test_2() {
    let root = Box::new(make_num_tree_6());
    let max_sum = max_path_sum(Some(&root));
    assert_eq!(max_sum.unwrap(), 59);
}

#[test]
fn max_path_sum_test_3() {
    let root = Box::new(make_num_tree_7());
    let max_sum = max_path_sum(Some(&root));
    assert_eq!(max_sum.unwrap(), -8);
}

#[test]
fn max_path_sum_test_4() {
    let root = Box::new(Node {
        value: 42,
        left: None,
        right: None,
    });
    let max_sum = max_path_sum(Some(&root));
    assert_eq!(max_sum.unwrap(), 42);
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
    let output: Vec<char> = PreOrderIterator::new(&root).map(|x| x.to_owned()).collect();
    assert_eq!(output, vec!['a', 'b', 'd', 'e', 'c', 'f']);
}

#[test]
fn pre_order_char_tree_2() {
    let root = make_char_tree_2();
    let output: Vec<char> = PreOrderIterator::new(&root).map(|x| x.to_owned()).collect();
    assert_eq!(output, vec!['a', 'b', 'd', 'e', 'g', 'c', 'f', 'h']);
}

#[test]
fn pre_order_char_tree_3() {
    let root = make_char_tree_3();
    let output: Vec<char> = PreOrderIterator::new(&root).map(|x| x.to_owned()).collect();
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
    let output: Vec<char> = InOrderIterator::new(&root).map(|x| x.to_owned()).collect();
    assert_eq!(output, vec!['d', 'b', 'e', 'a', 'c', 'f']);
}

#[test]
fn in_order_char_tree_2() {
    let root = make_char_tree_2();
    let output: Vec<char> = InOrderIterator::new(&root).map(|x| x.to_owned()).collect();
    assert_eq!(output, vec!['d', 'b', 'g', 'e', 'a', 'c', 'f', 'h']);
}

#[test]
fn in_order_char_tree_3() {
    let root = make_char_tree_3();
    let output: Vec<char> = InOrderIterator::new(&root).map(|x| x.to_owned()).collect();
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
    inorder_iterative(tree.as_ref(), |node_value, _level| {
        output.push(*node_value);
    });
    assert_eq!(output, vec![3, 2, 4, 1, 6, 5, 7]);
}

#[test]
fn test_pre_order_iterative() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let mut output: Vec<i32> = Vec::new();
    preorder_iterative(tree.as_ref(), |node_value, _level| {
        output.push(*node_value);
    });
    assert_eq!(output, vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_post_order_iterative() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let mut output: Vec<i32> = Vec::new();
    postorder_iterative(tree.as_ref(), |node_value, _level| {
        output.push(*node_value);
    });
    assert_eq!(output, vec![3, 4, 2, 6, 7, 5, 1]);
}

#[test]
fn test_level_order_iterative() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let mut output: Vec<i32> = Vec::new();
    levelorder_iterative(tree.as_ref(), |node_value| {
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
    let output: Vec<i32> = InOrderIterator::new(root).map(|x| x.to_owned()).collect();
    assert_eq!(output, vec![3, 2, 4, 1, 6, 5, 7]);
}

#[test]
fn test_pre_order_iterator() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let root = tree.as_ref().unwrap();
    let output: Vec<i32> = PreOrderIterator::new(root).map(|x| x.to_owned()).collect();
    assert_eq!(output, vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_post_order_iterator() {
    let mut counter: i32 = 1;
    let tree = generate_tree(3, &mut counter);
    let root = tree.as_ref().unwrap();
    let output: Vec<i32> = PostOrderIterator::new(root).map(|x| x.to_owned()).collect();
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


#[test]
fn height_1() {
    let root = Box::new(make_num_tree_1());
    let h = height(Some(&root));
    assert_eq!(h, 2);
}

#[test]
fn height_2() {
    let root = Box::new(make_num_tree_3());
    let h = height(Some(&root));
    assert_eq!(h, 3);
}

#[test]
fn height_3() {
    let root = Box::new(make_char_tree_3());
    let h = height(Some(&root));
    assert_eq!(h, 4);
}

#[test]
fn height_4() {
    let level = 4;
    let root = generate_tree(level, &mut 1).unwrap();
    let h = height(Some(&root));
    assert_eq!(h, level as isize - 1);
}

#[test]
fn height_5() {
    let level = 10;
    let root = generate_tree(level, &mut 1).unwrap();
    let h = height(Some(&root));
    assert_eq!(h, level as isize - 1);
}


#[test]
fn depth_1() {
    let root = generate_tree(3, &mut 1).unwrap();

    let d = depth(Some(&root), &root);
    assert_eq!(d, 0);

    let search = root.left.as_ref().unwrap();
    let d = depth(Some(&root), search);
    assert_eq!(d, 1);

    let search = root.left.as_ref().unwrap().left.as_ref().unwrap();
    let d = depth(Some(&root), search);
    assert_eq!(d, 2);
}

#[test]
fn depth_2() {
    //      a
    //       \
    //        b
    //       /
    //      c
    //    /  \
    //   x    d
    //         \
    //          e
    let root = Box::new(make_char_tree_3());
    let search = root.right.as_ref().unwrap();
    let search = search.left.as_ref().unwrap();
    let search = search.right.as_ref().unwrap();
    let search = search.right.as_ref().unwrap();
    let d = depth(Some(&root), search);
    assert_eq!(d, 4);

    let search = root.right.as_ref().unwrap();
    let search = search.left.as_ref().unwrap();
    let search = search.left.as_ref().unwrap();
    let d = depth(Some(&root), search);
    assert_eq!(d, 3);
}
