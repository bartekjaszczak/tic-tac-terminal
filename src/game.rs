use crate::board::{Board, BoardMove, Cell, WINNING_LINES};
use crate::player::Player;
use crate::ui::Ui;

pub type WinningLineIndex = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum GameResult {
    PlayerWon(usize, String, WinningLineIndex),
    Draw,
}

#[derive(Clone, Debug, PartialEq)]
enum GameState {
    NotStarted,
    Ongoing,
    Finished(GameResult),
}

pub struct Game<'a, T: Ui> {
    board: Board,
    players: [&'a Player; 2],
    current_player: usize,
    game_state: GameState,
    ui: &'a T,
}

impl<'a, T: Ui> Game<'a, T> {
    pub fn new(player1: &'a Player, player2: &'a Player, ui_backend: &'a T) -> Self {
        Self {
            board: Board::new(),
            players: [player1, player2],
            current_player: 0,
            game_state: GameState::NotStarted,
            ui: ui_backend,
        }
    }

    pub fn start(&mut self) -> Result<GameResult, ()> {
        if self.game_state == GameState::NotStarted {
            self.game_state = GameState::Ongoing;

            while self.game_state == GameState::Ongoing {
                self.take_turn();
                self.check_if_over();

                self.current_player = if self.current_player == 0 { 1 } else { 0 };
            }

            self.announce_result();
        }

        if let GameState::Finished(result) = &self.game_state {
            Ok(result.clone())
        } else {
            Err(())
        }
    }

    fn take_turn(&mut self) {
        let current_player = &self.players[self.current_player];

        self.ui.update_board(&self.board);

        let mut error_message = None;

        let board_move = loop {
            let board_move = current_player.get_move(&self.board, self.ui, error_message);
            if self.board.is_valid_move(&board_move) {
                break board_move;
            } else {
                error_message = Some("this cell is not empty");
            }
        };

        self.current_player_make_move(board_move);
    }

    fn current_player_make_move(&mut self, board_move: BoardMove) {
        self.board[board_move.index()] = self.board.current_player_symbol();
    }

    fn check_if_over(&mut self) {
        if let Some(winning_line_index) = self.board.get_winning_line() {
            let winner = match self.board[WINNING_LINES[winning_line_index][0]] {
                Cell::O => 0,
                Cell::X => 1,
                Cell::Empty(_) => panic!("Winning line cannot be empty"),
            };

            let winner_name = self.players[winner].get_name().to_owned();

            self.game_state = GameState::Finished(GameResult::PlayerWon(
                winner,
                winner_name,
                winning_line_index,
            ));
        } else if self.board.is_full() {
            self.game_state = GameState::Finished(GameResult::Draw);
        }
    }

