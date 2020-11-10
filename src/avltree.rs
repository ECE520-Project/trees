use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use crate::base::{QueryableTreeNode, QueryableTree};

type RcRefAVLTNode<T> = Rc<RefCell<AVLTreeNode<T>>>;
type AVLNodeLink<T> = Option<RcRefAVLTNode<T>>;

pub struct AVLTreeNode<T: Ord + Copy + fmt::Debug> {
    pub data: T,
    parent: AVLNodeLink<T>,
    left: AVLNodeLink<T>,
    right: AVLNodeLink<T>,
    height: usize,
}

pub struct AVLTree<T: Ord + Copy + fmt::Debug> {root: AVLNodeLink<T>}

impl <T: Ord + Copy + fmt::Debug> QueryableTreeNode<T> for AVLTreeNode<T> {
    fn get_left(&self) -> &AVLNodeLink<T> { return &self.left; }
    fn get_right(&self) -> &AVLNodeLink<T> { return &self.right; }
    fn get_data(&self) -> T { return self.data; }
}

impl <T: Ord + Copy + fmt::Debug> QueryableTree<T, AVLTreeNode<T>> for AVLTree<T> {
    fn get_root(&self) -> &AVLNodeLink<T> {
        &self.root
    }
}

impl<T: Ord + Copy + fmt::Debug> AVLTree<T> {
    // pub fn new() -> Self {
    //     Self { root: None }
    // }
    pub fn new(data: T) -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(AVLTreeNode {
                data,
                parent: None,
                left: None,
                right: None,
                height: 0
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
