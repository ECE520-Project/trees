//! AVL tree
//!
//! You can generate an AVL tree, and insert or delete nodes.
//!
//! ```
//! use trees::avltree::AVLTree;
//! // use this trait if you want to query information
//! use trees::base::QueryableTree;
//! ```

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use std::cmp::{Ord};

use crate::base::{QueryableTreeNode, QueryableTree};

type RcRefAVLTNode<T> = Rc<RefCell<AVLTreeNode<T>>>;
type AVLNodeLink<T> = Option<RcRefAVLTNode<T>>;

/// Node struct for [AVLTree](struct.AVLTree.html) struct
pub struct AVLTreeNode<T: Ord + Copy + fmt::Debug> {
    /// Data stored in the node
    pub data: T,
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
    /// Create an new node, which will be called by [AVLTree](struct.AVLTree.html)
    fn new(data:T) -> AVLNodeLink<T>{
        Some(Rc::new(RefCell::new(Self {
            data,
            left: None,
            right: None,
            height: 1,
        })))
    }

    #[inline]
    fn _max(a: usize, b: usize) -> usize {
        if a > b {
            a
        } else {
            b
        }
    }

    fn _is_balanced(&self) -> bool {
        let left_height = self.get_left().as_ref().map(
            |n| n.borrow().height()
        ).unwrap_or(0);
        let right_height = self.get_right().as_ref().map(
            |n| n.borrow().height()
        ).unwrap_or(0);
        let delta_height = left_height as i64 - right_height as i64;
        return if delta_height.abs() > 1 {
            println!("{:?} {:?}", left_height, right_height);
            false
        } else {
            let left_balanced = self.get_left().as_ref().map(
                |n| n.borrow()._is_balanced()
            ).unwrap_or(true);
            let right_balanced = self.get_right().as_ref().map(
                |n| n.borrow()._is_balanced()
            ).unwrap_or(true);
            if left_balanced && right_balanced {
                true
            } else {
                false
            }
        }
    }

    fn _get_delta_height(n: &RcRefAVLTNode<T>) -> i64 {
        Self::_get_left_height(n) as i64 - Self::_get_right_height(n) as i64
    }

    fn _get_left_height(n: &RcRefAVLTNode<T>) -> usize {
        Self::_get_height(n.borrow().left.clone())
    }

    fn _get_right_height(n: &RcRefAVLTNode<T>) -> usize {
        Self::_get_height(n.borrow().right.clone())
    }

