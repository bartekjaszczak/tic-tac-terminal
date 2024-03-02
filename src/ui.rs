pub mod tui;
pub mod gui;

use crate::board::{Board, BoardMove};
use crate::game::GameResult;

pub use tui::TerminalUi;

pub trait Ui {
    fn get_move(&self, player_name: &str, additional_message: Option<&str>) -> BoardMove;
    fn update_board(&self, board: &Board);
    fn notify_result(&self, result: &GameResult);
}
