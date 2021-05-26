use crate::calculator::get_roots;
use rand::prelude::*;
use hamcrest2::prelude::*;

use super::*;
use hamcrest2::core::assert_that;

#[test]
pub fn test_basic_roots() {
   assert_that!(get_roots(1.0,-1.0,-20.0),eq((true,5.0,-4.0)));
}
#[test]
pub fn test_single_root() {
   assert_that!(get_roots(3.0,6.0,3.0),eq((true,-1.0,-1.0)));
	 
}
#[test]
pub fn test_random_solvable_quadratic() {
   let mut rng = thread_rng();

   let a: f64 = rng.gen_range(5.0,10.0);
   let b: f64 = rng.gen_range(20.0,100.0);
   let c: f64 = rng.gen_range(5.0,10.0);
   let result = get_roots(a,b,c);

   assert_that!((a*(result.2.powi(2))+b*result.2+c)-0.0,lt(1e-6));
}

#[test]
pub fn test_random_non_solvable_quadratic() {
   let mut rng = thread_rng();

   let a: f64 = rng.gen_range(5.0,10.0);
   let b: f64 = rng.gen_range(1.0,9.0);
   let c: f64 = rng.gen_range(5.0,10.0);

   assert_that!(get_roots(a,b,c).0, eq(false));
	
}


   
