#[derive(PartialEq, PartialOrd)]
pub struct Point<T>
{
    pub x: T,
    pub y: T,
}

#[test]
fn test()
{
    let a = 4.5;
    let b = 3.2;
    assert!(a > b);

    let p1 = Point{x: 1.2, y:-2.3};
    let p2 = Point{x: 1.2, y:2.3};

    assert!(p1 != p2);
    assert!(p1 <  p2);
}
