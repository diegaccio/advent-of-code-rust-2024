use std::ops::{Add, AddAssign, Mul, Sub};

//clever use of point and direction by maneatingape
pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
// Left to right and top to bottom.
pub const DIAGONAL: [Point; 8] = [
    Point::new(-1, -1),
    UP,
    Point::new(1, -1),
    LEFT,
    RIGHT,
    Point::new(-1, 1),
    DOWN,
    Point::new(1, 1),
];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    #[must_use]
    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_grid() {
        let mut a = Point::new(1, 2);
        let b = Point::new(3, 4);
        //let k = 2;

        assert_eq!(a + b, Point::new(4, 6));

        a += b;
        assert_eq!(a, Point::new(4, 6));
        //assert_eq!(a - b, Point::new(-2, -2));
        //assert_eq!(a * k, Point::new(2, 4));
    }
}
