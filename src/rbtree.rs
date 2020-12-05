//! Red-black tree
//!
//! You can generate a red-black tree, and insert or delete nodes.
//!
//! ```
//! use trees::rbtree::RedBlackTree;
//! // use this trait if you want to query information
//! use trees::base::QueryableTree;
//! ```

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::base::{QueryableTree, QueryableTreeNode};

type RcRefRBTNode<T> = Rc<RefCell<RedBlackTreeNode<T>>>;
type RBNodeLink<T> = Option<RcRefRBTNode<T>>;

/// Color representation for the [Node](struct.RedBlackTreeNode.html)
/// of [RedBlackTree](struct.RedBlackTree.html) struct
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeColor {
    /// Red color
    Red,
    /// Black color, the root of [RedBlackTree](struct.RedBlackTree.html) will be set to Black
    Black,
}

/// Node struct for [RedBlackTree](struct.RedBlackTree.html) struct
pub struct RedBlackTreeNode<T: Ord + Copy + fmt::Debug> {
    /// Data stored in the node
    pub data: T,
    /// The color of the node
    pub color: NodeColor,
    parent: RBNodeLink<T>,
    left: RBNodeLink<T>,
    right: RBNodeLink<T>,
}

impl<T: Ord + Copy + fmt::Debug> RedBlackTreeNode<T> {
    // fn new(data: T) -> RcRefRBTNode<T> {
    //     Rc::new(RefCell::new(Self {
    //         data: data,
    //         color: NodeColor::Black,
    //         parent: None,
    //         left: None,
    //         right: None,
    //     }))
    // }

    fn new(data: T, color: NodeColor, parent: RBNodeLink<T>) -> RcRefRBTNode<T> {
        Rc::new(RefCell::new(Self {
            data: data,
            color,
            parent,
            left: None,
            right: None,
        }))
    }

    // ------------------------------------------------------------------------
    // Here are some functions which are unique to red black tree
    
    /// Rotate the subtree rooted at this node to the right and
    /// returns the new root to this subtree.
    fn rotate_right(node: RcRefRBTNode<T>) -> RBNodeLink<T> {
        let parent = node.borrow().parent.clone();
        let left = node.borrow().left.clone();
        node.borrow_mut().left = left.clone().unwrap().borrow().right.clone();
        if node.borrow().left.is_some() {
            let left = node.borrow().left.clone().unwrap();
            left.borrow_mut().parent = Some(node.clone());
        }
        node.borrow_mut().parent = left.clone();
        left.clone().unwrap().borrow_mut().right = Some(node.clone());
        if parent.is_some() {
            let right = parent.clone().unwrap().borrow().right.clone();
            match right {
                Some(right) if Rc::ptr_eq(&right, &node) => {
                    parent.clone().unwrap().borrow_mut().right = left.clone();
                }
                _ => parent.clone().unwrap().borrow_mut().left = left.clone(),
            }
        }

        left.clone().unwrap().borrow_mut().parent = parent;
        left
    }

    /// Rotate the subtree rooted at this node to the left and
    /// return the new root to this subtree.
    fn rotate_left(node: RcRefRBTNode<T>) -> RBNodeLink<T> {
        let parent = node.borrow().parent.clone();
        let right = node.borrow().right.clone();
        node.borrow_mut().right = right.clone().unwrap().borrow().left.clone();
        if node.borrow().right.is_some() {
            let right = node.borrow().right.clone().unwrap();
            right.borrow_mut().parent = Some(node.clone());
        }
        node.borrow_mut().parent = right.clone();
        right.clone().unwrap().borrow_mut().left = Some(node.clone());
        if parent.is_some() {
            let left = parent.clone().unwrap().borrow().left.clone();
            match left {
                Some(left) if Rc::ptr_eq(&left, &node) => {
                    parent.clone().unwrap().borrow_mut().left = right.clone();
                }
                _ => parent.clone().unwrap().borrow_mut().right = right.clone(),
            }
        }

        right.clone().unwrap().borrow_mut().parent = parent;
        right
    }

