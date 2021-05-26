use rug::{Assign,Integer};
use rand::Rng;
use is_prime::*;

fn main() {
    println!("{}",function(34534624563546));
}

fn function(n:u128) -> Integer { //define function
    let mut rng = rand::thread_rng();//call the random number generator
    loop {
        let mut candidate = Integer::new();//creat candidate
        candidate.assign(rng.gen_range(0,&n));

        let s:String = candidate.to_string();

        if is_prime(&s) == true {
            return candidate;
        }
    }
}


