use std::i32;

fn main() {
    let mut sum = 0;
    let mut start = 0;
    let mut end = 0;
    let mut X = 0;
    let mut Y = 0;
    let mut prime_list: Vec<i32> = Vec::new();

    for x in 2..1000 {
        if is_prime(x) == true {
            prime_list.push(x);//find all the prime numbers less than 1000
        }                            // store them in vector
    }
    let l = prime_list.len();

    for i in 0..l {
        sum = prime_list[i];
        for j in i+1..l {
            sum += prime_list[j];
            if sum > 1000 {
                break;
            }
            if is_prime(sum) {//if the sum is a prime number
                if X < j-i {// find the longest sequence
                    X = j-i+1;//that means the number of the prime number
                    Y = sum;//the sum of the prime number
                    start = i;
                    end = j;
                }
            }
        }
    }
    println!("The X is: {}", X);
    println!("The Y is: {}", Y);
    println!("The list is: ");
    for i in start..end+1 { //add 1 can iterate to the last number
        println!("{}",prime_list[i]);
    }
}

fn is_prime(p:i32) -> bool {
    let p1 = p;
    let mut p2 = 2;
    for x in 2..p1 {
        if p1%x == 0 {
            return false;
        }
    }
    return true;
}
