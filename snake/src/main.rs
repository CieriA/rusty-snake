use std::error::Error as StdError;
use game::Game;

fn main() -> Result<(), Box<dyn StdError>> {
    Game::default()
        .run()
}
