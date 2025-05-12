use advmath::Point;

pub const MATRIX_WIDTH: usize = 18;
pub const MATRIX_HEIGHT: usize = 15;
pub fn in_bounds(point: Point) -> bool {
    point.x < MATRIX_WIDTH as isize &&
        point.y < MATRIX_HEIGHT as isize &&
        point.x >= 0 &&
        point.y >= 0
}