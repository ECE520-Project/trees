use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use trees::bstree::BinarySearchTree;
use trees::base::QueryableTree;
use trees::avltree::AVLTree;
use trees::rbtree::RedBlackTree;
use rand::{rngs::StdRng, SeedableRng};
use rand::seq::{SliceRandom, IteratorRandom};


const TREE_SIZE: [i32; 5] = [10_000, 40_000, 70_000, 100_000, 130_000];
// const TREE_SIZE: [i32; 5] = [100, 400, 700, 1000, 1300];


fn benchmark_bst(tree_size: i32) {
    let mut bst = BinarySearchTree::new();
    for v in 0..tree_size {
        bst.insert(v);
    }
    for v in 0..tree_size / 10 {
        bst.contains(v);
    }
}

fn benchmark_bst_insert_delete(tree_size: i32) {
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut data: Vec<i32> = (0..tree_size).collect();
    data.shuffle(&mut rng);
    let sample = data.iter().choose_multiple(&mut rng, (tree_size / 10) as usize);

    let mut bst = BinarySearchTree::new();
    for v in &data {
        bst.insert(*v);
    }
    for v in sample.iter() {
        bst.delete(**v);
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

fn benchmark_avl_insert_delete(tree_size: i32) {
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut data: Vec<i32> = (0..tree_size).collect();
    data.shuffle(&mut rng);
    let sample = data.iter().choose_multiple(&mut rng, (tree_size / 10) as usize);

    let mut avl = AVLTree::new();
    for v in &data {
        avl.insert(*v);
    }
    for v in sample.iter() {
        avl.delete(**v);
    }
}

fn benchmark_rbt(tree_size: i32) {
    let mut rbt = RedBlackTree::new();
    for v in 0..tree_size {
        rbt.insert(v);
    }
    for v in 0..tree_size / 10 {
        rbt.contains(v);
    }
}

fn benchmark_rbt_insert_delete(tree_size: i32) {
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut data: Vec<i32> = (0..tree_size).collect();
    data.shuffle(&mut rng);
    let sample = data.iter().choose_multiple(&mut rng, (tree_size / 10) as usize);

    let mut rbt = RedBlackTree::new();
    for v in &data {
        rbt.insert(*v);
    }

    for v in sample.iter() {
        rbt.delete(**v);
    }
}

fn bench_compare_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare_10Sample");
    group.sample_size(10);
    for (idx, size) in TREE_SIZE.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("BST", idx), size,
            |b, i| b.iter(|| benchmark_bst(*i))
        );
        group.bench_with_input(
            BenchmarkId::new("AVL", idx), size,
            |b, i| b.iter(|| benchmark_avl(*i))
        );
        group.bench_with_input(
            BenchmarkId::new("RBT", idx), size,
            |b, i| {
                b.iter(|| benchmark_rbt(*i));
            }
        );
    }
    group.finish();
}

fn bench_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare");
    for (idx, size) in TREE_SIZE.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("AVL", idx), size,
            |b, i| b.iter(|| benchmark_avl(*i))
        );
        group.bench_with_input(
            BenchmarkId::new("RBT", idx), size,
            |b, i| {
                b.iter(|| benchmark_rbt(*i));
            }
        );
    }
    group.finish();
}

fn bench_compare_insert_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare_insert_delete");
    for (idx, size) in TREE_SIZE.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("BST", idx), size,
            |b, i| b.iter(|| benchmark_bst_insert_delete(*i))
        );
        group.bench_with_input(
            BenchmarkId::new("AVL", idx), size,
            |b, i| b.iter(|| benchmark_avl_insert_delete(*i))
        );
        group.bench_with_input(
            BenchmarkId::new("RBT", idx), size,
            |b, i| {
                b.iter(|| benchmark_rbt_insert_delete(*i));
            }
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_compare_all,
    bench_compare,
    bench_compare_insert_delete,
);
criterion_main!(benches);
