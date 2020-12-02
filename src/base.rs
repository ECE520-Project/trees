//! An automatically-implemented extension trait for nodes and trees
//!
//! Provides query functions for trees and nodes.
//!
//! ```
//! use trees::base::{QueryableTreeNode, QueryableTree};
//! ```

use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;
use std::fmt;

/// Provide query functions for nodes
pub trait QueryableTreeNode<T: Ord + Copy + fmt::Debug> {
    /// Get left child node
    fn get_left(&self) -> &Option<Rc<RefCell<Self>>>;

    /// Get right child node
    fn get_right(&self) -> &Option<Rc<RefCell<Self>>>;

    /// Get data from current node
    fn get_data(&self) -> T;

    /// Return the height of current node, which will be called by
    /// [QueryableTree.height](trait.QueryableTree.html#method.height)
    fn height(&self) -> usize {
        let left_height = self.get_left().as_ref().map(
            |l| l.borrow().height()
        ).unwrap_or(0);
        let right_height = self.get_right().as_ref().map(
            |r| r.borrow().height()
        ).unwrap_or(0);
        max(left_height, right_height) + 1
    }

    /// Return the number of leaves, which will be called by
    /// [QueryableTree.count_leaves](trait.QueryableTree.html#method.count_leaves)
    fn count_leaves(&self) -> usize {
        match self.get_left() {
            None => match self.get_right() {
                None => 1,
                Some(r) => r.borrow().count_leaves(),
            }
            Some(l) => match self.get_right() {
                None => l.borrow().count_leaves(),
                Some(r) => {
                    l.borrow().count_leaves() + r.borrow().count_leaves()
                }
            }
        }
    }

    /// Print nodes [inorder](https://en.wikipedia.org/wiki/Tree_traversal#In-order_(LNR))
    /// , which will be called by
    /// [QueryableTree.print_inorder](trait.QueryableTree.html#method.print_inorder)
    fn print_inorder(&self) {
        if let Some(l) = self.get_left() {
            l.borrow().print_inorder();
        }
        print!("{:?} ", self.get_data());
        if let Some(r) = self.get_right() {
            r.borrow().print_inorder();
        }
    }

    /// Return the minimum value of current node, which will be called by
    /// [QueryableTree.min](trait.QueryableTree.html#method.min)
    fn min(&self) -> T {
        self.get_left().as_ref().map_or(
            self.get_data(),
            |x| x.borrow_mut().min()
        )
    }

    /// Return the maximum value of current node, which will be called by
    /// [QueryableTree.max](trait.QueryableTree.html#method.max)
    fn max(&self) -> T {
        self.get_right().as_ref().map_or(
            self.get_data(),
            |x| x.borrow_mut().max()
        )
    }

    /// Determine whether the node and its successors contains given value,
    /// which will be called by
    /// [QueryableTree.contains](trait.QueryableTree.html#method.contains)
    fn contains(&self, value: T) -> bool {
        return if self.get_data() == value {
            true
        } else if self.get_data() < value {
            self.get_right().as_ref().map(
                |node| node.borrow().contains(value)
            ).unwrap_or(false)
        } else {
            self.get_left().as_ref().map(
                |node| node.borrow().contains(value)
            ).unwrap_or(false)
        }
    }

    /// Return the length of the current node,
    /// which will be called by
    /// [QueryableTree.len](trait.QueryableTree.html#method.len)
    fn len(&self) -> usize {
        let left_len = self.get_left().as_ref().map(
            |n| n.borrow().len()
        ).unwrap_or(0);
        let right_len = self.get_right().as_ref().map(
            |n| n.borrow().len()
        ).unwrap_or(0);
        left_len + right_len + 1
    }
}

/// Provide query functions for trees
///
/// `QTN` means [QueryableTreeNode](trait.QueryableTreeNode.html)
pub trait QueryableTree<T: Ord + Copy + fmt::Debug, QTN: QueryableTreeNode<T>> {
    fn get_root(&self) -> &Option<Rc<RefCell<QTN>>>;

    /// Return the number of leaves.
    ///
    /// # Example
    ///
    /// ```
    /// //                root
    /// //               /    \
    /// //            node   leaf
    /// //           /   \
    /// //        leaf   node
    /// //              /    \
    /// //           leaf   leaf
    /// use trees::bstree::BinarySearchTree;
    /// use trees::base::QueryableTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(1);
    /// println!("{}", tree.count_leaves());  // 1
    /// tree.insert(0);
    /// println!("{}", tree.height());  // still 1
    /// ```
    fn count_leaves(&self) -> usize {
        match self.get_root() {
            None => 0,
            Some(node) => node.borrow().count_leaves(),
        }
    }

    /// Return the height of tree.
    ///
    /// # Example
    ///
    /// ```
    /// use trees::bstree::BinarySearchTree;
    /// use trees::base::QueryableTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// println!("{}", tree.height());  // 0
    /// tree.insert(1);
    /// println!("{}", tree.height());  // 1
    /// ```
    fn height(&self) -> usize {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().height(),
        }
    }

    /// Print tree [inorder](https://en.wikipedia.org/wiki/Tree_traversal#In-order_(LNR))
    ///
    /// # Example
    ///
    /// ```
    /// use trees::bstree::BinarySearchTree;
    /// use trees::base::QueryableTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(1);
    /// tree.insert(0);
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(2);
    /// tree.print_inorder(); // 0 1 2 3 5
    /// ```
    fn print_inorder(&self) {
        match &self.get_root() {
            None => println!("It is an empty tree!"),
            Some(node) => {
                node.borrow().print_inorder();
                println!();
            }
        }
    }

    /// Determine whether the tree is empty
    ///
    /// # Example
    ///
    /// ```
    /// use trees::bstree::BinarySearchTree;
    /// use trees::base::QueryableTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// println!("{}", tree.is_empty());  // true
    /// tree.insert(1);
    /// println!("{}", tree.is_empty());  // false
    /// ```
    fn is_empty(&self) -> bool {
        match self.get_root() {
            None => true,
            Some(_) => false
        }
    }

    /// Return the minimum value of the tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::bstree::BinarySearchTree;
    /// use trees::base::QueryableTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// println!("{:?}", tree.min());  // None
    /// tree.insert(1);
    /// tree.insert(0);
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(2);
    /// println!("{:?}", tree.min());  // Some(0)
    /// ```
    fn min(&self) -> Option<T> {
        match self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().min()),
        }
    }

    /// Return the maximum value of the tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::bstree::BinarySearchTree;
    /// use trees::base::QueryableTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// println!("{:?}", tree.max());  // None
    /// tree.insert(1);
    /// tree.insert(0);
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(2);
    /// println!("{:?}", tree.max());  // Some(5)
    /// ```
    fn max(&self) -> Option<T> {
        match self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().max()),
        }
    }

    /// Determine whether the tree contains given value
    ///
    /// # Example
    ///
    /// ```
    /// use trees::bstree::BinarySearchTree;
    /// use trees::base::QueryableTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(1);
    /// println!("{}", tree.contains(1));  // true
    /// println!("{}", tree.contains(0));  // false
    /// ```
    fn contains(&self, value: T) -> bool {
        match self.get_root() {
            None => false,
            Some(node) => node.borrow().contains(value),
        }
    }

    /// Return the length of the tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::bstree::BinarySearchTree;
    /// use trees::base::QueryableTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(1);
    /// tree.insert(10);
    /// tree.insert(13);
    /// println!("{}", tree.len());  // 3
    /// ```
    fn len(&self) -> usize {
        match self.get_root() {
            None => 0,
            Some(node) => node.borrow().len(),
        }
    }
}
