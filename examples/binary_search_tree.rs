
pub use trees::prelude::{BinarySearchTree, QueryableTree};

pub fn main() {
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
}
