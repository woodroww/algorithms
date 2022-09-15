use core::cell::RefCell;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use core::hash::Hash;
use uuid::Uuid;

type NodeRef<T> = Rc<RefCell<GraphNode<T>>>;

pub struct GraphNode<T> {
    value: T,
    neighbors: Vec<Uuid>,
    id: Uuid,
}

impl<T> GraphNode<T> {

    pub fn new(value: T) -> Self {
        Self {
            value, 
            neighbors: Vec::new(),
            id: Uuid::new_v4(),
        }
    }

    pub fn add_neighbor(&mut self, v: Uuid) {
        if !self.neighbors.contains(&v) {
            self.neighbors.push(v);
        }
    }
}

pub struct Graph<T> {
    nodes: HashMap<Uuid, NodeRef<T>>,
}

impl<T> Graph<T>
where T: Hash + std::cmp::Eq + Clone + std::fmt::Display
{
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: GraphNode<T>) -> bool {
        if !self.nodes.contains_key(&node.id) {
            self.nodes.insert(node.id, Rc::new(RefCell::new(node)));
            true
        } else {
            false
        }
    }

    pub fn add_nodes(&mut self, values: Vec<T>) -> Vec<Uuid> {
        let mut uuids = Vec::new();
        for v in values {
            let node = GraphNode::new(v);
            uuids.push(node.id);
            self.add_node(node);
        }
        uuids
    }

    pub fn add_edge(&mut self, u: Uuid, v: Uuid) {
        self.nodes.entry(u).and_modify(|n| {
            n.borrow_mut().add_neighbor(v);
        });
        self.nodes.entry(v).and_modify(|n| {
            n.borrow_mut().add_neighbor(u);
        });
    }

    pub fn print(&self) {
        for (key, value) in &self.nodes {
            let node: &GraphNode<T> = &value.as_ref().borrow();
            print!("({})", node.value);
            print!(" [");
            let mut first = true;
            for n in &node.neighbors {
                if first {
                    first = false;
                } else {
                    print!(" ");
                }
                let neighbor = self.nodes.get(n).unwrap();
                let n_node = neighbor.as_ref().borrow();
                print!("{}", n_node.value);
            }
            println!("]");
        }
    }
}



