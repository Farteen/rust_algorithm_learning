#[test]
fn test_advanced_trait() {
    assert_eq!(Point {x: 1, y: 0} + Point{x: 2, y: 2}, Point{ x: 3, y: 2});
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

