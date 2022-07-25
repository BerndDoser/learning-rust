pub struct Point<T>
{
    pub x: T,
    pub y: T,
}

pub fn main()
{
    let a = 4.5;
    let b = 3.2;
    if a > b { println!("a > b"); }

    let p1 = Point{x: 1.2, y:-2.3};
    let p2 = Point{x: 1.2, y:2.3};

    if p1 > p2 { println!("p1 > p2"); }

    //println!("{}", a);
}
