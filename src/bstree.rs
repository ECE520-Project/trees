use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use crate::base::{QueryableTreeNode, QueryableTree};

type RcRefBaseNode<T> = Rc<RefCell<BinarySearchTreeNode<T>>>;
type BaseNodeLink<T> = Option<RcRefBaseNode<T>>;

pub struct BinarySearchTreeNode<T: Ord + Copy + fmt::Debug> {
    pub data: T,
    left: BaseNodeLink<T>,
    right: BaseNodeLink<T>,
}

impl <T: Ord + Copy + fmt::Debug> QueryableTreeNode<T> for BinarySearchTreeNode<T> {
    fn get_left(&self) -> &BaseNodeLink<T> { return &self.left; }
    fn get_right(&self) -> &BaseNodeLink<T> { return &self.right; }
    fn get_data(&self) -> T { return self.data; }
}

impl <T: Ord + Copy + fmt::Debug> BinarySearchTreeNode<T> {
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

pub struct BinarySearchTree<T: Ord + Copy + fmt::Debug> {root: BaseNodeLink<T>}

impl <T: Ord + Copy + fmt::Debug> QueryableTree<T, BinarySearchTreeNode<T>> for BinarySearchTree<T> {
    fn get_root(&self) -> &BaseNodeLink<T> {
        &self.root
    }
}

impl<T: Ord + Copy + fmt::Debug> BinarySearchTree<T> {
    // pub fn new(data: T) -> Self {
    //     Self{
    //         root: Some(Rc::new(RefCell::new(BinarySearchTreeNode{
    //             data,
    //             left: None,
    //             right: None
    //         })))
    //     }
    // }
    pub fn new() -> Self {
        Self{ root: None }
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

    pub fn delete(&self, val: T) {
        unimplemented!()
    }
}
