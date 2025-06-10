use sdl2::keyboard::Keycode;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Direction {
    Up,
    #[default]
    Right,
    Down,
    Left,
}

impl TryFrom<Keycode> for Direction {
    type Error = ();
    fn try_from(key: Keycode) -> Result<Self, Self::Error> {
        match key {
            Keycode::Up | Keycode::W => Ok(Self::Up),
            Keycode::Down | Keycode::S => Ok(Self::Down),
            Keycode::Left | Keycode::A => Ok(Self::Left),
            Keycode::Right | Keycode::D => Ok(Self::Right),
            _ => Err(())
        }
    }
}

impl From<usize> for Direction {
    fn from(item: usize) -> Self {
        match item  {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Direction {
    #[inline]
    pub fn opposite(self) -> Self {
        Self::from((self as usize + 2) % 4)
    }
}
