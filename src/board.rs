use crate::game::WinningLineIndex;
use std::{
    fmt,
    ops::{Index, IndexMut},
};

type Cells = [Cell; 9];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardMove {
    index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty(char),
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

pub const WINNING_LINES: [[usize; 3]; 8] = [
    [0, 3, 6], // 1st column
    [1, 4, 7], // 2nd column
    [2, 5, 8], // 3rd column
    [0, 1, 2], // 1st row
    [3, 4, 5], // 2nd row
    [6, 7, 8], // 3rd row
    [0, 4, 8], // main diagonal
    [2, 4, 6], // secondary diagonal
];

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cell::Empty(index) => write!(f, "{index}"),
            Cell::O => write!(f, "O"),
            Cell::X => write!(f, "X"),
        }
    }
}

impl fmt::Display for BoardMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index() - 1)
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

impl Cell {
    pub fn opposite(&self) -> Cell {
        match *self {
            Cell::O => Cell::X,
            Cell::X => Cell::O,
            Cell::Empty(n) => Cell::Empty(n)
        }
    }
}

impl BoardMove {
    pub fn try_new(num: usize) -> Result<BoardMove, ()> {
        if num < 1 || num > 9 {
            return Err(());
        }

        Ok(BoardMove { index: num - 1 })
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl Board {
    pub fn new() -> Board {
        let mut cells = [Cell::Empty('0'); 9];
        for i in 0..9 {
            let ascii_num = ((i + 1) as u8 + b'0') as char;
            cells[i] = Cell::Empty(ascii_num); // These values serve as cell position
        }
        Board { cells }
    }

    pub fn from(cells: Cells) -> Board {
        Board { cells }
    }

    pub fn iter(&self) -> BoardIterator {
        BoardIterator {
            inner: self.cells.iter(),
        }
    }

    pub fn is_full(&self) -> bool {
        !self.iter().any(|&cell| matches!(cell, Cell::Empty(_)))
    }

    pub fn get_possible_moves(&self) -> Vec<BoardMove> {
        let mut moves = Vec::new();

        for (index, cell) in self.iter().enumerate() {
            if let &Cell::Empty(_) = cell {
                moves.push(BoardMove::try_new(index + 1).unwrap())
            }
        }

        moves
    }

    pub fn is_valid_move(&self, board_move: &BoardMove) -> bool {
        self.get_possible_moves().contains(board_move)
    }

    pub fn get_winning_line(&self) -> Option<WinningLineIndex> {
        let b = &self.cells;

        for (index, cell_triplet) in WINNING_LINES.iter().enumerate() {
            let (c1, c2, c3) = (b[cell_triplet[0]], b[cell_triplet[1]], b[cell_triplet[2]]);
            if c1 == c2 && c1 == c3 {
                match c1 {
                    Cell::O | Cell::X => return Some(index),
                    _ => continue,
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_move() {
        for cell in 1..=9 {
            let m = BoardMove::try_new(cell);

            assert!(m.is_ok(), "Move should be constructed properly");

            let m = m.unwrap();

            assert_eq!(m.index(), cell - 1);
        }
    }

    #[test]
    fn incorrect_move() {
        for index in [0, 10, 11, 100, 55555] {
            let m = BoardMove::try_new(index);

            assert!(
                m.is_err(),
                "Move should fail to construct with move out of 1..=9 range"
            );
        }
    }

    #[test]
    fn move_validation() {
        let mut board = Board::new();

        let valid_move = BoardMove::try_new(1).unwrap();
        let invalid_move1 = BoardMove::try_new(4).unwrap();
        let invalid_move2 = BoardMove::try_new(5).unwrap();

        assert!(
            board.is_valid_move(&valid_move),
            "All moves should be valid when game starts"
        );
        assert!(
            board.is_valid_move(&invalid_move1),
            "All moves should be valid when game starts"
        );
        assert!(
            board.is_valid_move(&invalid_move2),
            "All moves should be valid when game starts"
        );

        board[3] = Cell::X;
        board[4] = Cell::O;

        assert!(
            board.is_valid_move(&valid_move),
            "Move on empty cell should be valid"
        );
        assert!(
            !board.is_valid_move(&invalid_move1),
            "Move on occupied cell shouldn't be valid"
        );
        assert!(
            !board.is_valid_move(&invalid_move2),
            "Move on occupied cell shouldn't be valid"
        );
    }

    #[test]
    fn full_board_check() {
        let board = Board::new();

        assert!(
            !board.is_full(),
            "Board is empty - method should return false"
        );

        let mut board = Board::from([Cell::O; 9]);

        assert!(board.is_full(), "Board is full - method should return true");

        board[4] = Cell::Empty('5');

        assert!(
            !board.is_full(),
            "Board is almost full - method should return false"
        );
    }

    #[test]
    fn possible_moves() {
        let board = Board::new();
        let possible_moves = board.get_possible_moves();

        assert_eq!(
            possible_moves.len(),
            9,
            "There should be 9 possible moves on empty board"
        );

        let mut board = Board::from([Cell::O; 9]);
        let possible_moves = board.get_possible_moves();

        assert_eq!(
            possible_moves.len(),
            0,
            "There should be no possible moves on full board"
        );

        board[4] = Cell::Empty('5');
        let possible_moves = board.get_possible_moves();

        assert_eq!(
            possible_moves.len(),
            1,
            "There should be only one move possible"
        );
        assert_eq!(
            possible_moves,
            vec![BoardMove::try_new(5).unwrap()],
            "Only move '5' should be available"
        )
    }

    #[test]
    fn valid_move() {
        let mut board = Board::new();

        let move1 = BoardMove::try_new(3).unwrap();
        let move2 = BoardMove::try_new(4).unwrap();
        let move3 = BoardMove::try_new(5).unwrap();

        assert!(
            board.is_valid_move(&move1),
            "Move on empty cell should be possible"
        );
        assert!(
            board.is_valid_move(&move2),
            "Move on empty cell should be possible"
        );
        assert!(
            board.is_valid_move(&move3),
            "Move on empty cell should be possible"
        );

        board[2] = Cell::O;

        assert!(
            !board.is_valid_move(&move1),
            "Move on occupied cell shouldn't be possible"
        );
        assert!(
            board.is_valid_move(&move2),
            "Move on empty cell should be possible"
        );
        assert!(
            board.is_valid_move(&move3),
            "Move on empty cell should be possible"
        );

        let board = Board::from([Cell::O; 9]);

        assert!(
            !board.is_valid_move(&move1),
            "Move on occupied cell shouldn't be possible"
        );
        assert!(
            !board.is_valid_move(&move2),
            "Move on occupied cell shouldn't be possible"
        );
        assert!(
            !board.is_valid_move(&move3),
            "Move on occupied cell shouldn't be possible"
        );
    }

    #[test]
    fn winning_lines() {
        let board = Board::new();

        assert_eq!(
            board.get_winning_line(),
            None,
            "There should be no winning line in empty board"
        );

        let mut board = Board::from([
            Cell::O,
            Cell::X,
            Cell::Empty('3'),
            Cell::O,
            Cell::X,
            Cell::Empty('6'),
            Cell::Empty('7'),
            Cell::Empty('8'),
            Cell::Empty('9'),
        ]); // Almost-winning board

        assert_eq!(
            board.get_winning_line(),
            None,
            "There should be no winning line"
        );

        board[6] = Cell::O;

        if let None = board.get_winning_line() {
            panic!("There should be winnig line - three 'O's in column 1");
        }
    }
}
