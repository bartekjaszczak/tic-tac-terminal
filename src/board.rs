use std::{
    fmt,
    ops::{Index, IndexMut},
};

type Cells = [Cell; 9];

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Board {
    cells: Cells,
}

pub struct BoardIterator<'a> {
    inner: std::slice::Iter<'a, Cell>,
}

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

impl Index<usize> for Board {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
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

impl Board {
    pub fn new() -> Board {
        let mut cells = [Cell::Empty(0); 9];
        for i in 0..9 {
            cells[i] = Cell::Empty(i + 1); // Put numbers 1..=9 into Empty cells. They'll
                                           // serve as cell positions
        }
        Board { cells }
    }

    pub fn from(cells: Cells) -> Board {
        Board {
            cells
        }
    }

    pub fn iter(&self) -> BoardIterator {
        BoardIterator {
            inner: self.cells.iter(),
        }
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
