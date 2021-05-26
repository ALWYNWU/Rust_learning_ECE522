use rand::prelude::*;
use hamcrest2::prelude::*;

use super::*;
use integer_sqrt::IntegerSquareRoot;
use hamcrest2::core::assert_that;

#[test]
pub fn test_random_positive_square_root() {
    let mut rng = thread_rng();
    if rng.gen() { // random bool
        let x: f64 = rng.gen_range(10.0,100.0);
        assert_that!(calculator::get_square_root(x).powi(2)-x, lt(1e-6));
    }
}
#[test]
pub fn test_random_negitive_square_root() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen_range(10.0,100.0 );

    assert_that!((-calculator::get_square_root(x)).powi(2)-x,lt(1e-6));

}
#[test]
pub fn test_square_root_of_zero() {
    assert_that!(calculator::get_square_root(0.0), eq(0.0));
}

#[test]
pub fn test_square_root_of_one() {
    assert_that!(calculator::get_square_root(1.0), eq(1.0));
}
