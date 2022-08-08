mod blas;
mod point;
mod point_compare;
mod square;

pub(crate) fn main() {
    println!("hey");
    let a = square::square_i32(10);
    println!("{}", a);

    let b: f32 = 2.5;
    let c = square::square_f32(b);
    println!("{}", c);

    let d = 1.5;
    let e = square::square(d);
    println!("{}", e);
}
