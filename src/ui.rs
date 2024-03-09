//! This module contains the Ui trait, which has to be implemented for every UI instance (such as
//! terminal UI and graphic UI). Also provides a mock UI object which can be helpful in testing.

mod tui;

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
    fn update_scores(
        &self,
        player1_name: &str,
        player1_score: i32,
        player2_name: &str,
        player2_score: i32,
    );
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::board::BoardMove;
    use std::cell::RefCell;

    pub struct MockUiBuilder {
        expected_moves: RefCell<Vec<BoardMove>>,
        expected_names: RefCell<Vec<String>>,

        update_scores_count: RefCell<u32>,
        notify_result_calls: RefCell<u32>,
        get_move_calls: RefCell<u32>,
    }

    pub struct MockUi {
        expected_moves: RefCell<Vec<BoardMove>>,
        expected_names: RefCell<Vec<String>>,

        update_scores_count: RefCell<u32>,
        notify_result_calls: RefCell<u32>,
        get_move_calls: RefCell<u32>,
    }

    impl Ui for MockUi {
        fn get_move(&self, _player_name: &str, _additional_message: Option<&str>) -> BoardMove {
            *self.get_move_calls.borrow_mut() += 1;
            self.expected_moves.borrow_mut().remove(0) // Make sure there are enough fake moves
        }

        fn update_board(&self, _board: &Board) {
            // Silently ignore the call
        }

        fn notify_result(&self, _result: &GameResult) {
            *self.notify_result_calls.borrow_mut() += 1;
        }

        fn get_player_name(&self, _name_placeholder: &str) -> String {
            self.expected_names.borrow_mut().remove(0) // Make sure there are enough fake names
        }

        fn select_mode(&self) -> GameMode {
            panic!("Mock method select_mode not used")
        }

        fn keep_playing(&self) -> bool {
            panic!("Mock method keep_playing not used")
        }

        fn update_scores(
            &self,
            _player1_name: &str,
            _player1_score: i32,
            _player2_name: &str,
            _player2_score: i32,
        ) {
            *self.update_scores_count.borrow_mut() += 1;
        }
    }

    impl MockUiBuilder {
        pub fn new() -> Self {
            Self {
                expected_moves: RefCell::new(vec![]),
                expected_names: RefCell::new(vec![]),
                update_scores_count: RefCell::new(0),
                notify_result_calls: RefCell::new(0),
                get_move_calls: RefCell::new(0),
            }
        }

        pub fn expected_moves(self, expected_moves: Vec<BoardMove>) -> Self {
            self.expected_moves.replace(expected_moves);
            self
        }

        pub fn expected_names(self, expected_names: Vec<String>) -> Self {
            self.expected_names.replace(expected_names);
            self
        }

        pub fn build(self) -> MockUi {
            MockUi {
                expected_moves: self.expected_moves,
                expected_names: self.expected_names,
                update_scores_count: self.update_scores_count,
                notify_result_calls: self.notify_result_calls,
                get_move_calls: self.get_move_calls,
            }
        }
    }

    impl MockUi {
        pub fn builder() -> MockUiBuilder {
            MockUiBuilder::new()
        }

        pub fn update_scores_count(&self) -> u32 {
            *self.update_scores_count.borrow()
        }

        pub fn notify_result_calls(&self) -> u32 {
            *self.notify_result_calls.borrow()
        }

        pub fn get_move_calls(&self) -> u32 {
            *self.get_move_calls.borrow()
        }
    }
}
