mod minimax;

use crate::board::{Board, BoardMove};
use crate::ui::Ui;
use std::thread;
use std::time::Duration;

pub enum Player {
    Human(String),
    CPU,
}

impl Player {
    pub fn get_move(
        &self,
        board: &Board,
        ui: &impl Ui,
        additional_message: Option<&str>,
    ) -> BoardMove {
        match self {
            Self::Human(name) => ui.get_move(name, additional_message),
            Self::CPU => {
                if !cfg!(test) {
                    thread::sleep(Duration::from_millis(200));
                }
                minimax::calculate_best_move(board)
            }
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Self::Human(name) => name,
            Self::CPU => "CPU",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameResult;
    use crate::tictactoe::GameMode;

    struct MockUi {
        returned_move: Option<BoardMove>,
    }

    impl Ui for MockUi {
        fn get_move(&self, _player_name: &str, _additional_message: Option<&str>) -> BoardMove {
            self.returned_move.unwrap().clone()
        }

        fn update_board(&self, _board: &Board) {
            panic!("update_board shouldn't be called");
        }

        fn notify_result(&self, _result: &GameResult) {
            panic!("notify_result shouldn't be called");
        }

        fn get_player_name(&self, _player_name: &str) -> String {
            panic!("get_player_name shouldn't be called");
        }

        fn select_mode(&self) -> GameMode {
            panic!("select_mode shouldn't be called");
        }

        fn keep_playing(&self) -> bool {
            panic!("keep_playing shouldn't be called");
        }

        fn update_scores(
            &self,
            _player1_name: &str,
            _player1_score: i32,
            _player2_name: &str,
            _player2_score: i32,
        ) {
            panic!("update_scores shouldn't be called");
        }
    }

    #[test]
    fn get_human_move() {
        let returned_move = BoardMove::try_new(3).unwrap();
        let mock_ui = MockUi {
            returned_move: Some(returned_move.clone()),
        };
        let fake_board = Board::new();
        let player = Player::Human(String::from("Steve"));

        let m = player.get_move(&fake_board, &mock_ui, None);

        assert_eq!(m, returned_move, "Player should return move given by Ui");
    }

    #[test]
    fn get_cpu_move() {
        let mock_ui = MockUi {
            returned_move: None,
        };
        let fake_board = Board::new();
        let cpu = Player::CPU;

        cpu.get_move(&fake_board, &mock_ui, None);
    }

    #[test]
    fn get_player_name() {
        let human = Player::Human(String::from("Steve"));

        assert_eq!(human.get_name(), "Steve");

        let cpu = Player::CPU;

        assert_eq!(cpu.get_name(), "CPU");
    }
}
