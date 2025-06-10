use std::error::Error as StdError;
use sdl2::{
    render::{Canvas, TextureQuery},
    ttf::Sdl2TtfContext,
    video::Window,
    rect::{Rect, Point as SdlPoint},
    pixels::Color,
};
use serpent::Serpent;
use crate::{Game, matrix::Matrix};

const PADDING: u32 = 75;
const CELL_SIZE: u32 = 52;
const MATRIX_WIDTH: u32 = CELL_SIZE * Matrix::WIDTH as u32;
const MATRIX_HEIGHT: u32 = CELL_SIZE * Matrix::HEIGHT as u32;
pub(crate) const WIDTH: u32 = MATRIX_WIDTH;
pub(crate) const HEIGHT: u32 = MATRIX_HEIGHT + PADDING;

pub fn draw(canvas: &mut Canvas<Window>, ttf: &Sdl2TtfContext, game: &mut Game) -> Result<(), Box<dyn StdError>> {
    // draw score
    text(
        canvas,
        ttf,
        50,
        10,
        format!("Score: {}", game.score).as_str(),
        "/System/Library/Fonts/Supplemental/Arial.ttf"
    )?;
    
    // draw matrix background
    canvas.set_draw_color(Color::RGB(50, 50, 50));
    canvas.fill_rect(Rect::new(
        0,
        PADDING as i32,
        MATRIX_WIDTH,
        MATRIX_HEIGHT,
    ))?;
    
    // draw apples
    for (i, row) in game.matrix.iter().enumerate() {
        for (j, &b) in row.iter().enumerate() {
            if b {
                canvas.set_draw_color(Matrix::APPLE_COLOR);
                canvas.fill_rect(Rect::new(
                    j as i32 * CELL_SIZE as i32,
                    i as i32 * CELL_SIZE as i32 + PADDING as i32,
                    CELL_SIZE,
                    CELL_SIZE,
                ))?;
            }
        }
    }

    // draw snake
    for coord in game.snake.coords.iter() {
        canvas.set_draw_color(Serpent::COLOR);
        canvas.fill_rect(Rect::new(
            coord.x as i32 * CELL_SIZE as i32,
            coord.y as i32 * CELL_SIZE as i32 + PADDING as i32,
            CELL_SIZE,
            CELL_SIZE,
        ))?;
    }
    
    canvas.set_draw_color(Color::BLACK);
    // horizontal
    for i in 0..=Matrix::HEIGHT {
        let y = (i as u32 * CELL_SIZE) as i32 + (PADDING as i32);
        canvas.draw_line(
            SdlPoint::new(0, y),
            SdlPoint::new(MATRIX_WIDTH as i32, y)
        )?;
    }
    // vertical
    for i in 0..=Matrix::WIDTH {
        let x = (i as u32 * CELL_SIZE) as i32;
        canvas.draw_line(
            SdlPoint::new(x, PADDING as i32),
            SdlPoint::new(x, HEIGHT as i32)
        )?;
    }
    
    
    
    Ok(())
}

fn text(canvas: &mut Canvas<Window>, ttf: &Sdl2TtfContext, x: i32, y: i32, text: &str, font_path: &str) -> Result<(), Box<dyn StdError>> {
    let texture_creator = canvas.texture_creator();

    let font = ttf.load_font(font_path, 50)?;

    let surface = font
        .render(text)
        .blended(Color::WHITE)?;

    let texture = texture_creator.create_texture_from_surface(&surface)?;

    let TextureQuery { width, height, .. } = texture.query();

    let target = Rect::new(x, y, width, height);

    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}
