use crate::tree::Node;

// ---------------------------------------------------------------------------------------
// tree creation for testing
// ---------------------------------------------------------------------------------------

pub fn make_num_tree_1() -> Node<i32> {
    //       3
    //    /    \
    //   11     4
    //  /  \     \
    // 4   -2     1

    let n_4 = Node {
        value: 4,
        left: None,
        right: None,
    };
    let n_neg2 = Node {
        value: -2,
        left: None,
        right: None,
    };
    let n_1 = Node {
        value: 1,
        left: None,
        right: None,
    };
    let n_4_2 = Node {
        value: 4,
        left: None,
        right: Some(Box::new(n_1)),
    };
    let n_11 = Node {
        value: 11,
        left: Some(Box::new(n_4)),
        right: Some(Box::new(n_neg2)),
    };
    Node {
        value: 3,
        left: Some(Box::new(n_11)),
        right: Some(Box::new(n_4_2)),
    }
}

pub fn make_num_tree_2() -> Node<i32> {
    //       5
    //    /    \
    //   11     3
    //  / \      \
    // 4   14     12

    let n_4 = Node {
        value: 4,
        left: None,
        right: None,
    };
    let n_14 = Node {
        value: 14,
        left: None,
        right: None,
    };
    let n_12 = Node {
        value: 12,
        left: None,
        right: None,
    };
    let n_11 = Node {
        value: 11,
        left: Some(Box::new(n_4)),
        right: Some(Box::new(n_14)),
    };
    let n_3 = Node {
        value: 3,
        left: None,
        right: Some(Box::new(n_12)),
    };
    Node {
        value: 5,
        left: Some(Box::new(n_11)),
        right: Some(Box::new(n_3)),
    }
}

pub fn make_num_tree_3() -> Node<i32> {
    //        -1
    //      /   \
    //    -6    -5
    //   /  \     \
    // -3   -4   -13
    //     /       \
    //    -2       -2

    let n_neg2_l = Node {
        value: -2,
        left: None,
        right: None,
    };
    let n_neg2_r = Node {
        value: -2,
        left: None,
        right: None,
    };

    let n_neg3 = Node {
        value: -3,
        left: None,
        right: None,
    };
    let n_neg4 = Node {
        value: -4,
        left: Some(Box::new(n_neg2_l)),
        right: None,
    };
    let n_neg13 = Node {
        value: -13,
        left: None,
        right: Some(Box::new(n_neg2_r)),
    };

    let n_neg6 = Node {
        value: -6,
        left: Some(Box::new(n_neg3)),
        right: Some(Box::new(n_neg4)),
    };
    let n_neg5 = Node {
        value: -6,
        left: None,
        right: Some(Box::new(n_neg13)),
    };
    Node {
        value: -1,
        left: Some(Box::new(n_neg6)),
        right: Some(Box::new(n_neg5)),
    }
}

pub fn make_num_tree_4() -> Node<i32> {
    //      1
    //    /   \
    //   6     0
    //  / \     \
    // 3   -6    2
    //    /       \
    //   2         2

    let n_2_r_2 = Node {
        value: 2,
        left: None,
        right: None,
    };
    let n_2_r_1 = Node {
        value: 2,
        left: None,
        right: Some(Box::new(n_2_r_2)),
    };
    let n_0 = Node {
        value: 0,
        left: None,
        right: Some(Box::new(n_2_r_1)),
    };
    let n_3 = Node {
        value: 3,
        left: None,
        right: None,
    };
    let n_2_l = Node {
        value: 2,
        left: None,
        right: None,
    };
    let n_neg6 = Node {
        value: -6,
        left: Some(Box::new(n_2_l)),
        right: None,
    };
    let n_6 = Node {
        value: 6,
        left: Some(Box::new(n_3)),
        right: Some(Box::new(n_neg6)),
    };
    Node {
        value: 1,
        left: Some(Box::new(n_6)),
        right: Some(Box::new(n_0)),
    }
}

fn make_num_tree_5() -> Node<i32> {
    //          5
    //        /   \
    //      11     3
    //     /  \     \
    //    4    2     1

    let node_4 = Node {
        value: 4,
        left: None,
        right: None,
    };
    let node_2 = Node {
        value: 2,
        left: None,
        right: None,
    };
    let node_1 = Node {
        value: 1,
        left: None,
        right: None,
    };

    let node_11 = Node {
        value: 11,
        left: Some(Box::new(node_4)),
        right: Some(Box::new(node_2)),
    };
    let node_3 = Node {
        value: 3,
        left: None,
        right: Some(Box::new(node_1)),
    };

    let root = Node {
        value: 5,
        left: Some(Box::new(node_11)),
        right: Some(Box::new(node_3)),
    };
    root
}

pub fn make_num_tree_6() -> Node<i32> {
    //        5
    //     /    \
    //    11    54
    //  /   \
    // 20   15
    //      / \
    //     1  3

    let n_1 = Node {
        value: 1,
        left: None,
        right: None,
    };
    let n_3 = Node {
        value: 3,
        left: None,
        right: None,
    };
    let n_15 = Node {
        value: 15,
        left: Some(Box::new(n_1)),
        right: Some(Box::new(n_3)),
    };

    let n_20 = Node {
        value: 20,
        left: None,
        right: None,
    };
    let n_11 = Node {
        value: 11,
        left: Some(Box::new(n_20)),
        right: Some(Box::new(n_15)),
    };
    let n_54 = Node {
        value: 54,
        left: None,
        right: None,
    };
    Node {
        value: 5,
        left: Some(Box::new(n_11)),
        right: Some(Box::new(n_54)),
    }
}

