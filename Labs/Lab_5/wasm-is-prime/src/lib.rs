mod utils;

use wasm_bindgen::prelude::*;
use prime_tools::is_u32_prime;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn CheckPrime(s: &JsValue) {
    let mut input: String = s.as_string().unwrap();
    if prime(input) {
        alert("Input is Prime");
    }
    else{
        alert("Input is NOT Prime");
    }
}
pub fn prime(s: String)->bool {
    let mut num:u32 = s.trim().parse().expect("ERROR");
    if is_u32_prime(num) {
        return true
    }
    else {
      return false
    }
}