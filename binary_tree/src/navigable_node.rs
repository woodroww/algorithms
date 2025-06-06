use crate::tree::{Node, NodeRef};
use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::Borrow;
use std::collections::VecDeque;

// https://stackoverflow.com/questions/36167160/how-do-i-express-mutually-recursive-data-structures-in-safe-rust/36168774#36168774

pub struct NavigableNode<T> {
    pub node: NodeRef<T>,
    pub parent: Option<Rc<RefCell<NavigableNode<T>>>>,
}

fn left_nav_node<T>(in_node: Rc<RefCell<NavigableNode<T>>>) -> Option<Rc<RefCell<NavigableNode<T>>>> { 
    let nav_cell: &RefCell<NavigableNode<T>> = in_node.borrow();
    let nav_node = nav_cell.borrow();

    let tree_cell: &RefCell<Node<T>> = nav_node.node.borrow();
    let tree_node = tree_cell.borrow();

    if let Some(left) = &tree_node.left {
        Some(Rc::new(RefCell::new(NavigableNode {
            node: Rc::clone(left),
            parent: Some(Rc::clone(&in_node)),
        })))
    } else {
        None
    }
}

impl <T> NavigableNode<T> {
    /*fn left(&self) -> Option<Rc<RefCell<NavigableNode<T>>>> {

        let cell: &RefCell<Node<T>> = self.node.borrow();
        let node = cell.borrow();


        if let Some(left) = node.left {
            Some(NavigableNode {
                node: left,
                parent: Some(Rc::new(RefCell::new(self))),
            })
        } else {
            None
        }
    }
    fn right(&self) -> Option<Rc<RefCell<NavigableNode<T>>>> {
        let cell: &RefCell<Node<T>> = self.node.borrow();
        let node = cell.borrow();
        if let Some(right) = node.right {
            Some(NavigableNode {
                node: right,
                parent: Some(self),
            })
        } else {
            None
        }
    }*/
    fn parent(&self) -> Option<Rc<RefCell<NavigableNode<T>>>> { 
        match &self.parent {
            Some(p) => {
                Some(Rc::clone(p))
            },
            None => {
                None
            }
        }
    }
}

fn navigate<T>(root: NodeRef<T>) -> NavigableNode<T> {
    NavigableNode { node: root, parent: None }
}


// breadth first
pub struct LevelOrderIteratorWithParents<T> {
    queue: VecDeque<Rc<RefCell<NavigableNode<T>>>>,
}

impl<T> LevelOrderIteratorWithParents<T> {
    pub fn new(root: NodeRef<T>) -> Self {
        let nav_root = navigate(root);
        let mut nav = Self {
            queue: VecDeque::new(),
        };
        nav.queue.push_back(Rc::new(RefCell::new(nav_root)));
        nav
    }
}

impl<T> Iterator for LevelOrderIteratorWithParents<T>
{
    type Item = Rc<RefCell<NavigableNode<T>>>;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.queue.is_empty() {
            let nav_node = self.queue.pop_front().unwrap();
            let nav_cell: &RefCell<NavigableNode<T>> = nav_node.borrow();
            let nav_borrow = nav_cell.borrow();

            let cell: &RefCell<Node<T>> = nav_borrow.node.borrow();
            let node = cell.borrow();

            if let Some(left) = node.left() {
                self.queue.push_back(Rc::new(RefCell::new(NavigableNode {
                    node: left,
                    parent: Some(Rc::clone(&nav_node)),
                })));
            }
            if let Some(right) = node.right() {
                self.queue.push_back(Rc::new(RefCell::new(NavigableNode {
                    node: right,
                    parent: Some(Rc::clone(&nav_node)),
                })));
            }
            Some(Rc::clone(&nav_node))
        } else {
            None
        }
    }
}