    fn announce_result(&self) {
        self.ui.update_board(&self.board);

        match &self.game_state {
            GameState::Finished(result) => {
                self.ui.notify_result(&result);
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::tests::MockUi;

    #[test]
    fn announce_result() {
        let mock_ui = MockUi::builder().build();
        let p1 = Player::Human(String::from("Steve"));
        let p2 = Player::Human(String::from("Another Steve"));
        let mut game = Game::new(&p1, &p2, &mock_ui);

        game.announce_result(); // GameState::NotStarted by default
        assert_eq!(
            mock_ui.notify_result_calls(),
            0,
            "Ui shouldn't be notified if game isn't finished"
        );

        game.game_state = GameState::Ongoing;
        game.announce_result();
        assert_eq!(
            mock_ui.notify_result_calls(),
            0,
            "Ui shouldn't be notified if game isn't finished"
        );

        game.game_state = GameState::Finished(GameResult::Draw);
        game.announce_result();
        assert_eq!(
            mock_ui.notify_result_calls(),
            1,
            "Ui should be notified when game is finished"
        );
    }

    #[test]
    fn player1_win_check() {
        let mock_ui = MockUi::builder().build();
        let p1 = Player::Human(String::from("Steve"));
        let p2 = Player::Human(String::from("Another Steve"));
        let mut game = Game::new(&p1, &p2, &mock_ui);

        game.game_state = GameState::Ongoing;

        game.board[2] = Cell::O;
        game.board[4] = Cell::O;

        game.check_if_over();

        assert_eq!(
            game.game_state,
            GameState::Ongoing,
            "Game state should remain ongoing if there's no winner"
        );

        game.board[6] = Cell::O; // indices 2-4-6 - secondary diagonal

        game.check_if_over();

        match game.game_state {
            GameState::Finished(GameResult::PlayerWon(_id, name, _winning_line)) => {
                assert_eq!(name, "Steve")
            }
            _ => panic!("Player 1 (playing with 'O' won), game state should reflect that"),
        }
    }

    #[test]
    fn player2_win_check() {
        let mock_ui = MockUi::builder().build();
        let p1 = Player::Human(String::from("Steve"));
        let p2 = Player::Human(String::from("Another Steve"));
        let mut game = Game::new(&p1, &p2, &mock_ui);

        game.game_state = GameState::Ongoing;

        game.board[1] = Cell::X;
        game.board[4] = Cell::X;

        game.check_if_over();

        assert_eq!(
            game.game_state,
            GameState::Ongoing,
            "Game state should remain ongoing if there's no winner"
        );

        game.board[7] = Cell::X; // indices 1-4-7 - second column

        game.check_if_over();

        match game.game_state {
            GameState::Finished(GameResult::PlayerWon(_id, name, _winning_line)) => {
                assert_eq!(name, "Another Steve")
            }
            _ => panic!("Player 2 (playing with 'X' won), game state should reflect that"),
        }
    }

    #[test]
    fn draw_check() {
        let mock_ui = MockUi::builder().build();
        let p1 = Player::Human(String::from("Steve"));
        let p2 = Player::Human(String::from("Another Steve"));
        let mut game = Game::new(&p1, &p2, &mock_ui);

        game.game_state = GameState::Ongoing;
        game.board = Board::from([
            Cell::O,
            Cell::X,
            Cell::O,
            Cell::X,
            Cell::X,
            Cell::O,
            Cell::O,
            Cell::O,
            Cell::X,
        ]); // draw board

        game.check_if_over();

        assert_eq!(
            game.game_state,
            GameState::Finished(GameResult::Draw),
            "On draw, game state should be Finished with result Draw"
        );
    }

    #[test]
    fn player_make_move() {
        let mock_ui = MockUi::builder().build();
        let p1 = Player::Human(String::from("Steve"));
        let p2 = Player::Human(String::from("Another Steve"));
        let mut game = Game::new(&p1, &p2, &mock_ui);

        assert_eq!(
            game.board[0],
            Cell::Empty('1'),
            "Cell 1 should be empty at the beginning"
        );
        assert_eq!(
            game.board[1],
            Cell::Empty('2'),
            "Cell 2 should be empty at the beginning"
        );

        game.current_player_make_move(BoardMove::try_new(1).unwrap());

        assert_eq!(
            game.board[0],
            Cell::O,
            "Cell 1 should contain 'O' after player 1's move"
        );
        assert_eq!(
            game.board[1],
            Cell::Empty('2'),
            "Cell 2 should be empty after player 1's move"
        );

        game.current_player = 1; // switch to player 2
        game.current_player_make_move(BoardMove::try_new(2).unwrap());

        assert_eq!(
            game.board[0],
            Cell::O,
            "Cell 1 should contain 'O' after player 1's move"
        );
        assert_eq!(
            game.board[1],
            Cell::X,
            "Cell 2 should contain 'X' after player 2's move"
        );
    }

    #[test]
    fn players_take_turn() {
        let mock_ui = MockUi::builder()
            .expected_moves(vec![
                BoardMove::try_new(1).unwrap(), // Player1 goes top left
                BoardMove::try_new(2).unwrap(), // Player2 goes top middle
                BoardMove::try_new(4).unwrap(), // Player1 goes middle left
            ])
            .build();
        let p1 = Player::Human(String::from("Steve"));
        let p2 = Player::Human(String::from("Another Steve"));
        let mut game = Game::new(&p1, &p2, &mock_ui);

        game.game_state = GameState::Ongoing;

        assert_eq!(
            game.board[0],
            Cell::Empty('1'),
            "Cell 1 should be empty at the beginning"
        );
        assert_eq!(
            game.board[1],
            Cell::Empty('2'),
            "Cell 2 should be empty at the beginning"
        );
        assert_eq!(
            game.board[3],
            Cell::Empty('4'),
            "Cell 4 should be empty at the beginning"
        );

        game.take_turn();

        assert_eq!(
            game.board[0],
            Cell::O,
            "Cell 1 should contain 'O' after player 1's move"
        );
        assert_eq!(
            game.board[1],
            Cell::Empty('2'),
            "Cell 2 should be empty after player 1's move"
        );
        assert_eq!(
            game.board[3],
            Cell::Empty('4'),
            "Cell 4 should be empty after player 1's move"
        );

        game.current_player = 1;
        game.take_turn();

        assert_eq!(
            game.board[0],
            Cell::O,
            "Cell 1 shouldn't change state after subsequent moves"
        );
        assert_eq!(
            game.board[1],
            Cell::X,
            "Cell 2 should contain 'X' after player 2's move"
        );
        assert_eq!(
            game.board[3],
            Cell::Empty('4'),
            "Cell 4 should be empty after player 2's move"
        );

        game.current_player = 0;
        game.take_turn();

        assert_eq!(
            game.board[0],
            Cell::O,
            "Cell 1 shouldn't change state after subsequent moves"
        );
        assert_eq!(
            game.board[1],
            Cell::X,
            "Cell 2 shouldn't change state after subsequent moves"
        );
        assert_eq!(
            game.board[3],
            Cell::O,
            "Cell 4 should contain 'O' after player 1's move"
        );
    }

    #[test]
    fn start_works_only_if_game_is_not_started() {
        let mock_ui = MockUi::builder().build();
        let p1 = Player::CPU;
        let p2 = Player::CPU;
        let mut game = Game::new(&p1, &p2, &mock_ui);

        game.game_state = GameState::Ongoing;
        let result = game.start();
        assert!(matches!(result, Err(())), "There should be no result");

        game.game_state = GameState::Finished(GameResult::Draw);
        let result = game.start();
        assert!(matches!(result, Ok(_)), "There should be a result");

        game.game_state = GameState::Finished(GameResult::PlayerWon(0, String::from("CPU"), 0));
        let result = game.start();
        assert!(matches!(result, Ok(_)), "There should be a result");
        let result = game.start();
        assert!(matches!(result, Ok(_)), "There should be a result");

        assert_eq!(
            mock_ui.get_move_calls(),
            0,
            "There should be no effect if start() was called in NotStarted state"
        );

        assert_eq!(
            mock_ui.notify_result_calls(),
            0,
            "UI should only be notified if start() was called in NotStarted state"
        );

        game.game_state = GameState::NotStarted;
        let result = game.start();
        assert!(matches!(result, Ok(_)), "There should be a result");

        assert_eq!(
            mock_ui.notify_result_calls(),
            1,
            "UI should only be notified if start() was called in NotStarted state"
        );

        let result = game.start();
        assert!(matches!(result, Ok(_)), "There should still be a result");

        assert_eq!(
            mock_ui.notify_result_calls(),
            1,
            "UI should be notified only once"
        );
    }

    #[test]
    fn full_game_player_1_wins() {
        let mock_ui = MockUi::builder()
            .expected_moves(vec![
                BoardMove::try_new(1).unwrap(),
                BoardMove::try_new(7).unwrap(),
                BoardMove::try_new(9).unwrap(),
                BoardMove::try_new(5).unwrap(),
                BoardMove::try_new(3).unwrap(),
                BoardMove::try_new(6).unwrap(),
                BoardMove::try_new(2).unwrap(), // Player 1 wins
            ])
            .build();
        let p1 = Player::Human(String::from("Steve"));
        let p2 = Player::Human(String::from("Another Steve"));

        let result = Game::new(&p1, &p2, &mock_ui).start();

        if let Ok(GameResult::PlayerWon(_id, name, _winning_line)) = result {
            assert_eq!(name, "Steve");
        } else {
            panic!("Player 1 is clearly ahead, he should definitely win");
        }

        assert_eq!(
            mock_ui.notify_result_calls(),
            1,
            "Game should notify player about the result via UI"
        );
    }

    #[test]
    fn full_game_draw() {
        let mock_ui = MockUi::builder()
            .expected_moves(vec![
                BoardMove::try_new(9).unwrap(),
                BoardMove::try_new(5).unwrap(),
                BoardMove::try_new(7).unwrap(),
                BoardMove::try_new(8).unwrap(),
                BoardMove::try_new(2).unwrap(),
                BoardMove::try_new(1).unwrap(),
                BoardMove::try_new(6).unwrap(),
                BoardMove::try_new(3).unwrap(),
                BoardMove::try_new(4).unwrap(), // Draw
            ])
            .build();
        let p1 = Player::Human(String::from("Steve"));
        let p2 = Player::Human(String::from("Another Steve"));

        let result = Game::new(&p1, &p2, &mock_ui).start();

        assert!(
            matches!(result, Ok(GameResult::Draw)),
            "CPU should always draw CPU"
        );

        assert_eq!(
            mock_ui.notify_result_calls(),
            1,
            "Game should notify player about the result via UI"
        );
    }

    #[test]
    fn cpu_vs_cpu_always_draws() {
        let mock_ui = MockUi::builder().build();
        let p1 = Player::CPU;
        let p2 = Player::CPU;

        for _ in 0..10 {
            let result = Game::new(&p1, &p2, &mock_ui).start();
            assert!(
                matches!(result, Ok(GameResult::Draw)),
                "CPU should always draw CPU"
            );
        }
    }
}
