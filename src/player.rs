mod minimax;

use crate::board::{Board, BoardMove};
use crate::ui::Ui;

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
            Self::CPU => minimax::calculate_best_move(board),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameResult;

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
}
