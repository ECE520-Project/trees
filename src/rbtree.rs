use std::cell::RefCell;
use std::rc::Rc;

use crate::base::{QueryableTreeNode, QueryableTree};

type RcRefRBTNode<T> = Rc<RefCell<RedBlackTreeNode<T>>>;
type RBNodeLink<T> = Option<RcRefRBTNode<T>>;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    Red,
    Black,
}

pub struct RedBlackTreeNode<T> {
    pub key: T,
    pub color: NodeColor,
    parent: RBNodeLink<T>,
    left: RBNodeLink<T>,
    right: RBNodeLink<T>,
}

pub struct RedBlackTree<T: Ord> {root: RBNodeLink<T>}

impl <T: Ord> QueryableTreeNode for RedBlackTreeNode<T> {
    fn get_left(&self) -> &RBNodeLink<T> { return &self.left; }
    fn get_right(&self) -> &RBNodeLink<T> { return &self.right; }
}

impl <T: Ord> QueryableTree<RedBlackTreeNode<T>> for RedBlackTree<T> {
    fn get_root(&self) -> &RBNodeLink<T> {
        &self.root
    }
}

impl<T: Ord> RedBlackTree<T> {
    pub fn new(data: T) -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(RedBlackTreeNode {
                key: data,
                color: NodeColor::Red,
                parent: None,
                left: None,
                right: None
            })))
        }
    }
}