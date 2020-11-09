use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;

// ============== Base Tree ============== //
type RcRefBaseNode<T> = Rc<RefCell<BinarySearchTreeNode<T>>>;
type BaseNodeLink<T> = Option<RcRefBaseNode<T>>;

struct BinarySearchTreeNode<T: Ord> {
    pub data: T,
    left: BaseNodeLink<T>,
    right: BaseNodeLink<T>,
}

impl <T: Ord> QueryableTreeNode for BinarySearchTreeNode<T> {
    fn left(&self) -> &BaseNodeLink<T> { return &self.left; }
    fn right(&self) -> &BaseNodeLink<T> { return &self.right; }
}

impl <T: Ord> BinarySearchTreeNode<T> {
    fn new(data: T) -> BaseNodeLink<T> {
        Some(Rc::new(RefCell::new(Self{
            data,
            left: None,
            right: None
        })))
    }

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
}

struct BinarySearchTree<T: Ord> {root: BaseNodeLink<T>}

impl <T: Ord> QueryableTree<BinarySearchTreeNode<T>> for BinarySearchTree<T> {
    fn root(&self) -> &BaseNodeLink<T> {
        &self.root
    }
}

impl<T: Ord> BinarySearchTree<T> {
    fn new(data: T) -> Self {
        Self{
            root: Some(Rc::new(RefCell::new(BinarySearchTreeNode{
                data,
                left: None,
                right: None
            })))
        }
    }

    fn print_in_order(&self) {
        unimplemented!()
    }

    fn insert(&mut self, new_val: T) {
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
}

trait QueryableTreeNode {
    fn left(&self) -> &Option<Rc<RefCell<Self>>>;
    fn right(&self) -> &Option<Rc<RefCell<Self>>>;
    fn height(&self) -> usize {
        let left_height = self.left().as_ref().map(
            |l| l.borrow().height()
        ).unwrap_or(0);
        let right_height = self.right().as_ref().map(
            |r| r.borrow().height()
        ).unwrap_or(0);
        max(left_height, right_height) + 1
    }
    fn count_leaves(&self) -> usize {
        match self.left() {
            None => match self.right() {
                None => 1,
                Some(r) => r.borrow().count_leaves(),
            }
            Some(l) => match self.right() {
                None => l.borrow().count_leaves(),
                Some(r) => {
                    l.borrow().count_leaves() + r.borrow().count_leaves()
                }
            }
        }
    }
}

trait QueryableTree<QTN: QueryableTreeNode> {
    fn root(&self) -> &Option<Rc<RefCell<QTN>>>;

    fn height(&self) -> usize {
        match &self.root() {
            None => 0,
            Some(node) => node.borrow().height(),
        }
    }

    fn is_empty(&self) -> bool {
        match self.root() {
            None => true,
            Some(_) => false
        }
    }

    fn count_leaves(&self) -> usize {
        match self.root() {
            None => 0,
            Some(node) => node.borrow().count_leaves(),
        }
    }
}

// ============== Red-Black Tree ============== //
type RcRefRBTNode<T> = Rc<RefCell<RedBlackTreeNode<T>>>;
type RBNodeLink<T> = Option<RcRefRBTNode<T>>;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

struct RedBlackTreeNode<T> {
    pub key: T,
    pub color: NodeColor,
    parent: RBNodeLink<T>,
    left: RBNodeLink<T>,
    right: RBNodeLink<T>,
}

struct RedBlackTree<T: Ord> {root: RBNodeLink<T>}

impl <T: Ord> QueryableTreeNode for RedBlackTreeNode<T> {
    fn left(&self) -> &RBNodeLink<T> { return &self.left; }
    fn right(&self) -> &RBNodeLink<T> { return &self.right; }
}

impl <T: Ord> QueryableTree<RedBlackTreeNode<T>> for RedBlackTree<T> {
    fn root(&self) -> &RBNodeLink<T> {
        &self.root
    }
}

impl<T: Ord> RedBlackTree<T> {
    fn new(data: T) -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(RedBlackTreeNode {
                key: data,
                color: NodeColor::Red,
                parent: None,
                left: None,
                right: None
            })))
        }
    }
}

// ============== AVL Tree ============== //
type RcRefAVLTNode<T> = Rc<RefCell<AVLTreeNode<T>>>;
type AVLNodeLink<T> = Option<RcRefAVLTNode<T>>;

struct AVLTreeNode<T> {
    pub key: T,
    parent: AVLNodeLink<T>,
    left: AVLNodeLink<T>,
    right: AVLNodeLink<T>,
    height: usize,
}

struct AVLTree<T: Ord> {root: AVLNodeLink<T>}

impl <T: Ord> QueryableTreeNode for AVLTreeNode<T> {
    fn left(&self) -> &AVLNodeLink<T> { return &self.left; }
    fn right(&self) -> &AVLNodeLink<T> { return &self.right; }
}

impl <T: Ord> QueryableTree<AVLTreeNode<T>> for AVLTree<T> {
    fn root(&self) -> &AVLNodeLink<T> {
        &self.root
    }
}

impl<T: Ord> AVLTree<T> {
    fn new(data: T) -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(AVLTreeNode {
                key: data,
                parent: None,
                left: None,
                right: None,
                height: 0
            })))
        }
    }
}

fn main() {
    let mut bst = BinarySearchTree::new(1);
    bst.insert(0);
    bst.insert(2);
    bst.insert(3);
    bst.insert(5);
    println!("{}", bst.height());
    println!("{}", bst.is_empty());
    println!("{}", bst.count_leaves());
    // println!("{:?}", bst);

    let avl = AVLTree::new(2);
    println!("{}", avl.height());
    println!("{}", avl.is_empty());

    let rbt = RedBlackTree::new(2);
    println!("{}", rbt.height());
    println!("{}", rbt.is_empty());
}
