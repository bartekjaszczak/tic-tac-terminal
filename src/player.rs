use crate::board::{Board, BoardMove, Cell, WINNING_LINES};
use crate::ui::Ui;
use rand::Rng;
use std::cmp;

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
            Self::CPU => Self::calculate_best_move(board),
        }
    }

    fn calculate_best_move(board: &Board) -> BoardMove {
        let maximizing_player_marker = if board
            .iter()
            .filter(|&cell| match &cell {
                &Cell::O | Cell::X => true,
                _ => false,
            })
            .count()
            % 2
            == 0
        {
            Cell::O
        } else {
            Cell::X
        };

        let mut best_moves = vec![];
        let mut best_eval = -1000;

        for board_move in board.get_possible_moves() {
            let mut next_board = board.clone();
            next_board[board_move.index()] = maximizing_player_marker;

            let score = Player::minimax(&next_board, &maximizing_player_marker, false);
            if score > best_eval {
                best_eval = score;
                best_moves.clear();
                best_moves.push(board_move);
            } else if score == best_eval {
                best_moves.push(board_move);
            }
        }

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..best_moves.len());
        best_moves[random_index]
    }

    fn minimax(board: &Board, maximizing_player_marker: &Cell, is_maximizing: bool) -> i32 {
        if let Some(winning_line_index) = board.get_winning_line() {
            let winner_marker = board[WINNING_LINES[winning_line_index][0]];

            return if &winner_marker == maximizing_player_marker {
                10
            } else {
                -10
            };
        } else if board.is_full() {
            return 0;
        }

        let (cmp_function, initial_score, current_player_marker): (fn(i32, i32) -> i32, i32, Cell) =
            if is_maximizing {
                (cmp::max, -1000, *maximizing_player_marker)
            } else {
                (cmp::min, 1000, maximizing_player_marker.opposite())
            };

        let mut best_score = initial_score;
        for board_move in board.get_possible_moves() {
            let mut next_board = board.clone();
            next_board[board_move.index()] = current_player_marker;
            let value = Player::minimax(&next_board, maximizing_player_marker, !is_maximizing);
            best_score = cmp_function(best_score, value);
        }
        return best_score;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameResult;

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

        fn notify_result(&self, _result: &GameResult) {
            panic!("notify_result shouldn't be called");
        }
    }

    #[test]
    fn get_human_move() {
        let returned_move = BoardMove::try_new(3).unwrap();
        let mock_ui = MockUi {
            returned_move: returned_move.clone(),
        };
        let fake_board = Board::new();
        let player = Player::Human(String::from("Steve"));

        let m = player.get_move(&fake_board, &mock_ui, None);

        assert_eq!(m, returned_move, "Player should return move given by Ui");
    }

    #[test]
    #[ignore = "Ignored until it's rewritten"]
    fn get_cpu_move() {
        // TODO: Update the test case once the functionality is in place
        let returned_move = BoardMove::try_new(3).unwrap();
        let mock_ui = MockUi { returned_move };
        let fake_board = Board::new();

        let cpu = Player::CPU;

        let _m = cpu.get_move(&fake_board, &mock_ui, None);
    }
}
