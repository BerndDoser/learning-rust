struct Point<T>
{
    pub x: T,
    pub y: T,
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[test]
fn test()
{
    let p1 = Point{x: 1.2, y:2.3};
    let p2 = Point{x: 1.2, y:2.3};

    assert_eq!(p1 == p2, true);
    assert_eq!(p1 != p2, false);
}
