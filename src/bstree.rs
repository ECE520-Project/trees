//! Binary search tree
//!
//! You can generate a binary search tree, and insert or delete nodes.
//!
//! ```
//! use trees::bstree::BinarySearchTree;
//! // use this trait if you want to query information
//! use trees::base::QueryableTree;
//! ```

use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::fmt;
use std::cmp::{Ord, Ordering};

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
    /// Create an new node, which will be called by [BinarySearchTree](struct.BinarySearchTree.html)
    fn new(data: T) -> BaseNodeLink<T> {
        Some(Rc::new(RefCell::new(Self{
            data,
            left: None,
            right: None
        })))
    }

    /// Insert a node, which will be called by [BinarySearchTree](struct.BinarySearchTree.html)
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

// <<<<<<< tests
    fn _delete_node_have_two_children(left: &RcRefBaseNode<T>) {
        let right_min = left.borrow().right.as_ref().unwrap().borrow().min();
        left.borrow_mut().delete(right_min);
        left.borrow_mut().data = right_min;
    }

    fn _delete_right(&mut self, val: T) {
        if let Some(right) = self.right.as_ref() {
            if right.borrow().data == val {
                if right.borrow().left.is_none() && right.borrow().right.is_none() {
                    self.right = None;
                } else if right.borrow().left.is_none() && !right.borrow().right.is_none() {
                    self.right.take().map(|node| {
                        self.right = node.borrow().right.clone()
                    });
                } else if !right.borrow().left.is_none() && right.borrow().right.is_none() {
                    self.right.take().map(|node| {
                        self.right = node.borrow().left.clone()
                    });
                } else {
                    Self::_delete_node_have_two_children(right);
                }
            } else {
                right.borrow_mut().delete(val);
            }
        }
    }
// =======

    /// Delete a node 
    // fn delete(current: &mut BaseNodeLink<T>, data: T) {
    //     if let Some(node) = current {
    //         if data == node.as_ref().borrow_mut().data{
    //             Self::delete_node(current);
    //         } else if data < node.as_ref().borrow().data {
    //             Self::delete(&mut node.as_ref().borrow_mut().left, data)
    //         } else {
    //             Self::delete(&mut node.as_ref().borrow_mut().right, data)
    //         }
    //     }
    // }

// <<<<<<< tests
    fn _delete_left(&mut self, val: T) {
        if let Some(left) = self.left.as_ref() {
            if left.borrow().data == val {
                if left.borrow().left.is_none() && left.borrow().right.is_none() {
                    self.left = None;
                } else if left.borrow().left.is_none() && !left.borrow().right.is_none() {
                    self.left.take().map(|node| {
                        self.left = node.borrow().right.clone()
                    });
                } else if !left.borrow().left.is_none() && left.borrow().right.is_none() {
                    self.left.take().map(|node| {
                        self.left = node.borrow().left.clone()
                    });
                } else {
                    Self::_delete_node_have_two_children(left);
                }
            } else {
                left.borrow_mut().delete(val);
            }
        }
    }

//     /// Delete a node, which will be called by [BinarySearchTree](struct.BinarySearchTree.html)
    fn delete(&mut self, val: T) {
        match self.data.cmp(&val) {
            Ordering::Greater => self._delete_left(val),
            Ordering::Less => self._delete_right(val),
            _ => panic!("impossible!"),
        }
    }
// =======
    // fn delete_node(current: &mut BaseNodeLink<T>) {
    //     if let Some(node) = current {
    //         if node.borrow_mut().right.is_some() {
    //             let mut sptr = &mut node.borrow_mut().right as *mut BaseNodeLink<T>;
    //             loop {
    //                 let successor = unsafe { &mut *sptr };
    //                 let snode = successor.as_mut().unwrap();
    //                 if snode.borrow_mut().left.is_none() {
    //                     std::mem::swap(&mut node.borrow_mut().data, &mut snode.borrow_mut().data);
    //                     Self::delete_node(successor);
    //                     break;
    //                 }
    //                 sptr = &mut snode.borrow_mut().left;
    //             }
    //         } else if node.borrow_mut().left.is_some() {
    //             let mut pptr = &mut node.borrow_mut().left as *mut BaseNodeLink<T>;
    //             loop {
    //                 let predecessor = unsafe { &mut *pptr };
    //                 let pnode = predecessor.as_mut().unwrap();
    //                 if pnode.borrow_mut().right.is_none() {
    //                     std::mem::swap(&mut node.borrow_mut().data, &mut pnode.borrow_mut().data);
    //                     Self::delete_node(predecessor);
    //                     break;
    //                 }
    //                 pptr = &mut pnode.borrow_mut().right;
    //             }
    //         } else {
    //             *current = None;
    //         }
    //     }
    // }
