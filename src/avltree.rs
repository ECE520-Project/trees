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
use std::cmp::{Ord, Ordering};

use crate::base::{QueryableTreeNode, QueryableTree};

type RcRefAVLTNode<T> = Rc<RefCell<AVLTreeNode<T>>>;
type AVLNodeLink<T> = Option<RcRefAVLTNode<T>>;

/// Node struct for [AVLTree](struct.AVLTree.html) struct
pub struct AVLTreeNode<T: Ord + Copy + fmt::Debug> {
    /// Data stored in the node
    pub data: T,
    left: AVLNodeLink<T>,
    right: AVLNodeLink<T>,
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
            left: None,
            right: None,
            height: 0,
        }
    }
    /// Insert a new node
    fn insert(&mut self, node: AVLTreeNode<T>) {
        match node.data.cmp(&self.data) {
            Ordering::Less => {
                match self.left.clone() {
                    None => self.left = Some(Rc::new(RefCell::new(node))),
                    Some(ref mut l) => l.borrow_mut().insert(node),
                }
            },
            Ordering::Greater => {
                match self.right.clone() {
                    None => self.right = Some(Rc::new(RefCell::new(node))),
                    Some(ref mut r) => r.borrow_mut().insert(node),
                }
            },
            _ => {},
        }
    }
}
/// Delete a node
fn delete<T: Ord + Copy + fmt::Debug>(data: T, root: RcRefAVLTNode<T>) -> AVLNodeLink<T>{
    match root.borrow().data.cmp(&data){
        Ordering::Equal =>  return delete_node(root.clone()),
        Ordering::Less => {
            if let Some(succ) = root.borrow().right.clone().take() {
                root.borrow_mut().right = delete(data, succ);
                return Some(updated_node(root.clone()))
            }
        },
        Ordering::Greater => {
            if let Some(succ) = root.borrow().left.clone().take() {
                let mut x = root.borrow().left.clone();
                x =  delete(data, succ);
                return Some(updated_node(root.clone()))
            }
        }
    }
    return Some(root.clone());
}

