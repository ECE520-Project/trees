//! Utilities for binary search tree, red-black tree, and AVL tree.
//!
//! **trees** provides utilities to generate search tree data structures:
//!
//! * [Binary Search Tree](https://en.wikipedia.org/wiki/Binary_search_tree)
//! * [Red-black Tree](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree)
//! * [AVL Tree](https://en.wikipedia.org/wiki/AVL_tree)
//!
//! # Examples
//!
//! ## Binary Search Tree
//!
//! ```
//! use trees::prelude::*;
//!
//! let mut bst = BinarySearchTree::new();
//! bst.insert(3);
//! bst.insert(5);
//! bst.insert(0);
//! println!("height: {}", bst.height());
//! println!("is_empty: {}", bst.is_empty());
//! println!("count_leaves: {}", bst.count_leaves());
//! println!("min: {}", bst.min().unwrap());
//! println!("max: {}", bst.max().unwrap());
//! println!("contains 1: {}", bst.contains(1));
//! println!("contains 10: {}", bst.contains(10));
//! print!("print_inorder: ");
//! bst.print_inorder();
//! ```
//!
//! ## Red-black Tree
//!
//! ```
//! use trees::prelude::*;
//!
//! let mut rbt = RedBlackTree::new();
//! rbt.insert(3);
//! rbt.insert(5);
//! rbt.insert(0);
//! println!("height: {}", rbt.height());
//! println!("is_empty: {}", rbt.is_empty());
//! println!("count_leaves: {}", rbt.count_leaves());
//! println!("min: {}", rbt.min().unwrap());
//! println!("max: {}", rbt.max().unwrap());
//! println!("contains 2: {}", rbt.contains(2));
//! println!("contains 10: {}", rbt.contains(0));
//! print!("print_inorder: ");
//! rbt.print_inorder();
//! ```
//!
//! ## AVL Tree
//!
//! ```
//! use trees::prelude::*;
//!
//! let mut avl = AVLTree::new();
//! avl.insert(3);
//! avl.insert(5);
//! avl.insert(0);
//! println!("height: {}", avl.height());
//! println!("is_empty: {}", avl.is_empty());
//! println!("count_leaves: {}", avl.count_leaves());
//! println!("min: {}", avl.min().unwrap());
//! println!("max: {}", avl.max().unwrap());
//! println!("contains 2: {}", avl.contains(2));
//! println!("contains 10: {}", avl.contains(10));
//! print!("print_inorder: ");
//! avl.print_inorder();
//! ```

pub mod prelude;
pub mod rbtree;
pub mod avltree;
pub mod bstree;
pub mod base;

#[cfg(test)]
mod tests;
