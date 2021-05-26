use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use rayon::prelude::*;

struct Person {
    age: u32,
}

fn age_average() {
    let mut v: Vec<Person> = Vec::new(); for i in 1..1000000 {
        v.push(Person { age: i });
    }
    let num_over_30 = v.iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = v.iter().map(|x| x.age).filter(|&x| x > 30).sum();
    let avg_over_30 = sum_over_30 as f32/ num_over_30;
    //println!("The average age of people older than 30 is {}", avg_over_30);
}

fn age_average_par() {
    let mut v: Vec<Person> = Vec::new(); for i in 1..1000000 {
        v.push(Person { age: i });
    }
    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();
    let avg_over_30 = sum_over_30 as f32/ num_over_30;
    //println!("The average age of people older than 30 is {}", avg_over_30);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    //let mut l: Vec<i64> = (0..10000).map(|_| {rng.gen_range(1, 10000)}).collect();
    //c.bench_function("age_average_par: ", |b| b.iter(|| age_average_par()));
    c.bench_function("age_average_par: ", |b| b.iter(|| age_average_par()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);