# Trees

This is a course project for ECE 522, which implements Binary Search Tree, Red-black Tree, and AVL Tree.

Promotional video [HERE](https://www.dropbox.com/s/u485c73z3vww7b0/ece522v2.mp4?dl=0)

## Quick Start

```rust
use trees::prelude::*;

let avl = AVLTree::new(2);
avl.insert(0);
avl.delete(0);

let bst = BinarySearchTree::new();
bst.insert(0);
bst.delete(0);

let rbt = RedBlackTree::new(2);
rbt.insert(0);
rbt.delete(0);

// you can query the tree using methods like: 
// - is_empty
// - contains
// - height
// - min/max
// - ...
println!("{:?}", bst.max());
println!("height: {}", bst.height());
println!("is_empty: {}", bst.is_empty());
println!("count_leaves: {}", bst.count_leaves());
println!("min: {}", bst.min().unwrap());
println!("max: {}", bst.max().unwrap());
println!("contains 1: {}", bst.contains(1));
println!("contains 10: {}", bst.contains(10));
print!("print_inorder: ");
bst.print_inorder();
```

## Documentation

Building the documentation using

```
$ cargo doc
```

then you can find the documentation in `./target/doc/trees/index.html`,  

## Benchmarks

Run the benchmarks

```
$ cargo bench
```

then you can find the bench results in `./target/criterion/Compare/report/index.html`

To plot pretty figures, use the script `./benches/plot_benches.py`

```
$ cd benches
$ python plot_benches.py
```

Then you can find the figures in `./target/criterion`
