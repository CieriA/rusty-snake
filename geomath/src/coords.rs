use std::{ops::{Add, AddAssign}, fmt::{Display, Formatter}};
use crate::Direction;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    #[inline]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    
    /// shift and return (does not change self)
    pub fn shift(&self, direction: Direction) -> Point {
        let offset: Point = match direction {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }.into();
        
        *self + offset
    }
}

impl From<(isize, isize)> for Point {
    #[inline]
    fn from((x, y): (isize, isize)) -> Self {
        Self::new(x, y)
    }
}

impl Add for Point {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Direction, Point};

    #[test]
    fn point() {
        let mut point = Point::new(1, 2);
        point.shift(Direction::Up);
        point.shift(Direction::Right);
        assert_eq!(point, Point::new(2, 1));
    }
}
