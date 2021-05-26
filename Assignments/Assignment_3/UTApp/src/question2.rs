use rand::prelude::*;
use hamcrest2::prelude::*;

use super::*;
use hamcrest2::core::assert_that;

#[test]
pub fn basic_multiply() {
    assert_that!(calculator::multiply(2.0, 3.0), eq(6.0));
}
#[test]
pub fn multiply_negative_number() {
    assert_that!(calculator::multiply(-1.0, 2.0), eq(-2.0));
}
#[test]
pub fn multiply_random_numbers() {
    let mut rng = thread_rng();
    if rng.gen() { // random bool
        let x: f64 = rng.gen(); // random number in range [0, 1)
        let y: f64 = rng.gen();
        assert_that!(calculator::multiply(x, y), eq(x*y));
    }
}

#[test]
pub fn basic_divide() {
    assert_that!(calculator::divide(4.0, 2.0), eq(2.0));
}
#[test]
pub fn divide_negative_number() {
    assert_that!(calculator::divide(-6.0, 2.0), eq(-3.0));
}
#[test]
pub fn divide_random_numbers() {
    let mut rng = thread_rng();
    if rng.gen() { // random bool
        let x: f64 = rng.gen(); // random number in range [0, 1)
        let y: f64 = rng.gen();
        assert_that!(calculator::divide(x, y), eq(x/y));
    }
}
   
