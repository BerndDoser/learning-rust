use std::{ops::Mul, marker::Copy};

pub fn square_i32(num: i32) -> i32 {
    num * num
}

pub fn square_f32(num: f32) -> f32 {
    num * num
}

pub fn square<T>(num: T) -> T where T: Mul<Output = T> + Copy {
    num * num
}

use float_cmp::assert_approx_eq;

#[test]
fn test_square() {
    assert_eq!(square(5), 25);
    assert_eq!(square(-4), 16);
    assert_approx_eq!(f32, square(2.7), 7.29, ulps = 2);
}

pub(crate) fn main()
{
    println!("hey");
    let a = square_i32(10);
    println!("{}", a);

    let b: f32 = 2.5;
    let c = square_f32(b);
    println!("{}", c);

    let d = 1.5;
    let e = square(d);
    println!("{}", e);
}