    /// Insert data into the subtree rooted at self,performs any rotations
    /// necessary to maintain banlance, and then returns the new root to this subtree.
    fn insert(node: RcRefRBTNode<T>, data: T) -> RBNodeLink<T> {
        let node_data = node.borrow().data;
        if node_data == data {
            return Some(node);
        } else if node_data > data {
            let left = node.borrow().left.clone();
            match left {
                Some(left) => {
                    Self::insert(left, data);
                }
                None => {
                    node.borrow_mut().left =
                        Some(Self::new(data, NodeColor::Red, Some(node.clone())));
                    let left = node.borrow().left.clone();
                    Self::insert_repair(left.unwrap());
                }
            }
        } else {
            let right = node.borrow().right.clone();
            match right {
                Some(right) => {
                    Self::insert(right, data);
                }
                None => {
                    node.borrow_mut().right =
                        Some(Self::new(data, NodeColor::Red, Some(node.clone())));
                    let right = node.borrow().right.clone().unwrap();
                    Self::insert_repair(right);
                }
            }
        }

        let parent = node.borrow().parent.clone();
        if parent.is_some() {
            parent
        } else {
            Some(node)
        }
    }

    /// Repair the coloring from inserting into a tree.
    fn insert_repair(node: RcRefRBTNode<T>) {
        let parent = node.borrow().parent.clone();
        match parent {
            //This node is the root,so it just needs to be black
            None => node.borrow_mut().color = NodeColor::Black,
            //If the parent is black, then node just needs to be red
            Some(parent) if Self::color(Some(parent.clone())) == NodeColor::Black => {
                node.borrow_mut().color = NodeColor::Red;
            }
            Some(parent) => {
                let uncle = Self::sibling(parent.clone());
                match Self::color(uncle.clone()) {
                    NodeColor::Black => {
                        if Self::is_left(node.clone()) && Self::is_right(parent.clone()) {
                            Self::rotate_right(parent);
                            let right = node.borrow().right.clone();
                            Self::insert_repair(right.unwrap())
                        } else if Self::is_right(node.clone()) && Self::is_left(parent.clone()) {
                            Self::rotate_left(parent);
                            let left = node.borrow().left.clone();
                            Self::insert_repair(left.unwrap());
                        } else if Self::is_left(node.clone()) {
                            let grandparent = Self::grandparent(node.clone());
                            Self::rotate_right(grandparent.unwrap());
                            let parent = node.borrow().parent.clone();
                            let parent = parent.unwrap();
                            parent.borrow_mut().color = NodeColor::Black;
                            let right = parent.borrow().right.clone();
                            right.unwrap().borrow_mut().color = NodeColor::Red;
                        } else {
                            let grandparent = Self::grandparent(node.clone());
                            Self::rotate_left(grandparent.unwrap());
                            let parent = node.borrow().parent.clone();
                            let parent = parent.unwrap();
                            parent.borrow_mut().color = NodeColor::Black;
                            let left = parent.borrow().left.clone();
                            left.unwrap().borrow_mut().color = NodeColor::Red;
                        }
                    }
                    NodeColor::Red => {
                        parent.borrow_mut().color = NodeColor::Black;
                        uncle.unwrap().borrow_mut().color = NodeColor::Black;
                        let grandparent = Self::grandparent(node.clone()).unwrap();
                        grandparent.borrow_mut().color = NodeColor::Red;
                        Self::insert_repair(grandparent);
                    }
                }
            }
        }
    }

