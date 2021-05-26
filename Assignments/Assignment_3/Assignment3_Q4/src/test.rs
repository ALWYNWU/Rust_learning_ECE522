use hamcrest2::prelude::*;
use super::*;
use hamcrest2::core::assert_that;

#[test]
#[should_panic]
pub fn test_1() {
    factorial(1,4);
}

#[test]
#[should_panic]
pub fn test_2() {
    input();
}
//When you test the program, just input a float number, then it will pass the teat