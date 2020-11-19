use trees::bstree::BinarySearchTree;
use trees::rbtree::RedBlackTree;
use trees::avltree::AVLTree;
use trees::base::QueryableTree;

fn main() {
    println!("============== Binary Search Tree ==============");
    let mut bst = BinarySearchTree::new();
    bst.insert(1);
    bst.insert(0);
    bst.insert(2);
    bst.insert(3);
    bst.insert(5);
    println!("height: {}", bst.height());
    println!("is_empty: {}", bst.is_empty());
    println!("count_leaves: {}", bst.count_leaves());
    println!("min: {}", bst.min().unwrap());
    println!("max: {}", bst.max().unwrap());
    println!("contains 1: {}", bst.contains(1));
    println!("contains 10: {}", bst.contains(10));
    print!("print_inorder: ");
    bst.print_inorder();

    println!("============== AVL Tree ==============");
    let avl = AVLTree::new(2);
    println!("height: {}", avl.height());
    println!("is_empty: {}", avl.is_empty());
    println!("count_leaves: {}", avl.count_leaves());
    println!("min: {}", avl.min().unwrap());
    println!("max: {}", avl.max().unwrap());
    println!("contains 2: {}", avl.contains(2));
    println!("contains 10: {}", avl.contains(10));
    print!("print_inorder: ");
    avl.print_inorder();

    println!("============== Red-black Tree ==============");
    let rbt = RedBlackTree::new(2);
    println!("height: {}", rbt.height());
    println!("is_empty: {}", rbt.is_empty());
    println!("count_leaves: {}", rbt.count_leaves());
    println!("min: {}", rbt.min().unwrap());
    println!("max: {}", rbt.max().unwrap());
    println!("contains 2: {}", rbt.contains(2));
    println!("contains 10: {}", rbt.contains(0));
    print!("print_inorder: ");
    rbt.print_inorder();
}


// use std::cell::RefCell;
// use std::rc::Rc;
//
// type RcRefBaseNode<T> = Rc<RefCell<BinarySearchTreeNode<T>>>;
// type BaseNodeLink<T> = Option<RcRefBaseNode<T>>;
//
// pub struct BinarySearchTreeNode<T: Ord> {
//     pub data: T,
//     left: BaseNodeLink<T>,
//     right: BaseNodeLink<T>,
// }
//
// impl <T: Ord> BinarySearchTreeNode<T> {
//     fn min(&self) -> &T {
//         self.left.as_ref().map_or(&self.data, |x| x.borrow_mut().min())
//     }
// }
//
// fn main() {
//
// }
