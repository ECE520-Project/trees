//! AVL tree
//!
//! You can generate an AVL tree, and insert or delete nodes.
//!
//! ```
//! use trees::avltree::AVLTree;
//! // use this trait if you want to query information
//! use trees::base::QueryableTree;
//! ```

use core::cmp::max;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use std::mem::{replace, swap};

use crate::base::{QueryableTreeNode, QueryableTree};

type RcRefAVLTNode<T> = Rc<RefCell<AVLTreeNode<T>>>;
type AVLNodeLink<T> = Option<RcRefAVLTNode<T>>;

/// Node struct for [AVLTree](struct.AVLTree.html) struct
pub struct AVLTreeNode<T: Ord + Copy + fmt::Debug> {
    /// Data stored in the node
    pub data: T,
    parent: AVLNodeLink<T>,
    left: AVLNodeLink<T>,
    right: AVLNodeLink<T>,
    height: usize,
}

/// An implementation of [AVL Tree](https://en.wikipedia.org/wiki/AVL_tree)
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
    /// Create a new node
    fn new(data:T) -> AVLTreeNode<T>{
        AVLTreeNode {
            data,
            parent: None,
            left: None,
            right: None,
            height: 0,
        }
    }

    /// Insert a new node
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

    /// Get height of left child
    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.borrow().height())
    }

    /// Get height of right child
    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.borrow().height())
    }

    /// Update height for current node based on children's height
    fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height());
    }

    /// Calculate balance factor for current node
    fn balance_factor(&self) -> i8 {
        let left_height = self.get_left().as_ref().map(
            |l| l.borrow().height()
        ).unwrap_or(0);
        let right_height = self.get_right().as_ref().map(
            |r| r.borrow().height()
        ).unwrap_or(0);

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    /// Rotate left to balance the tree
    fn rotate_left(&mut self) -> bool {
        if self.right.is_none() {
            return false;
        }
        let right_node = self.right.as_mut().unwrap();
        let right_left_tree = right_node.borrow_mut().left.take();
        let right_right_tree = right_node.borrow_mut().right.take();

        let mut new_left_tree = replace(&mut self.right, right_right_tree);
        swap(&mut self.data, &mut new_left_tree.as_mut().unwrap().borrow_mut().data);
        let left_tree = self.left.take();

        let new_left_node = new_left_tree.as_mut().unwrap();
        new_left_node.borrow_mut().right = right_left_tree;
        new_left_node.borrow_mut().left = left_tree;
        self.left = new_left_tree;

        if let Some(node) = self.left.as_mut() {
            node.borrow_mut().update_height();
        }
        self.update_height();
        true
    }

    /// Rotate right to balance the tree
    pub fn rotate_right(&mut self) -> bool {
        if self.left.is_none() {
            return false;
        }
        let left_node = self.left.as_mut().unwrap();
        let left_right_tree = left_node.borrow_mut().right.take();
        let left_left_tree = left_node.borrow_mut().left.take();

        let mut new_right_tree = replace(&mut self.left, left_left_tree);
        swap(&mut self.data, &mut new_right_tree.as_mut().unwrap().borrow_mut().data);
        let right_tree = self.right.take();

        let new_right_node = new_right_tree.as_mut().unwrap();
        new_right_node.borrow_mut().left = left_right_tree;
        new_right_node.borrow_mut().right = right_tree;
        self.right = new_right_tree;

        if let Some(node) = self.right.as_mut() {
            node.borrow_mut().update_height();
        }
        self.update_height();
        true
    }

    /// Rebalance after deletion
    fn rebalance(&mut self) -> bool {
        match self.balance_factor() {
            -2 => {
                let right_node = self.right.as_mut().unwrap();

                if right_node.borrow_mut().balance_factor() == 1 {
                    right_node.borrow_mut().rotate_right();
                }
                self.rotate_left();
                true
            }
            2 => {
                let left_node = self.left.as_mut().unwrap();
                if left_node.borrow_mut().balance_factor() == -1 {
                    left_node.borrow_mut().rotate_left();
                }
                self.rotate_right();
                true
            }
            _ => false,
        }
    }

    /// Delete node then use the balance function
    fn delete(&mut self, node: AVLTreeNode<T>){
        unimplemented!()
    }
}

impl<T: Ord + Copy + fmt::Debug> AVLTree<T> {
    /// Create a new AVL Tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::avltree::AVLTree;
    ///
    /// let mut avl = AVLTree::new();
    /// ```
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Insert a new value to the tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::avltree::AVLTree;
    ///
    /// let mut avl = AVLTree::new();
    /// avl.insert(1);
    /// ```
    pub fn insert(&mut self, val: T){
        match self.root {
            None => self.root = Some(Rc::new(RefCell::new(AVLTreeNode::new(val)))),
            Some(ref mut r) => r.borrow_mut().insert(AVLTreeNode::new(val)),
        }
    }

    /// Delete a value from the tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::avltree::AVLTree;
    ///
    /// let mut avl = AVLTree::new();
    /// avl.delete(1);
    /// ```
    pub fn delete(&self, val: T) {
        unimplemented!()
    }
}