    /// Delete data from this tree
    fn delete(node: RcRefRBTNode<T>, val: T) -> RBNodeLink<T> {
        let node_data = node.borrow().data;
        if node_data == val {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            match (left.clone(), right.clone()) {
            //It's easier to balance a node with at most one child,
            //So we replace this node with the greatest one less than it and 
            //delete that.  
                (Some(left), Some(_right)) => {
                    let v = Self::get_max(left.clone());
                    node.borrow_mut().data = v;
                    Self::delete(left, v);
                }
            //This node has at most one non-None child,so we don't need to replace    
                _ => {
                    if node.borrow().color == NodeColor::Red {
                        let parent = node.borrow().parent.clone().unwrap();
                    //This node is red, and its child is black
                    //The only way this happens to a node with one child
                    //if both children are None leaves,
                    //we can just delete this node and call it a day.
                        if Self::is_left(node.clone()) {
                            parent.borrow_mut().left = None;
                        } else {
                            parent.borrow_mut().right = None;
                        }
                    } else {
                        //The node is black
                        if left.is_none() && right.is_none() {
                            let parent = node.borrow().parent.clone();
                            match parent {
                                None => return None,
                                //This node and its child are black
                                Some(_parent) => {
                                    Self::delete_repair(node.clone());
                                    let parent = node.borrow().parent.clone();
                                    let parent = parent.unwrap();
                                    if Self::is_left(node.clone()) {
                                        parent.borrow_mut().left = None;
                                    } else {
                                        parent.borrow_mut().right = None;
                                    }
                                    node.borrow_mut().parent = None;
                                }
                            }
                        }
                        // This node is black and its child is red
                        // Move the child node here and make it black  
                        else {
                            let child = left.unwrap_or_else(|| right.unwrap());
                            let child_data = child.borrow().data;
                            let child_left = child.borrow().left.clone();
                            let child_right = child.borrow().right.clone();
                            node.borrow_mut().data = child_data;
                            node.borrow_mut().left = child_left;
                            node.borrow_mut().right = child_right;
                            if node.borrow().left.is_some() {
                                let left = node.borrow().left.clone().unwrap();
                                left.borrow_mut().parent = Some(node.clone());
                            }
                            if node.borrow().right.is_some() {
                                let right = node.borrow().right.clone().unwrap();
                                right.borrow_mut().parent = Some(node.clone());
                            }
                        }
                    }
                }
            }
        } else if node_data > val {
            let left = node.borrow().left.clone();
            if left.is_some() {
                Self::delete(left.unwrap(), val);
            }
        } else {
            let right = node.borrow().right.clone();
            if right.is_some() {
                Self::delete(right.unwrap(), val);
            }
        }

        let parent = node.borrow().parent.clone();
        if parent.is_some() {
            parent
        } else {
            Some(node)
        }
    }

