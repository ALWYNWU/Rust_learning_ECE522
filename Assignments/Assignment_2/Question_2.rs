#![feature(core_intrinsics)]
use std::fmt::Display;
use std::mem;
use std::intrinsics::size_of;

struct  Bag<T> {
    items: [T; 3],
}

impl<T> Bag<T>{
    fn BagSize(&self) {
        let mut byte_nums = 0;
        for element in self.items.iter() {
            byte_nums = byte_nums + mem::size_of_val(element);
        }
        println!("{}",byte_nums);
    }
}

fn Size<T>(stru:&Bag<T>) -> usize {
    let mut byte_nums = 0;
    for element in stru.items.iter() {
        byte_nums = byte_nums + mem::size_of_val(element);
    }
    byte_nums

}


fn main() {
    let b1 = Bag {items: [1u8,2u8,3u8],};
    let b2 = Bag {items: [1u32,2u32,3u32],};
    println!("size of First Bag = {} bytes",Size(&b1));
    println!("size of Second Bag = {} bytes",Size(&b2));
}