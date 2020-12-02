
pub use trees::prelude::{QueryableTree, RedBlackTree};

pub fn main() {
    println!("============== Red-black Tree ==============");
    let mut rbt = RedBlackTree::new();
    rbt.insert(2);
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
