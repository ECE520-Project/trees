//! Red-black tree
//!
//! You can generate a red-black tree, and insert or delete nodes.
//!
//! ```
//! use trees::rbtree::RedBlackTree;
//! // use this trait if you want to query information
//! use trees::base::QueryableTree;
//! ```

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use crate::base::{QueryableTreeNode, QueryableTree};

type RcRefRBTNode<T> = Rc<RefCell<RedBlackTreeNode<T>>>;
type RBNodeLink<T> = Option<RcRefRBTNode<T>>;

/// Color representation for the [Node](struct.RedBlackTreeNode.html)
/// of [RedBlackTree](struct.RedBlackTree.html) struct
#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    /// Red color
    Red,
    /// Black color, the root of [RedBlackTree](struct.RedBlackTree.html) will be set to Black
    Black,
}

/// Node struct for [RedBlackTree](struct.RedBlackTree.html) struct
pub struct RedBlackTreeNode<T: Ord + Copy + fmt::Debug> {
    /// Data stored in the node
    pub data: T,
    /// The color of the node
    pub color: NodeColor,
    parent: RBNodeLink<T>,
    left: RBNodeLink<T>,
    right: RBNodeLink<T>,
}

/// An implementation of [Red-black Tree](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree)
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
    /// Create a new Red-black Tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::rbtree::RedBlackTree;
    ///
    /// let mut rbt = RedBlackTree::new();
    /// ```
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

    /// Insert a new value to the tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::rbtree::RedBlackTree;
    ///
    /// let mut rbt = RedBlackTree::new();
    /// rbt.insert(1);
    /// ```
    pub fn insert(&self, val: T) {
        unimplemented!()
    }

    /// Delete a value from the tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::rbtree::RedBlackTree;
    ///
    /// let mut rbt = RedBlackTree::new();
    /// rbt.delete(1);
    /// ```
    pub fn delete(&self, val: T) {
        unimplemented!()
    }
}
