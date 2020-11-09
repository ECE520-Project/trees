use trees::bstree::BinarySearchTree;
use trees::rbtree::RedBlackTree;
use trees::avltree::AVLTree;
use trees::base::QueryableTree;

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
