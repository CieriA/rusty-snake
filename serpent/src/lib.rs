mod matrix;

use advmath::Point;

pub struct Serpent(Vec<Point>);

impl Serpent {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}
