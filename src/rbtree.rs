use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use crate::base::{QueryableTreeNode, QueryableTree};

type RcRefRBTNode<T> = Rc<RefCell<RedBlackTreeNode<T>>>;
type RBNodeLink<T> = Option<RcRefRBTNode<T>>;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    Red,
    Black,
}

pub struct RedBlackTreeNode<T: Ord + Copy + fmt::Debug> {
    pub data: T,
    pub color: NodeColor,
    parent: RBNodeLink<T>,
    left: RBNodeLink<T>,
    right: RBNodeLink<T>,
}

pub struct RedBlackTree<T: Ord + Copy + fmt::Debug> {root: RBNodeLink<T>}

impl <T: Ord + Copy + fmt::Debug> QueryableTreeNode<T> for RedBlackTreeNode<T> {
    fn get_left(&self) -> &RBNodeLink<T> { return &self.left; }
    fn get_right(&self) -> &RBNodeLink<T> { return &self.right; }
    fn get_data(&self) -> T { return self.data; }
}

impl <T: Ord + Copy + fmt::Debug> QueryableTree<T, RedBlackTreeNode<T>> for RedBlackTree<T> {
    fn get_root(&self) -> &RBNodeLink<T> {
        &self.root
    }
}

impl<T: Ord + Copy + fmt::Debug> RedBlackTree<T> {
    // pub fn new() -> Self {
    //     Self { root: None }
    // }
    pub fn new(data: T) -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(RedBlackTreeNode {
                data,
                color: NodeColor::Red,
                parent: None,
                left: None,
                right: None
            })))
        }
    }

    pub fn insert(&self, val: T) {
        unimplemented!()
    }

    pub fn delete(&self, val: T) {
        unimplemented!()
    }
}
