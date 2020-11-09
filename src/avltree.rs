use std::cell::RefCell;
use std::rc::Rc;

use crate::base::{QueryableTreeNode, QueryableTree};

type RcRefAVLTNode<T> = Rc<RefCell<AVLTreeNode<T>>>;
type AVLNodeLink<T> = Option<RcRefAVLTNode<T>>;

pub struct AVLTreeNode<T> {
    pub key: T,
    parent: AVLNodeLink<T>,
    left: AVLNodeLink<T>,
    right: AVLNodeLink<T>,
    height: usize,
}

pub struct AVLTree<T: Ord> {root: AVLNodeLink<T>}

impl <T: Ord> QueryableTreeNode for AVLTreeNode<T> {
    fn get_left(&self) -> &AVLNodeLink<T> { return &self.left; }
    fn get_right(&self) -> &AVLNodeLink<T> { return &self.right; }
}

impl <T: Ord> QueryableTree<AVLTreeNode<T>> for AVLTree<T> {
    fn get_root(&self) -> &AVLNodeLink<T> {
        &self.root
    }
}

impl<T: Ord> AVLTree<T> {
    pub fn new(data: T) -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(AVLTreeNode {
                key: data,
                parent: None,
                left: None,
                right: None,
                height: 0
            })))
        }
    }
}