fn height<T: Ord + Copy + fmt::Debug>(node: AVLNodeLink<T>) -> u32  {
    return node.map_or(0, |succ| succ.borrow().height.clone())
}
fn update_height<T: Ord + Copy + fmt::Debug>(node: &mut RcRefAVLTNode<T>){
    let mut h = node.borrow().height.clone();
    h = cmp::max( height(node.borrow().left.clone()), height(node.borrow().right.clone()) )+1;
}
fn rotate_right<T: Ord + Copy + fmt::Debug>(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T>{
    let mut new_root = root.borrow_mut().left.take().expect("broken tree");
    root.borrow_mut().left = new_root.borrow().right.clone().take();
    update_height(&mut root);
    new_root.borrow_mut().right = Some(root);
    update_height(&mut new_root);
    return new_root
}
fn rotate_left<T: Ord + Copy + fmt::Debug>(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T>{
    let mut new_root = root.borrow_mut().right.take().expect("not accepted");
    root.borrow_mut().right = new_root.borrow().left.clone().take();
    update_height(&mut root);
    new_root.borrow_mut().left = Some(root);
    update_height(&mut new_root);
    return new_root
}
fn rotate_right_node<T: Ord + Copy + fmt::Debug>(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T> {
    let right = root.as_ref().borrow_mut().right.take().expect("not accepted");
    if height(right.borrow().left.clone()) > height(right.borrow().right.clone()) {
        let rotated_node = rotate_right(right);
        root.borrow_mut().right = Some(rotated_node);
        update_height(&mut root);
    }
    else {
        root.as_ref().borrow_mut().right = Some(right)
    }
    rotate_left(root)
}
fn rotate_left_node<T: Ord + Copy + fmt::Debug>(mut root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T> {
    let left = root.as_ref().borrow_mut().left.take().expect("not accepted");
    if height(left.borrow().left.clone()) < height(left.borrow().right.clone()) {
        let rotated_node = rotate_left(left);
        root.borrow_mut().left = Some(rotated_node);
        update_height(&mut root);
    }
    else{
        root.borrow_mut().left = Some(left);
    }
    rotate_right(root)
}
fn diff_height<T: Ord + Copy + fmt::Debug>(root: &RcRefAVLTNode<T>) -> i32 {
    let left_height = height(root.borrow().left.clone());
    let right_height = height(root.borrow().right.clone());
    (left_height as i32) - (right_height as i32)
}
fn rebalance<T: Ord + Copy + fmt::Debug>(root: RcRefAVLTNode<T>) -> RcRefAVLTNode<T> {
    let diff  = diff_height(&root);
    if -1 <= diff && diff <= 1 {return root}
    match diff{
        2 => rotate_left_node(root),
        -2 => rotate_right_node(root),
        _ => unreachable!()
    }
}
fn updated_node<T: Ord + Copy + fmt::Debug>(mut root: RcRefAVLTNode<T>) ->RcRefAVLTNode<T> {
    update_height(&mut root);
    rebalance(root.clone())
}
fn delete_left_node<T: Ord + Copy + fmt::Debug>(root: RcRefAVLTNode<T>, left: RcRefAVLTNode<T>) -> (AVLNodeLink<T>,RcRefAVLTNode<T>){
    let (new_left, min) =  delete_min(left);
    let mut x = root.borrow().left.clone();
    x = new_left;
    (Some(updated_node(root)),min)
}
fn delete_min<T: Ord + Copy + fmt::Debug>(root: RcRefAVLTNode<T>) -> (AVLNodeLink<T>, RcRefAVLTNode<T>) {
    match root.borrow().left.clone().take() {
        Some(left) => delete_left_node(root.clone(), left),
        None => (root.borrow().right.clone().take(), root.clone())
    }
}
fn combine_tree<T: Ord + Copy + fmt::Debug>(l: RcRefAVLTNode<T>, r: RcRefAVLTNode<T>) -> RcRefAVLTNode<T>{
    let (tree, min) = delete_min(r);
    let new_root = min;
    new_root.borrow_mut().left = Some(l);
    new_root.borrow_mut().right = tree;
    updated_node(new_root)
}
fn delete_node<T: Ord + Copy + fmt::Debug>(node: RcRefAVLTNode<T>) -> AVLNodeLink<T> {
    match node.borrow().left.clone().take(){
        None => match node.borrow().right.clone().take(){
            None => None,
            Some(r) => Some(r),
        }
        Some(l) => match node.borrow().right.clone().take(){
            None => Some(l),
            Some(r) => Some(combine_tree(l,r)),
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
    /// let mut avl = AVLTree::new();
    /// avl.delete(1);
    /// ```
    pub fn delete(&mut self, val:T){
        match self.root.take() {
            Some(node) => self.root = delete(val, node),
            None => return
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
        }
        assert_eq!(avl.len(), 9);
        assert_eq!(avl.is_empty(), false);
        assert_eq!(avl.height(), 7);
        assert_eq!(avl.contains(2),true);
        assert_eq!(avl.contains(8),false);
        assert_eq!(avl.min().unwrap(),0);
        assert_eq!(avl.max().unwrap(),10);

        println!("{:#?}",avl.print_inorder());
    }

    #[test]
    fn test_avl_delete() {
        let mut avl = AVLTree::<i32>::new();

        assert_eq!(avl.height(),0);

        avl.insert(2);
        avl.insert(4);
        avl.insert(6);
        assert_eq!(avl.height(),3);

        avl.delete(2);
        assert_eq!(avl.height(),2);

        avl.delete(4);
        assert_eq!(avl.height(),1);
    }

    #[test]
    fn insert_delete_inorder_avl() {
        let mut tree = AVLTree::new();
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
    fn insert_delete_reverse_inorder_avl() {
        let mut tree = AVLTree::new();
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
    fn insert_delete_random_avl() {
        let seed = [0u8; 32];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut tree = AVLTree::new();
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

    #[test]
    fn test_debug_delete_avl() {
        let mut tree = AVLTree::new();

        for x in vec![7, 2, 4, 0, 9, 3, 5, 8, 6, 1] {
            tree.insert(x);
        }

        for (i, v) in (0..10).enumerate() {
            tree.delete(v);
            assert_eq!(tree.len(), 10 - i - 1);
        }
    }
}

 