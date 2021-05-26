use rayon::prelude::*;
fn concurrent_quick_sort(v: &mut [usize]) {
//add your code here:
//uses partition fn, multiple options exist. //use rayon for concurrency.
    if v.len() > 1 {
        let mut mid = partition(v);
        if mid < v.len() / 2 {  // to avoid endless loop
            mid += 1;
        }
        let (left, right) = v.split_at_mut(mid);
        rayon::join(|| concurrent_quick_sort(left),
                    || concurrent_quick_sort(right));
    }
}


fn partition(v: &mut [usize]) -> usize {

    let l = v.len();
    let mut mid = vec![0; l];
    let pivot_element = v[0];

    let mut less_num = vec![0; l];
    let mut greater_num = vec![0; l];
    let mut eq_num = vec![0; l];

    for i in 0..l{
        mid[i] = v[i];
        if mid[i] < pivot_element {
            less_num[i] = 1 as usize;
        }
        else if mid[i] > pivot_element {
            greater_num[i] = 1 as usize;
        }
        else if mid[i] == pivot_element {
            eq_num[i] = 1 as usize;
        }
    }

    less_num = par_prefix_sum(&mut less_num);
    greater_num = par_prefix_sum(&mut greater_num);
    eq_num = par_prefix_sum(&mut eq_num);

    for i in 0..l{
        if mid[i] < pivot_element {
            v[less_num[i]-1] = mid[i];
        }
        else if mid[i] > pivot_element{
            v[less_num[l-1]+eq_num[l-1]+greater_num[i]-1] = mid[i];
        }
        else if mid[i] == pivot_element {
            v[less_num[l-1]+eq_num[l-1] - 1] = mid[i];
        }
    }

    return less_num[l-1] as usize
}

fn par_prefix_sum(v: &mut [usize]) -> Vec<usize> {
    let mut v_p = Vec::new();
    for i in 0..v.len() {
        let mut sum = 0;
        for j in 0..i+1 {
            sum = sum + v[j];
        }
        v_p.push(sum);
    }
    v_p
}

fn main() {
    let mut a = vec![9,3,10,12,34,56,3,4,2,5,7];

    concurrent_quick_sort(&mut a);

    println!("{:?}",a);

}