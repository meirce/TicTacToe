use failure::{err_msg, Error};
use std::collections::HashMap;

use super::board::Board;

#[derive(Clone, Copy)]
struct MoveValue {
    cell: usize,
    value: i8,
}

pub struct Minmax;

impl Minmax {
    pub fn get_next_move(&self, board: &Board, player_value: &i8) -> Result<usize, Error> {
        Ok(self.minmax(&board, *player_value)?.cell)
    }

    fn minmax(&self, board: &Board, turn: i8) -> Result<MoveValue, Error> {
        let free_moves: Vec<usize> = board
            .data
            .iter()
            .enumerate()
            .filter(|(_, val)| **val == 0)
            .map(|(i, _)| i)
            .collect();

        let value = Minmax::eval_board(&board, 9 - free_moves.len() as i8);
        if value != 0 {
            return Ok(MoveValue { cell: 10, value });
        }

        if free_moves.is_empty() {
            return Ok(MoveValue { cell: 10, value: 0 });
        }

        let mut next_boards: HashMap<usize, MoveValue> = HashMap::new();
        for next_move in free_moves {
            let mut new_board = board.clone();
            new_board.data[next_move] = turn;
            next_boards.insert(
                next_move,
                MoveValue {
                    cell: next_move,
                    value: self.minmax(&new_board, -turn)?.value,
                },
            );
        }

        Ok(*next_boards
            .iter()
            .max_by_key(|(_, j)| turn * j.value)
            .ok_or_else(|| err_msg("Could not find min/max valued move."))?
            .1)
    }

    fn eval_board(board: &Board, depth: i8) -> i8 {
        match board.who_won() {
            0 => 0,
            winner_value => winner_value * 10 + (depth as i8 * -winner_value),
        }
    }
}
