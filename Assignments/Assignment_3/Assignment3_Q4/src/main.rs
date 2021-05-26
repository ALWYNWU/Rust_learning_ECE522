use std::io;
use factorial::Factorial;

fn main() {
    let number = input();
    factorial(number.0,number.1);
}

fn factorial(a:u32,b:u32) {

    if a<b {
        panic!("Wrong Number!");
    }
    else {
        let mut a_f = a.factorial();
        let mut b_f = b.factorial();
        let mut i = a-b;
        let mut i_f = i.factorial();
        println!("The result is {}",a_f/(i_f*b_f));
    }
}

fn input() ->(u32,u32) {
    println!("Please input a");
    let mut a = String::new();

    io::stdin().read_line(&mut a)
        .expect("Failed to read line");

    let a:u32 = a.trim().parse()
        .expect("Please type a number!");

    println!("Please input b");
    let mut b = String::new();

    io::stdin().read_line(&mut b)
        .expect("Failed to read line");

    let b:u32 = b.trim().parse()
        .expect("Please type a number!");

    (a,b)
}

#[cfg(test)]
mod test;