pub fn make_num_tree_7() -> Node<i32> {
    //        -1
    //      /   \
    //    -6    -5
    //   /  \     \
    // -3   0    -13
    //     /       \
    //    -1       -2

    let n_neg1 = Node {
        value: -1,
        left: None,
        right: None,
    };
    let n_neg2 = Node {
        value: -2,
        left: None,
        right: None,
    };

    let n_neg3 = Node {
        value: -3,
        left: None,
        right: None,
    };
    let n_0 = Node {
        value: 0,
        left: Some(Box::new(n_neg1)),
        right: None,
    };
    let n_neg13 = Node {
        value: -13,
        left: None,
        right: Some(Box::new(n_neg2)),
    };

    let n_neg6 = Node {
        value: -6,
        left: Some(Box::new(n_neg3)),
        right: Some(Box::new(n_0)),
    };
    let n_neg5 = Node {
        value: -5,
        left: None,
        right: Some(Box::new(n_neg13)),
    };
    Node {
        value: -1,
        left: Some(Box::new(n_neg6)),
        right: Some(Box::new(n_neg5)),
    }
}

pub fn make_num_tree_8() -> Node<i32> {
    //          5
    //        /   \
    //      11     3
    //     /  \
    //    4    2

    let node_4 = Node {
        value: 4,
        left: None,
        right: None,
    };
    let node_2 = Node {
        value: 2,
        left: None,
        right: None,
    };

    let node_11 = Node {
        value: 11,
        left: Some(Box::new(node_4)),
        right: Some(Box::new(node_2)),
    };
    let node_3 = Node {
        value: 3,
        left: None,
        right: None,
    };

    let root = Node {
        value: 5,
        left: Some(Box::new(node_11)),
        right: Some(Box::new(node_3)),
    };
    root
}

pub fn make_num_tree_9() -> Node<i32> {
    //          1
    //    2           3
    //  4   5       6
    //     7      8

    let node_7 = Node {
        value: 7,
        left: None,
        right: None,
    };
    let node_4 = Node {
        value: 4,
        left: None,
        right: None,
    };
    let node_5 = Node {
        value: 5,
        left: Some(Box::new(node_7)),
        right: None,
    };
    let node_2 = Node {
        value: 2,
        left: Some(Box::new(node_4)),
        right: Some(Box::new(node_5)),
    };

    let node_8 = Node {
        value: 8,
        left: None,
        right: None,
    };
    let node_6 = Node {
        value: 6,
        left: Some(Box::new(node_8)),
        right: None,
    };
    let node_3 = Node {
        value: 3,
        left: Some(Box::new(node_6)),
        right: None,
    };
    Node {
        value: 1,
        left: Some(Box::new(node_2)),
        right: Some(Box::new(node_3)),
    }
}

pub fn make_char_tree_1() -> Node<char> {
    //      a
    //    /   \
    //   b     c
    //  / \     \
    // d   e     f

    let f = Node {
        value: 'f',
        left: None,
        right: None,
    };
    let e = Node {
        value: 'e',
        left: None,
        right: None,
    };
    let d = Node {
        value: 'd',
        left: None,
        right: None,
    };

    let c = Node {
        value: 'c',
        left: None,
        right: Some(Box::new(f)),
    };
    let b = Node {
        value: 'b',
        left: Some(Box::new(d)),
        right: Some(Box::new(e)),
    };

    let a = Node {
        value: 'a',
        left: Some(Box::new(b)),
        right: Some(Box::new(c)),
    };
    a
}

pub fn make_char_tree_2() -> Node<char> {
    //      a
    //    /   \
    //   b     c
    //  / \     \
    // d   e     f
    //    /       \
    //   g         h

    let g = Node {
        value: 'g',
        left: None,
        right: None,
    };
    let h = Node {
        value: 'h',
        left: None,
        right: None,
    };
    let f = Node {
        value: 'f',
        left: None,
        right: Some(Box::new(h)),
    };
    let e = Node {
        value: 'e',
        left: Some(Box::new(g)),
        right: None,
    };
    let d = Node {
        value: 'd',
        left: None,
        right: None,
    };

    let c = Node {
        value: 'c',
        left: None,
        right: Some(Box::new(f)),
    };
    let b = Node {
        value: 'b',
        left: Some(Box::new(d)),
        right: Some(Box::new(e)),
    };

    let a = Node {
        value: 'a',
        left: Some(Box::new(b)),
        right: Some(Box::new(c)),
    };
    a
}

pub fn make_char_tree_3() -> Node<char> {
    //      a
    //       \
    //        b
    //       /
    //      c
    //    /  \
    //   x    d
    //         \
    //          e

    let e = Node {
        value: 'e',
        left: None,
        right: None,
    };
    let d = Node {
        value: 'd',
        left: None,
        right: Some(Box::new(e)),
    };
    let x = Node {
        value: 'x',
        left: None,
        right: None,
    };
    let c = Node {
        value: 'c',
        left: Some(Box::new(x)),
        right: Some(Box::new(d)),
    };
    let b = Node {
        value: 'b',
        left: Some(Box::new(c)),
        right: None,
    };
    let a = Node {
        value: 'a',
        left: None,
        right: Some(Box::new(b)),
    };
    a
}
