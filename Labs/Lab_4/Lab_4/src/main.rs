#![feature(core_intrinsics)]
use std::intrinsics::sqrtf64;
use std::io;

pub struct Point {
    x: i8,
    y: i8,
}

pub fn compute_euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    let a = ((p1.x as f64 - p2.x as f64) * (p1.x as f64 - p2.x as f64));
    let b = (p1.y as f64 - p2.y as f64) * (p1.y as f64 - p2.y as f64);
    let result = unsafe { sqrtf64(a + b) };
    result
}

pub fn compute_chebyshev_distance(p1: &Point, p2: &Point) -> i32 {
    let mut candidate_1 = p1.x as i32 - p2.x as i32;
    let mut candidate_2 = p1.y as i32 - p2.y as i32;
    let mut result:i32 = 0;

    if candidate_1 < 0 {
        candidate_1 = -candidate_1;
    }
    if candidate_2 < 0 {
        candidate_2 = -candidate_2;
    }

    if candidate_1 > candidate_2 {
        result = candidate_1
    }
    else {
        result = candidate_2
    }

    result
}

pub fn compute_manhattan_distanceC(p1: &Point, p2: &Point) -> i32 { unsafe
    {
        let a_abs = abs(p2.x as i32 - p1.x as i32);
        let b_abs = abs(p2.y as i32 - p1.y as i32);
        (a_abs + b_abs)
    } }

pub fn compute_chebyshev_distance_C(p1: &Point, p2: &Point) -> i32 {
    unsafe {
        let mut candidate_1 = abs(p1.x as i32 - p2.x as i32);
        let mut candidate_2 = abs(p1.y as i32 - p2.y as i32);
        let mut result:i32 = 0;

        if candidate_1 > candidate_2 {
            result = candidate_1
        }
        else {
            result = candidate_2
        }
        return result
    }
}

extern {
    pub fn abs(i: i32) -> i32;
}

fn main() {
    println!("Please input first point");
    println!("Please input x of first point: ");
    let mut x_1 = String::new();
    io::stdin().read_line(&mut x_1);
    let x_1:i8 = x_1.trim().parse().expect("Please type a number!");

    println!("Please input y of first point: ");
    let mut y_1 = String::new();
    io::stdin().read_line(&mut y_1);
    let y_1:i8 = y_1.trim().parse().expect("Please type a number!");

    let point_1 = Point {x:x_1, y:y_1};

    println!("Please input second point");
    println!("Please input x of second point: ");
    let mut x_2 = String::new();
    io::stdin().read_line(&mut x_2);
    let x_2:i8 = x_2.trim().parse().expect("Please type a number!");

    println!("Please input y of second point: ");
    let mut y_2 = String::new();
    io::stdin().read_line(&mut y_2);
    let y_2:i8 = y_2.trim().parse().expect("Please type a number!");

    let point_2 = Point {x:x_2, y:y_2};

    println!("Please input the kind of distance: ");
    let mut kind = String::new();
    io::stdin().read_line(&mut kind);
    let kind = kind.trim();

    if kind == "euclidean" {
        println!("The result is: {}",compute_euclidean_distance(&point_1,&point_2));
    }
    else if kind == "chebyshev" {
        println!("The result is: {}",compute_chebyshev_distance_C(&point_1,&point_2));
    }
    else if kind == "manhattan" {
        println!("The result is: {}",compute_manhattan_distanceC(&point_1,&point_2));
    }
    else {
        println!("Please input right instruction")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_distance() {
        let point_1 = Point {x:1, y:3 };
        let point_2 = Point {x:2, y:3 };
        assert_eq!(compute_euclidean_distance(&point_1,&point_2), 1.0);
        assert_eq!(compute_chebyshev_distance(&point_1,&point_2), 1);
    }
}
