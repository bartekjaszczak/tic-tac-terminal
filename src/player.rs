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
    use crate::ui::tests::MockUi;

    #[test]
    fn get_human_move() {
        let returned_move = BoardMove::try_new(3).unwrap();
        let mock_ui = MockUi::builder()
            .expected_moves(vec![returned_move.clone()])
            .build();
        let fake_board = Board::new();
        let player = Player::Human(String::from("Steve"));

        let m = player.get_move(&fake_board, &mock_ui, None);

        assert_eq!(m, returned_move, "Player should return move given by Ui");
    }

    #[test]
    fn get_cpu_move() {
        let mock_ui = MockUi::builder().build();
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
