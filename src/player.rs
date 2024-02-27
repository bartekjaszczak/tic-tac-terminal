use crate::ui::Ui;
use crate::board::{Board, Move};

pub enum Player {
    Human(String),
    CPU,
}

impl Player {
    pub fn get_move(&self, board: &Board, ui: &impl Ui, additional_message: Option<&str>) -> Move {
        match self {
            Self::Human(name) => ui.get_move(name, additional_message),
            Self::CPU => Self::calculate_best_move(board),
        }
    }

    fn calculate_best_move(_board: &Board) -> Move {
        todo!()
    }
}
