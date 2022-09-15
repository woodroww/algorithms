use std::collections::HashSet;

use rusty_graph::{GraphNode, Graph};

fn main() {

    let mut g = Graph::new();
    g.add_node(GraphNode::new(4));
    g.add_node(GraphNode::new(5));
    g.add_node(GraphNode::new(6));
    //g.add_edge(u, v)
    g.print();


    let _my_friends = vec![
        ("Sreeni", "Mike"),
        ("Sreeni", "Imran"),
        ("Sreeni", "Mustafa"),
        ("Sreeni", "Syed"),
        ("Sreeni", "Satya"),
        ("Sreeni", "Vamsi"),
        ("Imran", "Mustafa"),
        ("Imran", "Mike"),
        ("Mustafa", "Satya"),
        ("Mustafa", "Vamsi"),
        ("Syed", "Mike"),
        ("Syed", "Satya"),
    ];
    //                           0        1       2         3         4        5      6
    let unique_friends = vec!["Sreeni", "Mike", "Imran", "Mustafa", "Syed", "Satya", "Vamsi"];

    let mut g = Graph::new();
    let uuid = g.add_nodes(unique_friends);
    g.add_edge(uuid[0], uuid[1]);
    g.add_edge(uuid[0], uuid[2]);
    g.add_edge(uuid[0], uuid[3]);
    g.add_edge(uuid[0], uuid[4]);
    g.add_edge(uuid[0], uuid[5]);
    g.add_edge(uuid[0], uuid[6]);

    g.add_edge(uuid[2], uuid[3]);
    g.add_edge(uuid[2], uuid[1]);

    g.add_edge(uuid[3], uuid[5]);
    g.add_edge(uuid[3], uuid[6]);

    g.add_edge(uuid[4], uuid[1]);
    g.add_edge(uuid[4], uuid[5]);

    g.print();
}
