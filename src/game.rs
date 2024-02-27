use crate::board::{Board, Cell, Move};
use crate::player::Player;
use crate::ui::Ui;

pub const WINNING_LINES: [[usize; 3]; 8] = [
    [0, 3, 6], // 1st column
    [1, 4, 7], // 2nd column
    [2, 5, 8], // 3rd column
    [0, 1, 2], // 1st row
    [3, 4, 5], // 2nd row
    [6, 7, 8], // 3rd row
    [0, 4, 8], // main diagonal
    [2, 4, 6], // secondary diagonal
];

pub type WinningLineIndex = usize;

#[derive(PartialEq)]
pub enum GameResult {
    PlayerWon(String, WinningLineIndex),
    Draw,
}

#[derive(PartialEq)]
enum GameState {
    NotStarted,
    Ongoing,
    Finished(GameResult),
}

pub struct Game<T: Ui> {
    board: Board,
    players: [Player; 2],
    current_player: usize,
    game_state: GameState,
    ui: T,
}

impl<T: Ui> Game<T> {
    pub fn new(player1: Player, player2: Player, ui_backend: T) -> Game<T> {
        let mut game = Game {
            board: [Cell::Empty(0); 9],
            players: [player1, player2],
            current_player: 0,
            game_state: GameState::NotStarted,
            ui: ui_backend,
        };

        for i in 0..9 {
            game.board[i] = Cell::Empty(i + 1); // Put numbers 1..=9 into Empty cells. They'll
                                                // serve as cell positions
        }

        game
    }

    pub fn start(&mut self) {
        if self.game_state == GameState::NotStarted {
            self.game_state = GameState::Ongoing;

            while self.game_state == GameState::Ongoing {
                self.take_turn();
                self.check_if_over();

                self.current_player = if self.current_player == 0 { 1 } else { 0 };
            }

            self.announce_result();
        }
    }

    fn take_turn(&mut self) {
        let current_player = &self.players[self.current_player];

        self.ui.update_board(&self.board);

        let mut error_message = None;

        let player_move = loop {
            let player_move = current_player.get_move(&self.board, &self.ui, error_message);
            if self.is_valid_move(&player_move) {
                break player_move;
            } else {
                error_message = Some("this cell is not empty");
            }
        };

        self.current_player_make_move(player_move);
    }

    fn current_player_make_move(&mut self, player_move: Move) {
        self.board[player_move.index()] = if self.current_player == 0 {
            Cell::O
        } else {
            Cell::X
        };
    }

    fn is_valid_move(&self, player_move: &Move) -> bool {
        match self.board[player_move.index()] {
            Cell::Empty(_) => true,
            _ => false,
        }
    }

    fn check_if_over(&mut self) {
        let b = &self.board;

        for (index, cell_triplet) in WINNING_LINES.iter().enumerate() {
            let (c1, c2, c3) = (
                &b[cell_triplet[0]],
                &b[cell_triplet[1]],
                &b[cell_triplet[2]],
            );
            if c1 == c2 && c1 == c3 {
                let winner = match c1 {
                    &Cell::O => 0,
                    &Cell::X => 1,
                    _ => continue, // false alarm - it's row/col/diag of empty cells
                };

                let winner_name = if let Player::Human(name) = &self.players[winner] {
                    name.clone()
                } else {
                    String::from("CPU")
                };

                self.game_state = GameState::Finished(GameResult::PlayerWon(
                    winner_name,
                    index as WinningLineIndex,
                ));
                return;
            }
        }

        let board_is_full = !self
            .board
            .iter()
            .any(|&cell| matches!(cell, Cell::Empty(_)));

        if board_is_full {
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