    /// Repair the coloring of the tree that may have been messed up.
    fn delete_repair(node: RcRefRBTNode<T>) {
        let node_sibling = Self::sibling(node.clone());
        if Self::color(node_sibling.clone()) == NodeColor::Red {
            let node_sibling = node_sibling.unwrap();
            node_sibling.borrow_mut().color = NodeColor::Black;
            let parent = node.borrow().parent.clone().unwrap();
            parent.borrow_mut().color = NodeColor::Red;
            if Self::is_left(node.clone()) {
                Self::rotate_left(parent);
            } else {
                Self::rotate_right(parent);
            }
        }

        let node_sibling = Self::sibling(node.clone());
        let parent = node.borrow().parent.clone();
        if Self::color(parent.clone()) == NodeColor::Black
            && Self::color(node_sibling.clone()) == NodeColor::Black
        {   //fixed
            match node_sibling {
                Some(node_sibling) =>{
            //let node_sibling = node_sibling.unwrap();
            let left = node_sibling.borrow().left.clone();
            let right = node_sibling.borrow().right.clone();
            if Self::color(left) == NodeColor::Black && Self::color(right) == NodeColor::Black {
                node_sibling.borrow_mut().color = NodeColor::Red;
                Self::delete_repair(parent.unwrap());
                return;
            }
        }
         None =>{
          }
        }
     }
        let node_sibling = Self::sibling(node.clone());
        let parent = node.borrow().parent.clone();
        if Self::color(parent.clone()) == NodeColor::Red
            && Self::color(node_sibling.clone()) == NodeColor::Black
        {
            let node_sibling = node_sibling.unwrap();
            let left = node_sibling.borrow().left.clone();
            let right = node_sibling.borrow().right.clone();
            if Self::color(left) == NodeColor::Black && Self::color(right) == NodeColor::Black {
                node_sibling.borrow_mut().color = NodeColor::Red;
                parent.unwrap().borrow_mut().color = NodeColor::Black;
                return;
            }
        }

        let node_sibling = Self::sibling(node.clone());
        if Self::is_left(node.clone()) && Self::color(node_sibling.clone()) == NodeColor::Black {
            let node_sibling = node_sibling.unwrap();
            let left = node_sibling.borrow().left.clone();
            let right = node_sibling.borrow().right.clone();
            if Self::color(right.clone()) == NodeColor::Black && Self::color(left) == NodeColor::Red
            {
                Self::rotate_right(node_sibling);
                let node_sibling = Self::sibling(node.clone());
                let node_sibling = node_sibling.unwrap();
                node_sibling.borrow_mut().color = NodeColor::Black;
                let right = node_sibling.borrow().right.clone();
                let right = right.unwrap();
                right.borrow_mut().color = NodeColor::Red;
            }
        }

        let node_sibling = Self::sibling(node.clone());
        if Self::is_right(node.clone()) && Self::color(node_sibling.clone()) == NodeColor::Black {
            let node_sibling = node_sibling.unwrap();
            let left = node_sibling.borrow().left.clone();
            let right = node_sibling.borrow().right.clone();
            if Self::color(right.clone()) == NodeColor::Red
                && Self::color(left.clone()) == NodeColor::Black
            {
                Self::rotate_left(node_sibling);
                let node_sibling = Self::sibling(node.clone());
                let node_sibling = node_sibling.unwrap();
                node_sibling.borrow_mut().color = NodeColor::Black;
                let left = node_sibling.borrow().left.clone();
                let left = left.unwrap();
                left.borrow_mut().color = NodeColor::Red;
            }
        }

        let node_sibling = Self::sibling(node.clone());
        if Self::is_left(node.clone()) && Self::color(node_sibling.clone()) == NodeColor::Black {
            let node_sibling = node_sibling.unwrap();
            let right = node_sibling.borrow().right.clone();
            if Self::color(right.clone()) == NodeColor::Red {
                let parent = node.borrow().parent.clone();
                Self::rotate_left(parent.unwrap());
                let grandparent = Self::grandparent(node.clone()).unwrap();
                let parent = node.borrow().parent.clone();
                let parent = parent.unwrap();
                grandparent.borrow_mut().color = parent.borrow().color;
                parent.borrow_mut().color = NodeColor::Black;
                let sibling = Self::sibling(parent).unwrap();
                sibling.borrow_mut().color = NodeColor::Black;
            }
        }

        let node_sibling = Self::sibling(node.clone());
        if Self::is_right(node.clone()) && Self::color(node_sibling.clone()) == NodeColor::Black {
            let node_sibling = node_sibling.unwrap();
            let left = node_sibling.borrow().left.clone();
            if Self::color(left.clone()) == NodeColor::Red {
                Self::rotate_right(parent.clone().unwrap());
                let grandparent = Self::grandparent(node.clone()).unwrap();
                let parent = node.borrow().parent.clone();
                let parent = parent.unwrap();
                grandparent.borrow_mut().color = parent.borrow().color;
                parent.borrow_mut().color = NodeColor::Black;
                let sibling = Self::sibling(parent).unwrap();
                sibling.borrow_mut().color = NodeColor::Black;
            }
        }
    }
    #[allow(dead_code)]
     /// Check the coloring of the tree, and return true if the tree
     /// is colored in a way which matches these 5 Properties:
     ///   1. Each node is either red or black
     ///   2. The root node is black
     ///   3. All leaves are black
     ///   4. If a node is red, then both its children are black
     ///   5. Every path from any node to all of its descendent Nil nodes
     /// has the same number of black nodes.
    fn check_color_properties(node: RcRefRBTNode<T>) -> bool {
        // Propertity 1 is easy to get because nothing that can make the color
        // be anything other than red or black
            
        // Property 2
        if node.borrow().color == NodeColor::Red {
            return false;
        }
        // Propertity 3 does not need to be checked, because None is assumed to be black.
      
        // Propertity 4
        if !Self::check_coloring(node.clone()) {
            return false;
        }
        //Propertity 5 
        if Self::black_height(Some(node)).is_none() {
            return false;
        }
        //If all properties are met
        true
    }

