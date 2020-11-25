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
use std::cmp::{Ord, Ordering};

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
        self.rebalance();
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
}

impl<T: Ord + Copy + fmt::Debug> AVLTree<T> {
    /// Create a new AVL Tree
    ///
    /// # Example
    ///
    /// use trees::avltree::AVLTree;
    ///
    /// let mut avl: AVLTree<i64> = AVLTree::new();
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
    /// avl.delete(1);
    /// ```
    pub fn delete(&self, data: T) {
        self.remove(&data);
    }
    
    pub fn remove(&self, data: &T) -> Option<T> {
        let mut prev_ptrs = Vec::<*mut AVLTreeNode<T>>::new();
        let mut current_tree = &self.root;
        let mut target_value = None;

        while let Some(current_node) = current_tree {
            match current_node.borrow().data.cmp(&data) {
                Ordering::Less => {
                    prev_ptrs.push(&mut *current_node.borrow());
                    current_tree = &mut current_node.borrow().right;
                }
                Ordering::Equal => {
                    target_value = Some(&mut *current_node);
                    break;
                }
                Ordering::Greater => {
                    prev_ptrs.push(&mut *current_node.borrow());
                    current_tree = &mut current_node.borrow().left;
                }
            };
        }
        if target_value.is_none() {
            return None;
        }
        let mut target_node = target_value.unwrap();

        let taken_value = if target_node.as_ref().borrow().left.is_none() || target_node.as_ref().borrow().right.is_none() {
            if let Some(left_node) = target_node.as_ref().borrow_mut().left.take() {
                replace(target_node, *left_node).borrow_mut().data
            } else if let Some(right_node) = target_node.as_ref().borrow_mut().right.take() {
                replace(target_node, *right_node).borrow_mut().data
            } else if let Some(prev_ptr) = prev_ptrs.pop() {
                let prev_node = unsafe {&mut *prev_ptr };

                let inner_value = if let Some(left_node) = prev_node.left.as_ref() {
                    if left_node.borrow_mut().data == target_node.borrow_mut().data {
                        prev_node.left.take().unwrap().borrow_mut().data
                    } else {
                        prev_node.right.take().unwrap().borrow_mut().data
                    }
                } else {
                    prev_node.right.take().unwrap().borrow_mut().data
                };

                prev_node.update_height();
                prev_node.rebalance();

                inner_value
            } else {
                self.root.take().unwrap().borrow_mut().data
            }
        } else {
            let right_tree = &mut target_node.borrow_mut().right;

            if right_tree.as_ref().unwrap().borrow_mut().left.is_none() {
                let mut right_node = right_tree.take().unwrap();

                let inner_value = replace(&mut target_node.borrow_mut().data, right_node.borrow_mut().data);
                replace(&mut target_node.borrow_mut().right, right_node.borrow_mut().right.take());

                target_node.borrow_mut().update_height();
                target_node.borrow_mut().rebalance();

                inner_value
            } else {
                let mut next_tree = right_tree;
                let mut inner_ptrs = Vec::<*mut AVLTreeNode<T>>::new();

                while let Some(next_left_node) = next_tree {
                    if next_left_node.borrow_mut().left.is_some() {
                        inner_ptrs.push(&mut *next_left_node.borrow_mut());
                    }
                    next_tree = &mut next_left_node.borrow_mut().left;
                }

                let parent_left_node = unsafe { &mut *inner_ptrs.pop().unwrap() };
                let mut leftmost_node = parent_left_node.left.take().unwrap();

                let inner_value = replace(&mut target_node.borrow_mut().data, leftmost_node.borrow_mut().data);
                replace(&mut parent_left_node.left, leftmost_node.borrow_mut().right.take());

                parent_left_node.update_height();
                parent_left_node.rebalance();

                for node_ptr in inner_ptrs.into_iter().rev() {
                    let node = unsafe { &mut *node_ptr };
                    node.update_height();
                    node.rebalance();
                }

                target_node.borrow_mut().update_height();
                target_node.borrow_mut().rebalance();

                inner_value
            }
        };

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
            node.rebalance();
        }

        Some(taken_value)
    }
}
