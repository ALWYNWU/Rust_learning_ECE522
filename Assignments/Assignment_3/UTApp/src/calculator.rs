use integer_sqrt::IntegerSquareRoot;
use hamcrest2::prelude::*;

pub fn add(a:f64, b:f64) -> f64 {
    a + b
}
pub fn subtract(a:f64,b:f64) -> f64 {
    a - b
}

pub fn multiply(a:f64,b:f64) -> f64 {
    a * b
}
pub fn divide(a:f64,b:f64) -> f64 {
    a / b
}
pub fn get_square_root(a:f64) -> f64 {
    a.sqrt()
}

pub fn get_roots(a:f64,b:f64,c:f64) ->(bool,f64,f64) {//quadratic function: aX*X + bX + C = 0
    let delta = b*b - 4.0*a*c;
    let mut bool:bool = true;
    let mut solution_1:f64 = 0.0;
    let mut solution_2:f64 = 0.0;
    if delta < 0.0 {
        solution_1 = 0.0;
        solution_2 = 0.0;
        bool = false;
    }
    else if delta == 0.0 {
        solution_1 = -b/(2.0*a);
        solution_2 = -b/(2.0*a);
        bool = true;
    }
    else if delta > 0.0 {
        solution_1 = (-b+delta.sqrt())/(2.0*a);
        solution_2 = (-b-delta.sqrt())/(2.0*a);
        bool = true;
    }
    (bool,solution_1,solution_2)
}
