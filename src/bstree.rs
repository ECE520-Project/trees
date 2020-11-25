//! Binary search tree
//!
//! You can generate a binary search tree, and insert or delete nodes.
//!
//! ```
//! use trees::bstree::BinarySearchTree;
//! // use this trait if you want to query information
//! use trees::base::QueryableTree;
//! ```

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use crate::base::{QueryableTreeNode, QueryableTree};

type RcRefBaseNode<T> = Rc<RefCell<BinarySearchTreeNode<T>>>;
type BaseNodeLink<T> = Option<RcRefBaseNode<T>>;

/// Node struct for [BinarySearchTree](struct.BinarySearchTree.html) struct
pub struct BinarySearchTreeNode<T: Ord + Copy + fmt::Debug> {
    /// Data stored in the node
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
    /// Create a new node, which will be called by [BinarySearchTree](struct.BinarySearchTree.html)
    fn new(data: T) -> BaseNodeLink<T> {
        Some(Rc::new(RefCell::new(Self{
            data,
            left: None,
            right: None
        })))
    }
    /// Insert a node 
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
    /// Delete a node 
    fn delete(current: &mut BaseNodeLink<T>, data: T) {
        if let Some(node) = current {
            if data == node.borrow_mut().data{
                Self::delete_node(current);
            } else if data < node.borrow_mut().data {
                Self::delete(&mut node.borrow_mut().left, data)
            } else {
                Self::delete(&mut node.borrow_mut().right, data)
            }
        }
    }

    fn delete_node(current: &mut BaseNodeLink<T>) {
        if let Some(node) = current {
            if node.borrow_mut().right.is_some() {
                let mut sptr = &mut node.borrow_mut().right as *mut BaseNodeLink<T>;
                loop {
                    let successor = unsafe { &mut *sptr };
                    let snode = successor.as_mut().unwrap();
                    if snode.borrow_mut().left.is_none() {
                        std::mem::swap(&mut node.borrow_mut().data, &mut snode.borrow_mut().data);
                        Self::delete_node(successor);
                        break;
                    }
                    sptr = &mut snode.borrow_mut().left;
                }
            } else if node.borrow_mut().left.is_some() {
                let mut pptr = &mut node.borrow_mut().left as *mut BaseNodeLink<T>;
                loop {
                    let predecessor = unsafe { &mut *pptr };
                    let pnode = predecessor.as_mut().unwrap();
                    if pnode.borrow_mut().right.is_none() {
                        std::mem::swap(&mut node.borrow_mut().data, &mut pnode.borrow_mut().data);
                        Self::delete_node(predecessor);
                        break;
                    }
                    pptr = &mut pnode.borrow_mut().right;
                }
            } else {
                *current = None;
            }
        }
    }
}

/// An implementation of [Binary Search Tree](https://en.wikipedia.org/wiki/Binary_search_tree)
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
    /// Create a new Binary Search Tree
    ///
    /// # Example
    ///
    /// use trees::bstree::BinarySearchTree;
    ///
    /// let mut bst = BinarySearchTree::new();
    /// ```
    pub fn new() -> Self {
        Self{ root: None }
    }


    /// Insert a new value to the tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::bstree::BinarySearchTree;
    ///
    /// let mut bst = BinarySearchTree::new();
    /// bst.insert(1);
    /// ```
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
    /// Delete a value from the tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::bstree::BinarySearchTree;
    ///
    /// let mut bst = BinarySearchTree::new();
    /// bst.delete(1);
    /// ```
    pub fn delete(&mut self, val: T) {
        BinarySearchTreeNode::delete(&mut self.root, val)
    }
}
