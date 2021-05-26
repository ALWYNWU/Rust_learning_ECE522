use rand::prelude::*;
use hamcrest2::prelude::*;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;
use hamcrest2::core::assert_that;

#[test]
pub fn basic_add() {
    assert_that!(calculator::add(1.0, 2.0), eq(3.0));
}
#[test]
pub fn add_negative_number() {
	 assert_that!(calculator::add(-1.0, 2.0), eq(1.0));
}
#[test]
pub fn add_random_numbers() {
	let mut rng = thread_rng();
	if rng.gen() { // random bool
        let x: f64 = rng.gen(); // random number in range [0, 1)
        let y: f64 = rng.gen();
        assert_that!(calculator::add(x, y), eq(x+y));
	}
}

#[test]
pub fn basic_subtract() {
	assert_that!(calculator::subtract(4.0, 2.0), eq(2.0));
}
#[test]
pub fn subtract_negative_number() {
    assert_that!(calculator::subtract(-3.0, 2.0), eq(-5.0));
}
#[test]
pub fn subtract_random_numbers() {
    let mut rng = thread_rng();
    if rng.gen() { // random bool
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let y: f64 = rng.gen();
    assert_that!(calculator::subtract(x, y), eq(x-y));
    }
}
