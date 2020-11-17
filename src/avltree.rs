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
use std::cmp::{Ord, Ordering};

impl<T: Ord + Copy + fmt::Debug> AVLTreeNode<T> {
    fn new(data:T) -> AVLTreeNode<T>{
        AVLTreeNode {
            data,
            parent: None,
            left: None,
            right: None,
            height: 0,
        }
    }
    //new insert......................................
    fn insert(&mut self, node: AVLTreeNode<T>) {
        match (node).data.cmp(&self.data) {
            Ordering::Less => {
                match self.left {
                    None => self.left = Some(Rc::new(RefCell::new(node))),
                    Some(ref mut l) => l.borrow_mut().insert(node),
                }
            },
            Ordering::Greater => {
                match self.right {
                    None => self.right = Some(Rc::new(RefCell::new(node))),
                    Some(ref mut r) => r.borrow_mut().insert(node),
                }
            },
            _ => {},
        }
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

    pub fn insert(&mut self, val: T){
        match self.root {
            None => self.root = Some(Rc::new(RefCell::new(AVLTreeNode::new(val)))),
            Some(ref mut r) => r.borrow_mut().insert(AVLTreeNode::new(val)),
        }
    }

    pub fn delete(&self, val: T) {
        unimplemented!()
    }
}
