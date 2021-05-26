use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;


fn selection_sort(array: &mut [i64]) {

    let mut min;
    let length = array.len();
    for i in 0..length {
        min = i;
        for j in (i+1)..length {
            if array[j] < array[min] {
                min = j;
            }
        }
        array.swap(min, i);
    }
}

fn selection_sort_optim(array: &mut[i64]) {
    for i in 0..array.len() {
        if let Some((j,_)) = array.iter()
            .enumerate().skip(i).min_by_key(|x| x.1) { array.swap(i,j);}
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut l: Vec<i64> = (0..10000).map(|_| {rng.gen_range(1, 10000)}).collect();
    //c.bench_function("selection_sort: ", |b| b.iter(|| selection_sort(black_box(&mut l))));
    c.bench_function("selection_sort_optim: ", |b| b.iter(|| selection_sort_optim(black_box(&mut l))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);