use std::{error::Error as StdError, thread, time::Duration};
use std::time::Instant;
use sdl2::{event::Event::{self, KeyDown}, keyboard::{Keycode}, pixels::Color};
use geomath::Direction;
use serpent::Serpent;
use crate::matrix::Matrix;

mod matrix;
mod interface;

#[derive(Default)]
pub struct Game {
    matrix: Matrix,
    snake: Serpent,
    score: i32,
}

impl Game {
    pub const TICK: Duration = Duration::from_millis(200);
    pub fn run(&mut self) -> Result<(), Box<dyn StdError>> {
        let sdl = sdl2::init()?;
        let ttf = sdl2::ttf::init()?;
        let video = sdl.video()?;

        let window = video
            .window("Rusty Snake", interface::WIDTH, interface::HEIGHT)
            .position_centered()
            .build()?;

        let mut canvas = window
            .into_canvas()
            .build()?;

        let mut event_pump = sdl.event_pump()?;

        let mut last_tick = Instant::now();
        
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'running,
                    KeyDown { keycode: Some(key), .. } => {
                        if key == Keycode::Escape {
                            break 'running;
                        }
                        self.update(key); // change direction
                    }
                    _ => {}
                }
            }

            if last_tick.elapsed() >= Self::TICK {
                if self.matrix[self.snake.head()] {
                    self.matrix[self.snake.head()] = false;
                    self.snake.ate = true;
                    self.matrix.place_apple(&mut self.snake);
                }
                self.make_move();
                if !self.matrix.in_bounds(self.snake.head()) || self.snake.hit() {
                    println!("You lost!");
                    std::process::exit(0);
                }
                last_tick = Instant::now();
            }

            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            interface::draw(&mut canvas, &ttf, self)?;

            canvas.present();

            // FPS
            thread::sleep(Duration::from_millis(16));
        }
        
        Ok(())
    }
    
    pub fn update(&mut self, key: Keycode) {
        let Ok(new): Result<Direction, _> = key.try_into() else {
            return;
        };
        if self.snake.direction.opposite() == new {
            return;
        }
        self.snake.direction = new;
    }


    pub fn make_move(&mut self) {
        for i in (1..self.snake.coords.len()).rev() {
            self.snake.coords[i] = self.snake.coords[i - 1]
        }
        if self.snake.ate {
            self.snake.coords.push_front(self.snake.coords[0].ret_shift(self.snake.direction));
            self.snake.ate = false;
            self.score += 1
        } else {
            self.snake.coords[0].shift(self.snake.direction);
        }
    }
}
