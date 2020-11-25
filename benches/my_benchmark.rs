use criterion::{black_box, criterion_group, criterion_main, Criterion};
use trees::bstree::BinarySearchTree;
use trees::base::QueryableTree;
use trees::avltree::AVLTree;


fn benchmark_bst(tree_size: i32) {
    let mut bst = BinarySearchTree::new();
    for v in 0..tree_size {
        bst.insert(v);
    }
    for v in 0..tree_size / 10 {
        bst.contains(v);
    }
}

fn criterion_benchmark_bst(c: &mut Criterion) {
    let tree_sizes = [10_000, 40_000, 70_000, 100_000, 130_000];
    for size in &tree_sizes {
        c.bench_function(
            "Binary Tree",
            |b| b.iter(|| benchmark_bst(black_box(*size)))
        );
        benchmark_bst(*size)
    }
}

fn benchmark_avl(tree_size: i32) {
    let mut avl = AVLTree::new();
    for v in 0..tree_size {
        avl.insert(v);
    }
    for v in 0..tree_size / 10 {
        avl.contains(v);
    }
}

fn criterion_benchmark_avl(c: &mut Criterion) {
    let tree_sizes = [10_000, 40_000, 70_000, 100_000, 130_000];
    for size in &tree_sizes {
        c.bench_function(
            "AVL Tree",
            |b| b.iter(|| benchmark_avl(black_box(*size)))
        );
        benchmark_avl(*size)
    }
}

criterion_group!(benches, criterion_benchmark_bst, criterion_benchmark_avl);
criterion_main!(benches);