

use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use trees::bstree::BinarySearchTree;
use trees::base::QueryableTree;
use trees::avltree::AVLTree;
use trees::rbtree::RedBlackTree;


const SAMPLE_SIZE: usize = 10;
// const TREE_SIZE: [i32; 5] = [10_000, 40_000, 70_000, 100_000, 130_000];
const TREE_SIZE: [i32; 5] = [100, 400, 700, 1000, 1300];


fn benchmark_bst(tree_size: i32) {
    let mut bst = BinarySearchTree::new();
    for v in 0..tree_size {
        bst.insert(v);
    }
    for v in 0..tree_size / 10 {
        bst.contains(v);
    }
}

// fn criterion_benchmark_bst(c: &mut Criterion) {
//     let mut group = c.benchmark_group("BST");
//     group.sample_size(SAMPLE_SIZE);
//     for (idx, size) in TREE_SIZE.iter().enumerate() {
//         group.bench_function(
//             idx.to_string(),
//             |b| b.iter(|| benchmark_bst(black_box(*size)))
//         );
//     }
// }

fn benchmark_avl(tree_size: i32) {
    let mut avl = AVLTree::new();
    for v in 0..tree_size {
        avl.insert(v);
    }
    for v in 0..tree_size / 10 {
        avl.contains(v);
    }
}

// fn criterion_benchmark_avl(c: &mut Criterion) {
//     let mut group = c.benchmark_group("AVL");
//     group.sample_size(SAMPLE_SIZE);
//     for (idx, size) in TREE_SIZE.iter().enumerate() {
//         group.bench_function(
//             idx.to_string(),
//             |b| b.iter(|| benchmark_bst(black_box(*size)))
//         );
//     }
// }

fn benchmark_rbt(tree_size: i32) {
    let mut rbt = RedBlackTree::new();
    for v in 0..tree_size {
        rbt.insert(v);
    }
    for v in 0..tree_size / 10 {
        rbt.contains(v);
    }
}

// fn criterion_benchmark_rbt(c: &mut Criterion) {
//     let mut group = c.benchmark_group("RBT");
//     group.sample_size(SAMPLE_SIZE);
//     for (idx, size) in TREE_SIZE.iter().enumerate() {
//         group.bench_function(
//             idx.to_string(),
//             |b| b.iter(|| benchmark_bst(black_box(*size)))
//         );
//     }
// }

fn bench_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare");
    group.sample_size(SAMPLE_SIZE);
    for (idx, size) in TREE_SIZE.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("BST", idx), size,
            |b, i| b.iter(|| benchmark_bst(*i))
        );
        group.bench_with_input(
            BenchmarkId::new("AVL", idx), size,
            |b, i| b.iter(|| benchmark_avl(*i))
        );
        // FIXME: memory leak, uncomment following code to see the difference
        group.bench_with_input(
            BenchmarkId::new("RBT", idx), size,
            |b, i| b.iter(|| benchmark_rbt(*i))
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    // criterion_benchmark_bst,
    // criterion_benchmark_rbt,
    // criterion_benchmark_avl,
    bench_compare,
);
criterion_main!(benches);