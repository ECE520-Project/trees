use criterion::{black_box, criterion_group, criterion_main, Criterion};
use trees::bstree::BinarySearchTree;
use trees::base::QueryableTree;


fn benchmark_bst(tree_size: i32) {
    let mut bst = BinarySearchTree::new();
    for v in 0..tree_size {
        bst.insert(v);
    }
    for v in 0..tree_size / 10 {
        bst.contains(v);
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    let tree_sizes = [10_000, 40_000, 70_000, 100_000, 130_000];
    for size in &tree_sizes {
        c.bench_function(
            "Binary Tree",
            |b| b.iter(|| benchmark_bst(black_box(*size)))
        );
        benchmark_bst(*size)
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);