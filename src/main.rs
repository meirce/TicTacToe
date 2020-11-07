use failure::Error;

mod tictactoe;

use tictactoe::game::Game;

fn main() -> Result<(), Error> {
    let mut game = Game::new();

    game.main_loop()?;

    Ok(())
}
