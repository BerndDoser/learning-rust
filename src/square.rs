use std::{marker::Copy, ops::Mul};

pub fn square_i32(num: i32) -> i32 {
    num * num
}

pub fn square_f32(num: f32) -> f32 {
    num * num
}

pub fn square<T>(num: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    num * num
}