         /// A helper function to recursively check Property 4 of a Red-Black tree.
         fn check_coloring(node: RcRefRBTNode<T>) -> bool {
            if node.borrow().color == NodeColor::Red {
                if Self::color(node.borrow().left.clone()) == NodeColor::Red
                    || Self::color(node.borrow().right.clone()) == NodeColor::Red
                {
                    return false;
                }
            }
    
            let left = node.borrow().left.clone();
            match left {
                Some(left) => {
                    if !Self::check_coloring(left) {
                        return false;
                    }
                }
                None => (),
            }
    
            let right = node.borrow().right.clone();
            match right {
                Some(right) => {
                    if !Self::check_coloring(right) {
                        return false;
                    }
                }
                None => (),
            }
    
            true
        }

    /// Return the number of black nodes from this node to the leaves
    /// of the tree. or None if there is not one such value.
    fn black_height(node: RBNodeLink<T>) -> Option<usize> {
        match node {
            // If we are already at a leaf,there is no path
            None => Some(1),
            Some(node) => {
                let lh = Self::black_height(node.borrow().left.clone());
                let rh = Self::black_height(node.borrow().right.clone());
                match (lh, rh) {
                    (Some(lh), Some(rh)) => {
                        if lh != rh {
                        //The 2 children have unequal depths
                            None
                        } else {
                            let node_color = node.borrow().color;
                        //Return the black depth of children,plus 1 if the node is black
                            match node_color {
                                NodeColor::Red => Some(lh),
                                NodeColor::Black => Some(lh + 1),
                            }
                        }
                    }
                    //There are issues with coloring below children nodes
                    _ => None,
                }
            }
        }
    }

    // ------------------------------------------------------------
    // Here are some functions which are general to all binary search trees
    #[allow(dead_code)]
    fn search(node: RcRefRBTNode<T>, v: T) -> RBNodeLink<T> {
         //Search through the trees for data, returning its node if it is 
        //found and None otherwise.
        let node_data = node.borrow().data;
        if node_data == v {
            Some(node)
        } else if v > node_data {
            let right = node.borrow().right.clone();
            match right {
                None => None,
                Some(right) => Self::search(right, v),
            }
        } else {
            let left = node.borrow().left.clone();
            match left {
                None => None,
                Some(left) => Self::search(left, v),
            }
        }
    }

    fn get_max(node: RcRefRBTNode<T>) -> T {
        // Return the largest element in the tree
       match node.borrow().right.clone() {
           // go as far right as possible
           Some(right) => Self::get_max(right),
           None => node.borrow().data,
       }
   }

    fn grandparent(node: RcRefRBTNode<T>) -> RBNodeLink<T> {
        // Get the current node's grandparent, or None if it does not exist.
        match node.borrow().parent.clone() {
            Some(parent) => parent.borrow().parent.clone(),
            _ => None,
        }
    }

    fn sibling(node: RcRefRBTNode<T>) -> RBNodeLink<T> {
        // Get the current node's sibling, or None if it does not exist.
        match node.borrow().parent.clone() {
            None => None,
            Some(parent) => {
                let left = parent.borrow().left.clone();
                match left {
                    Some(left) if Rc::ptr_eq(&left, &node) => parent.borrow().right.clone(),
                    _ => left,
                }
            }
        }
    }

    fn is_left(node: RcRefRBTNode<T>) -> bool {
        // Return true if the node is the left child of its parent.
        match node.borrow().parent.clone() {
            Some(parent) => match parent.borrow().left.clone() {
                Some(left) => Rc::ptr_eq(&left, &node),
                None => false,
            },
            _ => false,
        }
    }

    fn is_right(node: RcRefRBTNode<T>) -> bool {
        // Return true if the node is the right child of its parent.
        match node.borrow().parent.clone() {
            Some(parent) => match parent.borrow().right.clone() {
                Some(right) => Rc::ptr_eq(&right, &node),
                None => false,
            },
            _ => false,
        }
    }

