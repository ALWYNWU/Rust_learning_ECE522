use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ECE522_Project::AVL_tree::AvlTree;
use ECE522_Project::red_black_tree::*;
use ECE522_Project::BST::*;



fn avl_insert_and_search(n:i32) {
    let mut tree = AvlTree::new();

    for i in 0..n {
        tree.Insert(i);
    }
    let end = n/10;
    for j in 0..end {
        tree.Search(&j);
    }
}

fn rb_insert_and_search(n: i32) {
    let mut tree: RBTree<i32> = RBTree::new();
    for i in 0..n {
        tree.insert(i);
    }
    let end = n/10;
    for j in 0..end {
        tree.search_node(j);
    }
}

fn bst_insert_and_search(n: i32) {
    let mut tree: BSTree<i32> = BSTree::new_empty();
    for i in 0..n {
        tree.insert(i);
    }
    let end = n/10;
    for j in 0..end {
        tree.search(j);
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert 10000 elements and search for the 1000 lowest elements in the Red-Black Tree", |b| {
        b.iter(|| rb_insert_and_search(black_box(10000)))
    });
    // c.bench_function("insert 40000 elements and search for the 4000 lowest elements in the Red-Black Tree", |b| {
    //     b.iter(|| rb_insert_and_search(black_box(40000)))
    // });
    // c.bench_function("insert 70000 elements and search for the 7000 lowest elements in the Red-Black Tree", |b| {
    //     b.iter(|| rb_insert_and_search(black_box(70000)))
    // });
    // c.bench_function("insert 100000 elements and search for the 10000 lowest elements in the Red-Black Tree", |b| {
    //     b.iter(|| rb_insert_and_search(black_box(100000)))
    // });
    // c.bench_function("insert 130000 elements and search for the 13000 lowest elements in the Red-Black Tree", |b| {
    //     b.iter(|| rb_insert_and_search(black_box(130000)))
    // });

    c.bench_function("insert 10000 elements and search for the 1000 lowest elements in the AvlTree", |b| {
        b.iter(|| avl_insert_and_search(black_box(10000)))
    });
    // c.bench_function("insert 40000 elements and search for the 4000 lowest elements in the AvlTree", |b| {
    //     b.iter(|| avl_insert_and_search(black_box(40000)))
    // });
    // c.bench_function("insert 70000 elements and search for the 7000 lowest elements in the AvlTree", |b| {
    //     b.iter(|| avl_insert_and_search(black_box(70000)))
    // });
    // c.bench_function("insert 100000 elements and search for the 10000 lowest elements in the AvlTree", |b| {
    //     b.iter(|| avl_insert_and_search(black_box(100000)))
    // });
    // c.bench_function("insert 130000 elements and search for the 13000 lowest elements in the AvlTree", |b| {
    //     b.iter(|| avl_insert_and_search(black_box(130000)))
    // });

    c.bench_function("insert 10000 elements and search for the 1000 lowest elements in the Binary Search tree", |b| {
        b.iter(|| bst_insert_and_search(black_box(10000)))
    });
    // c.bench_function("insert 40000 elements and search for the 4000 lowest elements in the Binary Search tree", |b| {
    //     b.iter(|| bst_insert_and_search(black_box(40000)))
    // });
    // c.bench_function("insert 70000 elements and search for the 7000 lowest elements in the Binary Search tree", |b| {
    //     b.iter(|| bst_insert_and_search(black_box(70000)))
    // });
    // c.bench_function("insert 100000 elements and search for the 10000 lowest elements in the Binary Search tree", |b| {
    //     b.iter(|| bst_insert_and_search(black_box(100000)))
    // });
    // c.bench_function("insert 130000 elements and search for the 13000 lowest elements in the Binary Search tree", |b| {
    //     b.iter(|| bst_insert_and_search(black_box(130000)))
    // });

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);