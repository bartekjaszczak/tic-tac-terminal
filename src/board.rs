use std::fmt;

pub struct Move {
    index: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty(usize),
    O,
    X,
}

pub type Board = [Cell; 9];

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cell::Empty(index) => write!(f, "{index}"),
            Cell::O => write!(f, "O"),
            Cell::X => write!(f, "X"),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.marker())
    }
}

impl Move {
    pub fn try_new(num: usize) -> Result<Move, ()> {
        if num < 1 || num > 9 {
            return Err(());
        }

        Ok(Move { index: num - 1 })
    }

    pub fn index(&self) -> usize {
        self.index
    }

    fn marker(&self) -> usize {
        self.index + 1
    }
}
