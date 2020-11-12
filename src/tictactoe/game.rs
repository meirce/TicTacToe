use failure::{err_msg, Error};
use text_io::read;

use super::board::Board;
use super::minmax::Minmax;

pub struct Game {
    board: Board,
    computer: Minmax,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            computer: Minmax::new(),
        }
    }

    fn user_move(&mut self, player_value: &i8) -> Result<(), Error> {
        for retry_count in 0..3 {
            println!("{}", self.board);
            println!("Please choose your move: ");

            let mut next_move = read!("{}\r\n");
            next_move -= 1;
            match self.board.try_set(&next_move, &player_value) {
                Ok(()) => break,
                err => {
                    if retry_count == 2 {
                        return err;
                    }
                    println!("{:?}", err);
                }
            }
        }
        Ok(())
    }

    fn computer_move(&mut self, player_value: &i8) -> Result<(), Error> {
        let next_move = self.computer.get_next_move(&self.board, &player_value)?;
        println!("{}", self.board);
        self.board.try_set(&next_move, &player_value)?;
        Ok(())
    }

    fn check_for_win(&self) -> Result<bool, Error> {
        if self.board.data.iter().filter(|x| **x == 0).count() == 0 {
            println!("{}", self.board);
            println!("Tie!");
            return Ok(true);
        }

        match self.board.who_won() {
            0 => Ok(false),
            winner => {
                println!("{}", self.board);
                match winner {
                    1 => {
                        println!("X won!");
                    }
                    -1 => {
                        println!("O won!");
                    }
                    _ => return Err(err_msg("Invalid Player")),
                }
                Ok(true)
            }
        }
    }

    pub fn main_loop(&mut self) -> Result<(), Error> {
        let mut player_value = 1;
        for _ in 0..9 {
            self.user_move(&player_value)?;

            if self.check_for_win()? {
                return Ok(());
            }

            player_value *= -1;

            self.computer_move(&player_value)?;

            if self.check_for_win()? {
                return Ok(());
            }

            player_value *= -1;
        }

        Ok(())
    }
}
