use std::cell::RefCell;
use std::rc::Rc;

use crate::base::{QueryableTreeNode, QueryableTree};

type RcRefBaseNode<T> = Rc<RefCell<BinarySearchTreeNode<T>>>;
type BaseNodeLink<T> = Option<RcRefBaseNode<T>>;

pub struct BinarySearchTreeNode<T: Ord> {
    pub data: T,
    left: BaseNodeLink<T>,
    right: BaseNodeLink<T>,
}

impl <T: Ord> QueryableTreeNode for BinarySearchTreeNode<T> {
    fn get_left(&self) -> &BaseNodeLink<T> { return &self.left; }
    fn get_right(&self) -> &BaseNodeLink<T> { return &self.right; }
}

impl <T: Ord> BinarySearchTreeNode<T> {
    fn new(data: T) -> BaseNodeLink<T> {
        Some(Rc::new(RefCell::new(Self{
            data,
            left: None,
            right: None
        })))
    }

    fn insert(&mut self, new_value: T) {
        if self.data == new_value {
            return
        }
        let new_node =
            if new_value < self.data {&mut self.left}
            else {&mut self.right};
        match new_node {
            Some(node) => node.borrow_mut().insert(new_value),
            None => {
                *new_node = Self::new(new_value);
            }
        }
    }
}

pub struct BinarySearchTree<T: Ord> {root: BaseNodeLink<T>}

impl <T: Ord> QueryableTree<BinarySearchTreeNode<T>> for BinarySearchTree<T> {
    fn get_root(&self) -> &BaseNodeLink<T> {
        &self.root
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new(data: T) -> Self {
        Self{
            root: Some(Rc::new(RefCell::new(BinarySearchTreeNode{
                data,
                left: None,
                right: None
            })))
        }
    }

    pub fn insert(&mut self, new_val: T) {
        if self.root.is_none() {
            self.root = Some(Rc::new(RefCell::new(BinarySearchTreeNode{
                data: new_val,
                left: None,
                right: None
            })));
        } else {
            self.root.as_ref().unwrap().borrow_mut().insert(new_val);
        }
    }
}