    fn _get_height(node: Option<RcRefAVLTNode<T>>) -> usize {
        node.map_or(0, |n| n.borrow().height)
    }
    #[allow(unused_mut)]
    fn _lr_rotate(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T> {
        let left = root.borrow().left.clone().take().unwrap();
        root.borrow_mut().left = Some(Self::_left_rotate(left));
        return Self::_right_rotate(root)
    }
    #[allow(unused_mut)]
    fn _rl_rotate(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T> {
        let right = root.borrow().right.clone().take().unwrap();
        root.borrow_mut().right = Some(Self::_right_rotate(right));
        return Self::_left_rotate(root)
    }
    #[allow(unused_mut)]
    fn _right_rotate(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T> {
        let mut new_root = root.borrow().left.clone().unwrap();
        root.borrow_mut().left = new_root.borrow().right.clone().take();
        root.borrow_mut().height = Self::_max(
            Self::_get_left_height(&root),
            Self::_get_right_height(&root)
        ) + 1;
        new_root.borrow_mut().right = Some(root);
        new_root.borrow_mut().height = Self::_max(
            Self::_get_left_height(&new_root),
            Self::_get_right_height(&new_root)
        ) + 1;
        return new_root
    }
    #[allow(unused_mut)]
    fn _left_rotate(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T> {
        let mut new_root = root.borrow().right.clone().unwrap();
        root.borrow_mut().right = new_root.borrow().left.clone().take();
        root.borrow_mut().height = Self::_max(
            Self::_get_left_height(&root),
            Self::_get_right_height(&root)
        ) + 1;
        new_root.borrow_mut().left = Some(root);
        new_root.borrow_mut().height = Self::_max(
            Self::_get_left_height(&new_root),
            Self::_get_right_height(&new_root)
        ) + 1;
        return new_root
    }
    #[allow(unused_mut)]
    /// Insert a node, which will be called by [AVLTree](struct.AVLTree.html)
    fn insert(node: AVLNodeLink<T>, data: T) -> AVLNodeLink<T> {
        // insert the node
        let ret_node = match node {
            None => AVLTreeNode::new(data).unwrap(),
            Some(mut n) => {
                let node_data = n.borrow().data;
                if data < node_data  {
                    let left = n.borrow().left.clone();
                    n.borrow_mut().left = Self::insert(left, data);
                } else if data > node_data {
                    let right = n.borrow().right.clone();
                    n.borrow_mut().right = Self::insert(right, data);
                }
                // else: data == node, nothing happens
                n
            }
        };
        // re-balance
        let delta_height = Self::_get_delta_height(&ret_node);
        let ret_node = if delta_height == 2 {
            if data < ret_node.borrow().left.clone().unwrap().borrow().data {
                Self::_right_rotate(ret_node)
            } else {
                Self::_lr_rotate(ret_node)
            }
        } else if delta_height == -2 {
            if data < ret_node.borrow().right.clone().unwrap().borrow().data {
                Self::_rl_rotate(ret_node)
            } else {
                Self::_left_rotate(ret_node)
            }
        } else {
            ret_node
        };
        // update height
        ret_node.borrow_mut().height = Self::_max(
            Self::_get_left_height(&ret_node),
            Self::_get_right_height(&ret_node)
        ) + 1;
        Some(ret_node)
    }
    #[allow(unused_variables)]
    /// Delete a node, which will be called by [AVLTree](struct.AVLTree.html)
    fn delete(node: AVLNodeLink<T>, data: T) -> AVLNodeLink<T> {
        // delete the node
        let ret_node = match node {
            None => node,
            Some(n) => {
                let node_data = n.borrow().data;
                // found the node which contains the same data
                if node_data == data {
                    let left = n.borrow().left.clone();
                    let right = n.borrow().right.clone();
                    let ret = match (left.clone(), right.clone()) {
                        (Some(l), Some(r)) => {
                            let min_val = r.borrow().min();
                            n.borrow_mut().data = min_val;
                            let right = n.borrow().right.clone().take();
                            n.borrow_mut().right = Self::delete(right, min_val);
                            Some(n)
                        }
                        (Some(l), _) => Some(l),
                        (_, Some(r)) => Some(r),
                        (_, None) => None,
                    };
                    ret
                }
                // go left
                else if node_data > data  {
                    let left = n.borrow().left.clone();
                    if left.is_none() {
                        return Some(n)
                    } else {
                        let left = n.borrow().left.clone().take();
                        n.borrow_mut().left = Self::delete(left, data);
                    }
                    Some(n)
                }
                // go right
                else {
                    let right = n.borrow().right.clone();
                    if right.is_none() {
                        return Some(n)
                    } else {
                        let right = n.borrow().right.clone().take();
                        n.borrow_mut().right = Self::delete(right, data);
                    }
                    Some(n)
                }
            }
        };
        // re-balance
        match ret_node {
            None => ret_node,
            Some(n) => {
                let delta_height = Self::_get_delta_height(&n);
                let ret_n = if delta_height == 2 {
                    if Self::_get_left_height(&n.borrow().left.clone().unwrap())
                        >= Self::_get_right_height(&n.borrow().left.clone().unwrap()) {
                        Self::_right_rotate(n)
                    } else {
                        Self::_lr_rotate(n)
                    }
                } else if delta_height == -2 {
                    if Self::_get_right_height(&n.borrow().right.clone().unwrap())
                        >= Self::_get_left_height(&n.borrow().right.clone().unwrap()) {
                        Self::_left_rotate(n)
                    } else {
                        Self::_rl_rotate(n)
                    }
                } else {
                    n
                };
                // update height
                ret_n.borrow_mut().height = Self::_max(
                    Self::_get_left_height(&ret_n),
                    Self::_get_right_height(&ret_n)
                ) + 1;
                Some(ret_n)
            }
        }
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
        match self.root.take() {
            Some(r) => self.root = AVLTreeNode::insert(Some(r), val),
            None => self.root = AVLTreeNode::new(val),
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
    /// avl.insert(1);
    /// avl.delete(1);
    /// ```
    pub fn delete(&mut self, val:T){
        match self.root.take() {
            Some(node) => self.root = AVLTreeNode::delete(Some(node), val),
            None => return
        }
    }

    fn _is_balanced(&self) -> bool {
        match self.get_root() {
            Some(node) => node.borrow()._is_balanced(),
            None => true
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};
    use rand::seq::SliceRandom;

    #[test]
    fn test_basic_avl() {
        let mut avl = AVLTree::<i32>::new();

        assert_eq!(avl.height(), 0);
        assert_eq!(avl.is_empty(), true);
        assert_eq!(avl.len(), 0);

        for a in vec![1, 0, 2, 3, 5, 10, 6, 9, 4] {
            avl.insert(a);
            avl.print_inorder();
        }
        avl.print_inorder();
        assert_eq!(avl.len(), 9);
        assert_eq!(avl.is_empty(), false);
        assert_eq!(avl.height(), 4);
        assert_eq!(avl.contains(2),true);
        assert_eq!(avl.contains(8),false);
        assert_eq!(avl.min().unwrap(),0);
        assert_eq!(avl.max().unwrap(),10);

        println!("{:#?}",avl.print_inorder());
    }

    #[test]
    fn insert_delete_inorder_avl() {
        let mut tree = AVLTree::new();
        let tree_size = 1000;
        for v in 0..tree_size {
            tree.insert(v);
            assert!(tree._is_balanced());
        }
        for (i, v) in (0..tree_size).enumerate() {
            tree.delete(v);
            assert!(tree._is_balanced());
            assert_eq!(tree.len(), tree_size - i - 1);
        }
    }

    #[test]
    fn insert_delete_reverse_inorder_avl() {
        let mut tree = AVLTree::new();
        let tree_size = 1000;
        for v in (0..tree_size).rev() {
            tree.insert(v);
            assert!(tree._is_balanced());
        }
        for (i, v) in (0..tree_size).rev().enumerate() {
            tree.delete(v);
            assert!(tree._is_balanced());
            assert_eq!(tree.len(), tree_size - i - 1);
        }
    }

    #[test]
    fn insert_delete_random_avl() {
        let seed = [0u8; 32];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut tree = AVLTree::new();
        let tree_size = 1000;
        let mut x: Vec<_> = (0..tree_size).collect();
        x.shuffle(&mut rng);

        for v in x.iter() {
            tree.insert(*v);
            assert!(tree._is_balanced());
        }
        assert_eq!(tree.len(), tree_size);
        assert!(tree._is_balanced());
        for (i, v) in x.iter().enumerate() {
            tree.delete(*v);
            assert_eq!(tree.len(), tree_size - i - 1);
            assert!(tree._is_balanced());
        }
    }

    #[test]
    fn test_debug_delete_avl() {
        let mut tree = AVLTree::new();

        for x in vec![7, 2, 4, 0, 9, 3, 5, 8, 6, 1] {
            tree.insert(x);
            assert!(tree._is_balanced());
            tree.print_inorder();
        }
        assert_eq!(tree.len(), 10);
        assert!(tree._is_balanced());
        tree.print_inorder();

        for (i, v) in (0..10).enumerate() {
            tree.delete(v);
            assert_eq!(tree.len(), 10 - i - 1);
            assert!(tree._is_balanced());
        }
    }
}

 