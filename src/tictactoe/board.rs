use failure::{err_msg, Error};
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Board {
    pub data: [i8; 9],
    ways_to_win: [[usize; 3]; 8],
}

impl Board {
    pub fn new() -> Self {
        Board {
            data: [0; 9],
            ways_to_win: [
                [0, 1, 2],
                [3, 4, 5],
                [6, 7, 8],
                [0, 3, 6],
                [1, 4, 7],
                [2, 5, 8],
                [0, 4, 8],
                [2, 4, 6],
            ],
        }
    }

    pub fn try_set(&mut self, index: &usize, value: &i8) -> Result<(), Error> {
        if *index > 8 {
            return Err(err_msg("Cell index must be between 1 and 9."));
        }
        if ![-1, 1].contains(&value) {
            return Err(err_msg("Only two players can play."));
        }
        match self.data[*index] {
            0 => {
                self.data[*index] = *value;
                Ok(())
            }
            _ => Err(err_msg(format!("Cell number {} is not free.", index + 1))),
        }
    }

    pub fn who_won(&self) -> i8 {
        self.ways_to_win
            .iter()
            .map(|x| x.iter().map(|y| self.data[*y]).sum::<i8>())
            .enumerate()
            .map(|(i, x)| {
                if x.abs() == 3 {
                    self.data[self.ways_to_win[i][0]]
                } else {
                    0
                }
            })
            .sum()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|x| match x {
                    1 => 'X',
                    -1 => 'O',
                    _ => ' ',
                })
                .enumerate()
                .fold(String::new(), |s, (i, j)| if i % 3 == 0 {
                    format!("{}\r\n{}", s, j)
                } else {
                    format!("{} | {}", s, j)
                })
        )
    }
}
