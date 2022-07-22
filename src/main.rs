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