    fn color(node: RBNodeLink<T>) -> NodeColor {
        // Return the color of a node,allowing for none leaves.
        match node {
            None => NodeColor::Black,
            Some(v) => v.borrow().color,
        }
    }
    #[allow(dead_code)]
    fn is_equal(left: RBNodeLink<T>, right: RBNodeLink<T>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(left), Some(right)) => {
                let left_data = left.borrow().data;
                let right_data = right.borrow().data;
                //Test if 2 trees are equal
                if left_data == right_data {
                    let left_left = left.borrow().left.clone();
                    let left_right = left.borrow().right.clone();
                    let right_left = right.borrow().left.clone();
                    let right_right = right.borrow().right.clone();
                    Self::is_equal(left_left, right_left) && Self::is_equal(left_right, right_right)
                } else {
                    false
                }
            }
        }
    }

    #[allow(dead_code)]
    fn preorder_traverse(node: RcRefRBTNode<T>, container: &mut Vec<T>) {
        container.push(node.borrow().data);
        let left = node.borrow().left.clone();
        if left.is_some() {
            Self::preorder_traverse(left.unwrap(), container);
        }
        let right = node.borrow().right.clone();
        if right.is_some() {
            Self::preorder_traverse(right.unwrap(), container);
        }
    }
    #[allow(dead_code)]
    fn inorder_traverse(node: RcRefRBTNode<T>, container: &mut Vec<T>) {
        let left = node.borrow().left.clone();
        if left.is_some() {
            Self::inorder_traverse(left.unwrap(), container);
        }
        container.push(node.borrow().data);
        let right = node.borrow().right.clone();
        if right.is_some() {
            Self::inorder_traverse(right.unwrap(), container);
        }
    }
    #[allow(dead_code)]
    fn postorder_traverse(node: RcRefRBTNode<T>, container: &mut Vec<T>) {
        let left = node.borrow().left.clone();
        if left.is_some() {
            Self::postorder_traverse(left.unwrap(), container);
        }
        let right = node.borrow().right.clone();
        if right.is_some() {
            Self::postorder_traverse(right.unwrap(), container);
        }
        container.push(node.borrow().data);
    }

    fn clear(&mut self) {
        self.parent = None;
        match self.left.take() {
            None => {},
            Some(node) => {
                node.borrow_mut().clear();
            }
        }
        self.left = None;
        match self.right.take() {
            None => {},
            Some(node) => {
                node.borrow_mut().clear();
            }
        }
        self.right = None;
    }
}

/// An implementation of [Red-black Tree](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree)
pub struct RedBlackTree<T: Ord + Copy + fmt::Debug> {
    root: RBNodeLink<T>,
}

impl<T: Ord + Copy + fmt::Debug> Drop for RedBlackTree<T> {
    fn drop(&mut self) {
        match self.root.take() {
            Some(node) => node.borrow_mut().clear(),
            None => return
        }
    }
}

impl<T: Ord + Copy + fmt::Debug> Drop for RedBlackTreeNode<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T: Ord + Copy + fmt::Debug> QueryableTreeNode<T> for RedBlackTreeNode<T> {
    fn get_left(&self) -> &RBNodeLink<T> {
        return &self.left;
    }
    fn get_right(&self) -> &RBNodeLink<T> {
        return &self.right;
    }
    fn get_data(&self) -> T {
        return self.data;
    }
}

impl<T: Ord + Copy + fmt::Debug> QueryableTree<T, RedBlackTreeNode<T>> for RedBlackTree<T> {
    fn get_root(&self) -> &RBNodeLink<T> {
        &self.root
    }
}

impl<T: Ord + Copy + fmt::Debug> RedBlackTree<T> {
    /// Create a new Red-black Tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::rbtree::RedBlackTree;
    ///
    /// let mut rbt: RedBlackTree<i32> = RedBlackTree::new();
    /// ```
    pub fn new() -> Self {
        Self { root: None }
    }

    // /// Create a new Red-black Tree
    // ///
    // /// # Example
    // ///
    // /// ```
    // /// use trees::rbtree::RedBlackTree;
    // ///
    // /// let mut rbt = RedBlackTree::new_from_data(3);
    // /// ```
    // pub fn new_from_data(data: T) -> Self {
    //     Self {
    //         root: Some(Rc::new(RefCell::new(RedBlackTreeNode {
    //             data,
    //             color: NodeColor::Black,
    //             parent: None,
    //             left: None,
    //             right: None,
    //         }))),
    //     }
    // }

