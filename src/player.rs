use crate::board::{Board, BoardMove};
use crate::ui::Ui;

pub enum Player {
    Human(String),
    CPU,
}

impl Player {
    pub fn get_move(&self, board: &Board, ui: &impl Ui, additional_message: Option<&str>) -> BoardMove {
        match self {
            Self::Human(name) => ui.get_move(name, additional_message),
            Self::CPU => Self::calculate_best_move(board),
        }
    }

    fn calculate_best_move(_board: &Board) -> BoardMove {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockUi {
        returned_move: BoardMove,
    }

    impl Ui for MockUi {
        fn get_move(&self, _player_name: &str, _additional_message: Option<&str>) -> BoardMove {
            self.returned_move.clone()
        }

        fn update_board(&self, _board: &Board) {
            panic!("update_board shouldn't be called");
        }

        fn notify_result(&self, _result: &crate::game::GameResult) {
            panic!("notify_result shouldn't be called");
        }
    }

    #[test]
    fn get_human_move() {
        let returned_move = BoardMove::try_new(3).unwrap();
        let mock_ui = MockUi {
            returned_move: returned_move.clone()
        };
        let fake_board = Board::new();
        let player = Player::Human(String::from("Steve"));

        let m = player.get_move(&fake_board, &mock_ui, None);

        assert_eq!(m, returned_move, "Player should return move given by Ui");
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn get_cpu_move() {
        // TODO: Update the test case once the functionality is in place
        let returned_move = BoardMove::try_new(3).unwrap();
        let mock_ui = MockUi { returned_move };
        let fake_board = Board::new();

        let cpu = Player::CPU;

        let _m = cpu.get_move(&fake_board, &mock_ui, None);
    }
}
