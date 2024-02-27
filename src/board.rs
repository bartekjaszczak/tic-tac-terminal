use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_move() {
        for index in 1..=9 {
            let m = Move::try_new(index);

            assert!(m.is_ok(), "Move should be constructed properly");

            let m = m.unwrap();

            assert_eq!(m.index(), index - 1);
            assert_eq!(m.marker(), index);
        }
    }

    #[test]
    fn incorrect_move() {
        for index in [0, 10, 11, 100, 55555] {
            let m = Move::try_new(index);

            assert!(
                m.is_err(),
                "Move should fail to construct with move out of 1..=9 range"
            );
        }
    }
}
