use std::io;

fn main() {
    let income = input();
    println!("The taxed income is: {}",tax(income));
}

fn input() -> i64 {
    println!("Please input your income");
    let mut a = String::new();

    io::stdin().read_line(&mut a)
        .expect("Failed to read line");

    let a:i64 = a.trim().parse()
        .expect("Please type a positive integer");

    if a < 0{
        panic!("Negative number");
    }

    a
}

fn tax(income:i64) ->i64 {
    let mut tax = 0;
    if income < 10000 {
        tax = 0;
    }
    else if (income>=10000) && (income<50000) {
        tax = (income-10000)/10;
    }
    else if (income>=50000) && (income<100000) {
        tax = 4000+(tax-50000)/5;
    }
    else if (income>=100000) && (income<1000000) {
        tax = 4000+10000+(income-100000)*3/10;
    }
    else if income>=1000000 {
        tax = 4000+10000+270000+(income-1000000)*4/10;
    }
    income - tax
}
#[cfg(test)]
mod Test;