//! AVL tree
//!
//! You can generate an AVL tree, and insert or delete nodes.
//!
//! ```
//! use trees::avltree::AVLTree;
//! // use this trait if you want to query information
//! use trees::base::QueryableTree;
//! ```

use std::cmp;
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
    //height: usize,
    height:u32,
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
    }
    /// Delete a node
    fn delete(data: T, mut root: RcRefAVLTNode<T>) -> AVLNodeLink<T>{
        match root.borrow_mut().data.cmp(&data){
            Ordering::Equal =>  return Self::delete_node(root.clone()),
            Ordering::Less => {
                if let Some(succ) = root.borrow_mut().right.take() {
                    root.borrow_mut().right = Self::delete(data, succ);
                    return Some(Self::updated_node(root.clone()))
                }
            },
            Ordering::Greater => {
                if let Some(succ) = root.borrow_mut().left.take() {
                    root.borrow_mut().left =  Self::delete(data, succ);
                    return Some(Self::updated_node(root.clone()))
                }
            }
        }
        return Some(root);
    } 
    fn delete_node(mut node: RcRefAVLTNode<T>) -> AVLNodeLink<T> {
        match node.borrow_mut().left.take(){
            None => match node.borrow_mut().right.take(){
                None => None,
                Some(r) => Some(r),
            }
            Some(l) => match node.borrow_mut().right.take(){
                None => Some(l),
                Some(r) => Some(Self::combine_tree(l,r)),
            }
        }
    }
    
    fn height(node: &AVLNodeLink<T>) -> u32  {
        return node.as_ref().map_or(0, |succ| succ.borrow().height)
    }
    fn update_height(node:&mut AVLTreeNode<T>){
        node.height = cmp::max( Self::height(&node.left), Self::height(&node.right) )+1;
    }
    fn rotate_right(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T>{
        let new_root = root.as_ref().borrow_mut().left.take().expect("broken tree");
        root.as_ref().borrow_mut().left = new_root.as_ref().borrow_mut().right.take();
        Self::update_height(&mut root.as_ref().borrow_mut());
        new_root.borrow_mut().right = Some(root);
        Self::update_height(&mut new_root.as_ref().borrow_mut());
        return new_root
    }
    fn rotate_left(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T>{
        let mut new_root = root.as_ref().borrow_mut().right.take().expect("not accepted");
        root.as_ref().borrow_mut().right = new_root.as_ref().borrow_mut().left.take();
        Self::update_height(&mut root.as_ref().borrow_mut());
        new_root.borrow_mut().left = Some(root);
        Self::update_height(&mut new_root.as_ref().borrow_mut());
        return new_root
    }
    fn rotate_right_node(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T> {
        let right = root.as_ref().borrow_mut().right.take().expect("not accepted");
        if Self::height(&right.borrow_mut().left) > Self::height(&right.borrow_mut().right) {
            let rotated_node = Self::rotate_right(right);
            root.as_ref().borrow_mut().right = Some(rotated_node);
            Self::update_height(&mut root.as_ref().borrow_mut());
        }
        else {
            root.as_ref().borrow_mut().right = Some(right)
        }
        Self::rotate_left(root)
    }
    fn rotate_left_node(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T> {
        let left = root.as_ref().borrow_mut().left.take().expect("not accepted");
        if Self::height(&left.borrow_mut().left) < Self::height(&left.borrow_mut().right) {
            let rotated_node = Self::rotate_left(left);
            root.as_ref().borrow_mut().left = Some(rotated_node);
            Self::update_height(&mut root.as_ref().borrow_mut());
        }
        else{
            root.as_ref().borrow_mut().left = Some(left);
        }
        Self::rotate_right(root)
    }
    fn diff_height(root: &RcRefAVLTNode<T>) -> i32 {
        let left_height = Self::height(&root.as_ref().borrow_mut().left);
        let right_height = Self::height(&root.as_ref().borrow_mut().right);
        (left_height as i32) - (right_height as i32)
    }
    fn rebalance(root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T> {
        let diff  = Self::diff_height(&root);
        if -1 <= diff && diff <= 1 {return root}
        match diff{
            2 => Self::rotate_left_node(root),
            -2 => Self::rotate_right_node(root),
            _ => unreachable!()
        }
    }
    fn updated_node(root: RcRefAVLTNode<T>) ->RcRefAVLTNode<T> {
        Self::update_height(&mut root.as_ref().borrow_mut());
        Self::rebalance(root)
    }
    fn delete_left_node(mut root: RcRefAVLTNode<T>, left: RcRefAVLTNode<T>) -> (AVLNodeLink<T>,RcRefAVLTNode<T>){
        let (new_left, min) =  Self::delete_min(left);
        root.as_ref().borrow_mut().left = new_left;
        (Some(Self::updated_node(root)),min)
    }
    fn delete_min(mut root: RcRefAVLTNode<T>) -> (AVLNodeLink<T>, RcRefAVLTNode<T>) {
        match root.borrow_mut().left.take() {
            Some(left) => Self::delete_left_node(root.clone(), left),
            None => (root.borrow_mut().right.take(), root.clone())
        }
    }
    fn combine_tree(l: RcRefAVLTNode<T>, r: RcRefAVLTNode<T>) -> RcRefAVLTNode<T>{
        let (tree, min) = Self::delete_min(r);
        let mut new_root = min;
        new_root.borrow_mut().left = Some(l);
        new_root.borrow_mut().right = tree;
        Self::updated_node(new_root)
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
    pub fn delete(&mut self, val:T){
        match self.root.take() {
            Some(node) => self.root = AVLTreeNode::delete(val, node),
            None => return
        }
    }
}