    /// Insert a new value to the tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::rbtree::RedBlackTree;
    ///
    /// let mut rbt = RedBlackTree::new();
    /// rbt.insert(1);
    /// ```
    pub fn insert(&mut self, val: T) {
        match self.root.clone() {
            Some(root) => {
                let r = RedBlackTreeNode::insert(root, val);
                self.root = r;
            }
            None => {
                self.root = Some(Rc::new(RefCell::new(RedBlackTreeNode {
                    data: val,
                    color: NodeColor::Black,
                    parent: None,
                    left: None,
                    right: None,
                })));
            }
        }
    }

    /// Delete a value from the tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::rbtree::RedBlackTree;
    ///
    /// let mut rbt = RedBlackTree::new();
    /// rbt.delete(1);
    /// ```
    pub fn delete(&mut self, val: T) {
        match self.root.clone() {
            Some(root) => {
                let r = RedBlackTreeNode::delete(root, val);
                self.root = r;
            }
            None => (),
        }
    }
    #[allow(dead_code)]
    fn is_equal(&self, other: &RedBlackTree<T>) -> bool {
        RedBlackTreeNode::is_equal(self.root.clone(), other.root.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};
    use rand::seq::SliceRandom;

    #[test]
    //""Test that the rotate_left and rotate_right functions work."""
    // Make a tree to test on
    fn rotations() {
        let mut tree = RedBlackTree::new();
        tree.insert(0);
        {
            let root = tree.root.clone().unwrap();
            root.borrow_mut().left = Some(RedBlackTreeNode::new(
                -10,
                NodeColor::Black,
                Some(root.clone()),
            ));
            root.borrow_mut().right = Some(RedBlackTreeNode::new(
                10,
                NodeColor::Black,
                Some(root.clone()),
            ));
            let left = root.borrow().left.clone();
            let left = left.unwrap();
            left.borrow_mut().left = Some(RedBlackTreeNode::new(
                -20,
                NodeColor::Black,
                Some(left.clone()),
            ));
            left.borrow_mut().right = Some(RedBlackTreeNode::new(
                -5,
                NodeColor::Black,
                Some(left.clone()),
            ));
            let right = root.borrow().right.clone();
            let right = right.unwrap();
            right.borrow_mut().left = Some(RedBlackTreeNode::new(
                5,
                NodeColor::Black,
                Some(right.clone()),
            ));
            right.borrow_mut().right = Some(RedBlackTreeNode::new(
                20,
                NodeColor::Black,
                Some(right.clone()),
            ));
        }
        // Make the left rotation
        let mut left_rot = RedBlackTree::new();
        left_rot.insert(10);
        {
            let root = left_rot.root.clone().unwrap();
            root.borrow_mut().left = Some(RedBlackTreeNode::new(
                0,
                NodeColor::Black,
                Some(root.clone()),
            ));
            let left = root.borrow().left.clone();
            let left = left.unwrap();
            left.borrow_mut().left = Some(RedBlackTreeNode::new(
                -10,
                NodeColor::Black,
                Some(left.clone()),
            ));
            left.borrow_mut().right = Some(RedBlackTreeNode::new(
                5,
                NodeColor::Black,
                Some(left.clone()),
            ));
            let left = left.borrow().left.clone();
            let left = left.unwrap();
            left.borrow_mut().left = Some(RedBlackTreeNode::new(
                -20,
                NodeColor::Black,
                Some(left.clone()),
            ));
            left.borrow_mut().right = Some(RedBlackTreeNode::new(
                -5,
                NodeColor::Black,
                Some(left.clone()),
            ));
            root.borrow_mut().right = Some(RedBlackTreeNode::new(
                20,
                NodeColor::Black,
                Some(root.clone()),
            ));
        }

        {
            let root = tree.root.clone().unwrap();
            tree.root = RedBlackTreeNode::rotate_left(root);
        }
        assert!(tree.is_equal(&left_rot))
    }

    #[test]
    fn insert() {
        // Test the insert() method of the tree correctly
        // balances, colors and inserts.
        let mut tree = RedBlackTree::new();
        tree.insert(0);
        vec![8, -8, 4, 12, 10, 11].iter().for_each(|v| {
            tree.insert(*v);
        });

        let mut ans = RedBlackTree::new();
        ans.insert(0);
        {
            let root = ans.root.clone().unwrap();
            root.borrow_mut().left = Some(RedBlackTreeNode::new(
                -8,
                NodeColor::Black,
                Some(root.clone()),
            ));
            root.borrow_mut().right = Some(RedBlackTreeNode::new(
                8,
                NodeColor::Red,
                Some(root.clone()),
            ));
            let right = root.borrow().right.clone();
            let right = right.unwrap();
            right.borrow_mut().left = Some(RedBlackTreeNode::new(
                4,
                NodeColor::Black,
                Some(right.clone()),
            ));
            right.borrow_mut().right = Some(RedBlackTreeNode::new(
                11,
                NodeColor::Black,
                Some(right.clone()),
            ));
            let right = right.borrow().right.clone();
            let right = right.unwrap();
            right.borrow_mut().left = Some(RedBlackTreeNode::new(
                10,
                NodeColor::Red,
                Some(right.clone()),
            ));
            right.borrow_mut().right = Some(RedBlackTreeNode::new(
                12,
                NodeColor::Red,
                Some(right.clone()),
            ));
        }

        assert!(tree.is_equal(&ans));
    }

    #[test]
    fn insert_and_search() {
        // Test searching through the tree for values.
        let mut tree = RedBlackTree::new();
        tree.insert(0);
        vec![8, -8, 4, 12, 10, 11].iter().for_each(|v| {
            tree.insert(*v);
        });
        // Did not find something in the tree
        vec![5, -6, -10, 13].iter().for_each(|v| {
            assert!(!tree.contains(*v));
        });
        // Found something in the tree
        vec![11, 12, -8, 0].iter().for_each(|v| {
            assert!(tree.contains(*v));
        })
    }

    #[test]
    fn insert_delete() {
        // Test the insert() and delete() method of the tree, verifying the
        // insertion，deletion of elements, and the balancing of the tree.
        let mut tree = RedBlackTree::new();
        tree.insert(0);
        vec![-12, 8, -8, 15, 4, 12, 10, 9, 11].iter().for_each(|v| {
            tree.insert(*v);
        });
        vec![15, -12, 9].iter().for_each(|v| {
            tree.delete(*v);
            // Did not find something in the tree
            assert!(!tree.contains(*v));
        });

        let root = tree.root.clone().unwrap();
        // fixed it
        assert!(RedBlackTreeNode::check_color_properties(root.clone()));
        let mut container = vec![];
        RedBlackTreeNode::inorder_traverse(root, &mut container);
        assert_eq!(container, vec![-8, 0, 4, 8, 10, 11, 12]);
    }

    #[test]
    fn tree_traversal() {
        // Test the three different tree traversal functions.
        let mut tree = RedBlackTree::new();
        tree.insert(0);
        vec![-16, 16, 8, 24, 20, 22].iter().for_each(|v| {
            tree.insert(*v);
        });
        let root = tree.root.clone().unwrap();

        let mut container = vec![];
        RedBlackTreeNode::inorder_traverse(root.clone(), &mut container);
        assert_eq!(container, vec![-16, 0, 8, 16, 20, 22, 24]);

        let mut container = vec![];
        RedBlackTreeNode::preorder_traverse(root.clone(), &mut container);
        assert_eq!(container, vec![0, -16, 16, 8, 22, 20, 24]);

        let mut container = vec![];
        RedBlackTreeNode::postorder_traverse(root, &mut container);
        assert_eq!(container, vec![-16, 8, 20, 24, 22, 16, 0]);
    }

    #[test]
    fn max() {
        // Test the get_max functions in the tree.
        let mut tree = RedBlackTree::new();
        tree.insert(0);
        vec![-16, 16, 8, 24, 20, 22].iter().for_each(|v| {
            tree.insert(*v);
        });
        let v_max = RedBlackTreeNode::get_max(tree.root.clone().unwrap());
        assert_eq!(v_max, 24)
    }

    #[test]
    fn insert_delete_inorder() {
        let mut tree = RedBlackTree::new();
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
        let mut tree = RedBlackTree::new();
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
        let mut tree = RedBlackTree::new();
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