// >>>>>>> main
}

/// An implementation of [Binary Search Tree](https://en.wikipedia.org/wiki/Binary_search_tree)
pub struct BinarySearchTree<T: Ord + Copy + fmt::Debug> {root: BaseNodeLink<T>}

impl <T: Ord + Copy + fmt::Debug> QueryableTree<T, BinarySearchTreeNode<T>> for BinarySearchTree<T> {
    fn get_root(&self) -> &BaseNodeLink<T> {
        &self.root
    }
}

impl<T: Ord + Copy + fmt::Debug> BinarySearchTree<T> {
    /// Create a new Binary Search Tree
    ///
    /// # Example
    ///
    /// use trees::bstree::BinarySearchTree;
    ///
    /// let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
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
    /// bst.insert(1);
    /// bst.delete(1);
    /// ```
    pub fn delete(&mut self, val: T) {
// <<<<<<< tests
        if self.root.is_none() {
            return
        } else {
            if let Some(root) = self.root.as_ref() {
                if root.borrow().data == val {
                    if root.borrow().left.is_none() && root.borrow().right.is_none() {
                        self.root = None;
                    } else if root.borrow().left.is_none() && !root.borrow().right.is_none() {
                        self.root.take().map(|node| {
                            self.root = node.borrow().right.clone()
                        });
                    } else if !root.borrow().left.is_none() && root.borrow().right.is_none() {
                        self.root.take().map(|node| {
                            self.root = node.borrow().left.clone()
                        });
                    } else {
                        BinarySearchTreeNode::_delete_node_have_two_children(root);
                    }
                } else {
                    root.borrow_mut().delete(val);
                }
            }
        }
// =======
        // BinarySearchTreeNode::delete(&mut self.root, val)
// >>>>>>> main
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_demo() {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.height(), 0);
        bst.insert(1);
        assert_eq!(bst.height(), 1);
        bst.insert(2);
        bst.delete(2);
        assert_eq!(bst.height(), 1);
    }

    #[test]
    fn test_len() {
        let mut bst = BinarySearchTree::new();
        bst.insert(0);
        assert_eq!(bst.len(), 1);
        bst.delete(0);
        assert_eq!(bst.len(), 0);
    }

    // test delete function
    //          5
    //        /   \
    //       3     7
    //      / \   / \
    //     2   4 6   8
    #[test]
    fn test_delete() {
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(2);
        bst.insert(4);
        bst.insert(7);
        bst.insert(6);
        bst.insert(8);
        assert_eq!(bst.height(), 3);
        assert_eq!(bst.count_leaves(), 4);
        bst.delete(2);
        assert_eq!(bst.height(), 3);
        assert_eq!(bst.count_leaves(), 3);
        bst.delete(3);
        bst.print_inorder();
        assert_eq!(bst.height(), 3);
        assert_eq!(bst.count_leaves(), 3);
        bst.delete(7);
        bst.print_inorder();
    }

    #[test]
    fn test_delete2() {
        // delete a left child with two children
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(2);
        bst.insert(4);
        bst.insert(7);
        bst.insert(6);
        bst.insert(8);
        assert_eq!(bst.len(), 7);
        bst.delete(3);
        assert_eq!(bst.len(), 6);
        bst.print_inorder();
    }

    #[test]
    fn test_delete3() {
        // delete a right child with two children
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(2);
        bst.insert(4);
        bst.insert(7);
        bst.insert(6);
        bst.insert(8);
        assert_eq!(bst.len(), 7);
        bst.delete(7);
        assert_eq!(bst.len(), 6);
        bst.print_inorder();
    }

    #[test]
    fn test_delete4() {
        // delete root with two children
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(2);
        bst.insert(4);
        bst.insert(7);
        bst.insert(6);
        bst.insert(8);
        assert_eq!(bst.len(), 7);
        bst.delete(5);
        assert_eq!(bst.len(), 6);
        bst.print_inorder();
    }
}
