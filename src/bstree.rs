//! Binary search tree
//!
//! You can generate a binary search tree, and insert or delete nodes.
//!
//! ```
//! use trees::bstree::BinarySearchTree;
//! // use this trait if you want to query information
//! use trees::base::QueryableTree;
//! ```

use std::cell::{RefCell};
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

    /// Delete a node, which will be called by [BinarySearchTree](struct.BinarySearchTree.html)
    fn delete(&mut self, val: T) {
        match self.data.cmp(&val) {
            Ordering::Greater => self._delete_left(val),
            Ordering::Less => self._delete_right(val),
            _ => unreachable!(),
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
    /// Create a new Binary Search Tree
    ///
    /// # Example
    ///
    /// ```
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
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};
    use rand::seq::SliceRandom;

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
    fn test_count_leaves() {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.count_leaves(), 0);
        bst.insert(5);
        assert_eq!(bst.count_leaves(), 1);
        bst.insert(3);
        assert_eq!(bst.count_leaves(), 1);
        bst.insert(2);
        assert_eq!(bst.count_leaves(), 1);
        bst.insert(4);
        assert_eq!(bst.count_leaves(), 2);
        bst.insert(7);
        assert_eq!(bst.count_leaves(), 3);
        bst.insert(6);
        assert_eq!(bst.count_leaves(), 3);
        bst.insert(8);
        assert_eq!(bst.count_leaves(), 4);
    }

    #[test]
    fn test_height() {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.height(), 0);
        bst.insert(5);
        assert_eq!(bst.height(), 1);
        bst.insert(3);
        assert_eq!(bst.height(), 2);
        bst.insert(2);
        assert_eq!(bst.height(), 3);
        bst.insert(4);
        assert_eq!(bst.height(), 3);
        bst.insert(7);
        assert_eq!(bst.height(), 3);
        bst.insert(6);
        assert_eq!(bst.height(), 3);
        bst.insert(8);
        assert_eq!(bst.height(), 3);
        bst.insert(10);
        assert_eq!(bst.height(), 4);
    }

    #[test]
    fn test_is_empty() {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.is_empty(), true);
        bst.insert(5);
        assert_eq!(bst.is_empty(), false);
        bst.delete(5);
        assert_eq!(bst.is_empty(), true);
    }

    #[test]
    fn test_min() {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.min(), None);
        bst.insert(5);
        assert_eq!(bst.min(), Some(5));
        bst.insert(3);
        assert_eq!(bst.min(), Some(3));
        bst.insert(2);
        assert_eq!(bst.min(), Some(2));
        bst.insert(4);
        assert_eq!(bst.min(), Some(2));
        bst.insert(7);
        assert_eq!(bst.min(), Some(2));
        bst.insert(6);
        assert_eq!(bst.min(), Some(2));
        bst.insert(8);
        assert_eq!(bst.min(), Some(2));
    }

    #[test]
    fn test_max() {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.max(), None);
        bst.insert(5);
        assert_eq!(bst.max(), Some(5));
        bst.insert(3);
        assert_eq!(bst.max(), Some(5));
        bst.insert(2);
        assert_eq!(bst.max(), Some(5));
        bst.insert(4);
        assert_eq!(bst.max(), Some(5));
        bst.insert(7);
        assert_eq!(bst.max(), Some(7));
        bst.insert(6);
        assert_eq!(bst.max(), Some(7));
        bst.insert(8);
        assert_eq!(bst.max(), Some(8));
    }

    #[test]
    fn test_contains() {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.contains(5), false);
        bst.insert(5);
        assert_eq!(bst.contains(5), true);
        assert_eq!(bst.contains(3), false);
        bst.insert(3);
        assert_eq!(bst.contains(3), true);
        assert_eq!(bst.contains(2), false);
        bst.insert(2);
        assert_eq!(bst.contains(2), true);
        assert_eq!(bst.contains(4), false);
        bst.insert(4);
        assert_eq!(bst.contains(4), true);
        assert_eq!(bst.contains(7), false);
        bst.insert(7);
        assert_eq!(bst.contains(7), true);
        assert_eq!(bst.contains(6), false);
        bst.insert(6);
        assert_eq!(bst.contains(6), true);
        assert_eq!(bst.contains(8), false);
        bst.insert(8);
        assert_eq!(bst.contains(8), true);
    }

    #[test]
    fn test_len() {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.len(), 0);
        bst.insert(5);
        assert_eq!(bst.len(), 1);
        bst.insert(3);
        assert_eq!(bst.len(), 2);
        bst.insert(2);
        assert_eq!(bst.len(), 3);
        bst.delete(5);
        assert_eq!(bst.len(), 2);
        bst.delete(3);
        assert_eq!(bst.len(), 1);
        bst.delete(2);
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

    #[test]
    fn insert_delete_inorder() {
        let mut tree = BinarySearchTree::new();
        let tree_size = 1000;
        for v in 0..tree_size {
            tree.insert(v);
        }
        for (i, v) in (0..tree_size).enumerate() {
            tree.delete(v);
            assert_eq!(tree.len(), tree_size - i - 1);
        }
    }

    #[test]
    fn insert_delete_reverse_inorder() {
        let mut tree = BinarySearchTree::new();
        let tree_size = 1000;
        for v in (0..tree_size).rev() {
            tree.insert(v);
        }
        for (i, v) in (0..tree_size).rev().enumerate() {
            tree.delete(v);
            assert_eq!(tree.len(), tree_size - i - 1);
        }
    }

    #[test]
    fn insert_delete_random() {
        let seed = [0u8; 32];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut tree = BinarySearchTree::new();
        let tree_size = 1000;
        let mut x: Vec<_> = (0..tree_size).collect();
        x.shuffle(&mut rng);

        for v in x.iter() {
            tree.insert(*v);
        }
        for (i, v) in x.iter().enumerate() {
            tree.delete(*v);
            assert_eq!(tree.len(), tree_size - i - 1);
        }
    }
}
