use criterion::black_box;
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

fn main() {

}