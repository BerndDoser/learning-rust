use std::{cmp::Ordering, ops::Add};

#[derive(Debug)]
struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: PartialOrd> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x
            .partial_cmp(&other.x)
            .and(self.y.partial_cmp(&other.y))
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    type Output = Self;
}

#[test]
fn test() {
    let p1 = Point { x: 1.2, y: 2.3 };
    let p2 = Point { x: 1.2, y: 2.3 };

    assert!(p1 == p2);
    assert!(!(p1 != p2));
    assert!(!(p1 < p2));
    assert!(!(p1 > p2));
    assert!(p1 <= p2);
    assert!(p1 <= p2);

    let p3 = p1 + p2;
    assert_eq!(p3, Point { x: 2.4, y: 4.6 });
}
