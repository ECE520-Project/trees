use trees::prelude::*;

fn main() {
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
}
