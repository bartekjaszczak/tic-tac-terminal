mod tui;
mod gui;

use crate::board::{Board, BoardMove};
use crate::game::GameResult;
use crate::tictactoe::GameMode;

pub use tui::TerminalUi;

pub trait Ui {
    fn get_move(&self, player_name: &str, additional_message: Option<&str>) -> BoardMove;
    fn update_board(&self, board: &Board);
    fn notify_result(&self, result: &GameResult);
    fn get_player_name(&self, name_placeholder: &str) -> String;
    fn select_mode(&self) -> GameMode;
    fn keep_playing(&self) -> bool;
    fn update_scores(&self, player1_name: &str, player1_score: i32, player2_name: &str, player2_score: i32);
}
