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
        let maximizing_player_symbol = if board
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
            next_board[board_move.index()] = maximizing_player_symbol;

            let score = Player::minimax(&next_board, &maximizing_player_symbol, false, 1);
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

    fn minimax(
        board: &Board,
        maximizing_player_symbol: &Cell,
        is_maximizing: bool,
        depth: i32,
    ) -> i32 {
        if let Some(winning_line_index) = board.get_winning_line() {
            let winner_symbol = board[WINNING_LINES[winning_line_index][0]];

            return if &winner_symbol == maximizing_player_symbol {
                100 - depth
            } else {
                depth - 100
            };
        } else if board.is_full() {
            return 0;
        }

        let (cmp_function, initial_score, current_player_symbol): (fn(i32, i32) -> i32, i32, Cell) =
            if is_maximizing {
                (cmp::max, -1000, *maximizing_player_symbol)
            } else {
                (cmp::min, 1000, maximizing_player_symbol.opposite())
            };

        let mut best_score = initial_score;
        for board_move in board.get_possible_moves() {
            let mut next_board = board.clone();
            next_board[board_move.index()] = current_player_symbol;
            let value = Player::minimax(
                &next_board,
                maximizing_player_symbol,
                !is_maximizing,
                depth + 1,
            );
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
    fn cpu_makes_valid_moves() {
        let mock_ui = MockUi {
            returned_move: None,
        };
        let boards = vec![
            Board::new(),
            Board::from([
                Cell::O,
                Cell::O,
                Cell::X,
                Cell::X,
                Cell::Empty('5'),
                Cell::Empty('6'),
                Cell::Empty('7'),
                Cell::Empty('8'),
                Cell::Empty('9'),
            ]),
            Board::from([
                Cell::O,
                Cell::Empty('2'),
                Cell::Empty('3'),
                Cell::Empty('4'),
                Cell::O,
                Cell::Empty('6'),
                Cell::Empty('7'),
                Cell::X,
                Cell::Empty('9'),
            ]),
        ];

        let cpu = Player::CPU;

        for board in boards {
            let m = cpu.get_move(&board, &mock_ui, None);

            assert!(
                board.is_valid_move(&m),
                "CPU should always return a valid move",
            );
        }
    }

    #[test]
    fn cpu_wins_whenever_possible() {
        let mock_ui = MockUi {
            returned_move: None,
        };
        let boards_and_expected_moves = vec![
            (
                Board::from([
                    Cell::O,
                    Cell::Empty('2'),
                    Cell::X,
                    Cell::O,
                    Cell::O,
                    Cell::Empty('6'),
                    Cell::Empty('7'),
                    Cell::Empty('8'),
                    Cell::X,
                ]),
                BoardMove::try_new(6).unwrap(),
            ),
            // (
            //     Board::from([
            //         Cell::Empty('1'),
            //         Cell::X,
            //         Cell::X,
            //         Cell::Empty('4'),
            //         Cell::O,
            //         Cell::X,
            //         Cell::O,
            //         Cell::Empty('8'),
            //         Cell::O,
            //     ]),
            //     BoardMove::try_new(8).unwrap(),
            // ),
            // (
            //     Board::from([
            //         Cell::O,
            //         Cell::X,
            //         Cell::Empty('3'),
            //         Cell::O,
            //         Cell::X,
            //         Cell::Empty('6'),
            //         Cell::Empty('7'),
            //         Cell::Empty('8'),
            //         Cell::Empty('9'),
            //     ]),
            //     BoardMove::try_new(7).unwrap(),
            // ),
        ];

        let cpu = Player::CPU;

        for (board, expected_move) in boards_and_expected_moves {
            let m = cpu.get_move(&board, &mock_ui, None);

            assert_eq!(
                m, expected_move,
                "CPU should always make winning move whenever possible"
            );
        }
    }

    #[test]
    fn cpu_prevents_player_win() {
        let mock_ui = MockUi {
            returned_move: None,
        };
        let boards_and_expected_moves = vec![
            // In this first case CPU is lost nonetheless, but putting an 'X' in cell 7 prolongs
            // the game
            (
                Board::from([
                    Cell::O,
                    Cell::Empty('2'),
                    Cell::X,
                    Cell::O,
                    Cell::Empty('5'),
                    Cell::Empty('6'),
                    Cell::Empty('7'),
                    Cell::Empty('8'),
                    Cell::Empty('9'),
                ]),
                BoardMove::try_new(7).unwrap(),
            ),
            (
                Board::from([
                    Cell::X,
                    Cell::O,
                    Cell::O,
                    Cell::Empty('4'),
                    Cell::X,
                    Cell::Empty('6'),
                    Cell::Empty('7'),
                    Cell::Empty('8'),
                    Cell::O,
                ]),
                BoardMove::try_new(6).unwrap(),
            ),
            (
                Board::from([
                    Cell::O,
                    Cell::X,
                    Cell::O,
                    Cell::X,
                    Cell::X,
                    Cell::Empty('6'),
                    Cell::Empty('7'),
                    Cell::O,
                    Cell::Empty('9'),
                ]),
                BoardMove::try_new(6).unwrap(),
            ),
        ];

        let cpu = Player::CPU;

        for (board, expected_move) in boards_and_expected_moves {
            let m = cpu.get_move(&board, &mock_ui, None);

            assert_eq!(
                m, expected_move,
                "CPU should always prevent other player from winning whenever possible"
            );
        }
    }

    #[test]
    fn cpu_moves_in_last_free_cell() {
        let mock_ui = MockUi {
            returned_move: None,
        };
        let boards_and_expected_moves = vec![
            (
                Board::from([
                    Cell::O,
                    Cell::Empty('2'),
                    Cell::X,
                    Cell::X,
                    Cell::O,
                    Cell::O,
                    Cell::O,
                    Cell::X,
                    Cell::X,
                ]),
                BoardMove::try_new(2).unwrap(),
            ),
            (
                Board::from([
                    Cell::O,
                    Cell::O,
                    Cell::X,
                    Cell::X,
                    Cell::X,
                    Cell::O,
                    Cell::O,
                    Cell::X,
                    Cell::Empty('9'),
                ]),
                BoardMove::try_new(9).unwrap(),
            ),
        ];

        let cpu = Player::CPU;

        for (board, expected_move) in boards_and_expected_moves {
            let m = cpu.get_move(&board, &mock_ui, None);

            assert_eq!(
                m, expected_move,
                "CPU should always move in last cell if it's the only one available"
            );
        }
    }

    #[test]
    #[should_panic]
    fn panics_on_full_board() {
        let mock_ui = MockUi {
            returned_move: None,
        };
        let full_board = Board::from([
            Cell::O,
            Cell::O,
            Cell::X,
            Cell::X,
            Cell::X,
            Cell::O,
            Cell::O,
            Cell::X,
            Cell::O,
        ]);

        assert!(full_board.is_full(), "Board must be full for this test");

        let cpu = Player::CPU;

        cpu.get_move(&full_board, &mock_ui, None);
    }
}
