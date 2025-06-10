use std::ops::{Index, IndexMut};
use sdl2::pixels::Color;
use geomath::Point;
use rand::random_range;
use serpent::Serpent;

type Grid = [[bool; Matrix::WIDTH]; Matrix::HEIGHT];

/// A 2*2 Array containing either true (apple) or false (nothing)
pub struct Matrix(Grid);

impl Default for Matrix {
    fn default() -> Self {
        let mut matrix = Self(Grid::default());
        matrix.place_apple(&mut Serpent::default());
        matrix
    }
}

impl Index<Point> for Matrix {
    type Output = bool;
    fn index(&self, index: Point) -> &Self::Output {
        assert!(self.in_bounds(index), "value of (x, y): {}", index);
        &self.0[index.y as usize][index.x as usize]
    }
}
impl IndexMut<Point> for Matrix {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        assert!(self.in_bounds(index), "value of (x, y): {}", index);
        &mut self.0[index.y as usize][index.x as usize]
    }
}
// USELESS (?)
impl Index<usize> for Matrix {
    type Output = [bool; Self::WIDTH];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Matrix {
    pub const APPLE_COLOR: Color = Color::RED;
    pub const WIDTH: usize = 17;
    pub const HEIGHT: usize = 15;

    pub fn iter(&self) -> core::slice::Iter<[bool; Self::WIDTH]> {
        self.0.iter()
    }
    #[inline]
    pub const fn in_bounds(&self, coord: Point) -> bool {
        coord.x >= 0 &&
        coord.y >= 0 &&
        coord.x < Self::WIDTH as isize &&
        coord.y < Self::HEIGHT as isize
    }
    
    pub fn place_apple(&mut self, snake: &mut Serpent) {
        let coord = loop {
            let point = Point::new(
                random_range(0..Self::WIDTH) as isize,
                random_range(0..Self::HEIGHT) as isize
            );
            if snake.coords.contains(&point) {
                continue;
            }
            if self[point] {
                continue;
            }
            break point;
        };
        self[coord] = true;
    }
}
