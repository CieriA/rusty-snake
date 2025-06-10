use geomath::{Direction, Point};
use sdl2::pixels::Color;
use std::collections::VecDeque;

pub struct Serpent {
    // change to VecDeque (?)
    pub coords: VecDeque<Point>,
    pub direction: Direction,
    pub ate: bool,
}

impl Default for Serpent {
    fn default() -> Self {
        Self {
            coords: [(4, 7), (3, 7), (2, 7)].into_iter().map(Into::into).collect(),
            direction: Direction::default(),
            ate: false,
        }
    }
}

impl Serpent {
    pub const COLOR: Color = Color::GREEN;
    pub fn head(&self) -> Point {
        self.coords[0]
    }
    pub fn hit(&self) -> bool {
        self.coords
            .iter()
            .skip(1)
            .any(|&coord| coord == self.head())
    }
}
