//! This module contains main Tic-Tac-Toe application (which has the main game loop).

use crate::game::{Game, GameResult};
use crate::player::Player;
use crate::ui::Ui;

pub enum GameMode {
    PlayerVsPlayer,
    PlayerVsCpu,
    CpuVsPlayer,
    CpuVsCpu,
    Quit,
}

pub struct TicTacToe<'a, T: Ui> {
    ui: &'a T,
    mode: Option<GameMode>,
    scores: (i32, i32),
}

impl<'a, T: Ui> TicTacToe<'a, T> {
    /// Creates new instance of Tic-Tac-Toe game. Accepts one argument - UI backend.
    ///
    /// # Examples
    ///
    /// ```
    /// use tic_tac_toe::{TerminalUi, TicTacToe};
    ///
    /// let ui = TerminalUi::new();
    /// let game = TicTacToe::new(&ui);
    /// ```
    pub fn new(ui: &'a T) -> Self {
        Self {
            ui,
            mode: None,
            scores: (0, 0),
        }
    }

    /// Starts the Tic-Tac-Toe application.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tic_tac_toe::{TerminalUi, TicTacToe};
    ///
    /// let ui = TerminalUi::new();
    /// let mut game = TicTacToe::new(&ui);
    /// game.start();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics at every time the UI panics (pretty much only on stdin and stdout errors).
    pub fn start(&mut self) {
        loop {
            self.mode = Some(self.ui.select_mode());
            self.scores = (0, 0);

            if let Some((player1, player2)) = self.create_players() {
                loop {
                    let result = Game::new(&player1, &player2, self.ui).start();
                    if let Ok(result) = result {
                        self.update_scores(&player1, &player2, &result);
                    }

                    if !self.ui.keep_playing() {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    fn update_scores(&mut self, player1: &Player, player2: &Player, result: &GameResult) {
        match result {
            GameResult::PlayerWon(0, _winner_name, _winning_line) => self.scores.0 += 1,
            GameResult::PlayerWon(1, _winner_name, _winning_line) => self.scores.1 += 1,
            _ => (),
        }

        let (player1_score, player2_score) = self.scores;
        let (player1_name, player2_name) = (player1.get_name(), player2.get_name());
        self.ui
            .update_scores(player1_name, player1_score, player2_name, player2_score);
    }

    fn create_players(&self) -> Option<(Player, Player)> {
        match self.mode {
            Some(GameMode::PlayerVsPlayer) => {
                let player1_name = self.ui.get_player_name("Player1");
                let player2_name = self.ui.get_player_name("Player2");
                Some((Player::Human(player1_name), Player::Human(player2_name)))
            }
            Some(GameMode::PlayerVsCpu) => {
                let player1_name = self.ui.get_player_name("Player1");
                Some((Player::Human(player1_name), Player::CPU))
            }
            Some(GameMode::CpuVsPlayer) => {
                let player2_name = self.ui.get_player_name("Player2");
                Some((Player::CPU, Player::Human(player2_name)))
            }
            Some(GameMode::CpuVsCpu) => Some((Player::CPU, Player::CPU)),
            Some(GameMode::Quit) => None,
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::tests::MockUi;

    #[test]
    fn update_scores() {
        let mock_ui = MockUi::builder().build();
        let human = Player::Human(String::from("Steve"));
        let cpu = Player::CPU;

        let mut ttt = TicTacToe::new(&mock_ui);

        assert_eq!(ttt.scores, (0, 0), "Initial score should be 0 to 0");

        ttt.update_scores(
            &human,
            &cpu,
            &GameResult::PlayerWon(0, String::from("Steve"), 0),
        );

        assert_eq!(ttt.scores, (1, 0), "Player1's score should be incremented");

        ttt.update_scores(
            &human,
            &cpu,
            &GameResult::PlayerWon(1, String::from("Steve"), 0),
        );

        assert_eq!(ttt.scores, (1, 1), "Player2's score should be incremented");

        ttt.update_scores(&human, &cpu, &GameResult::Draw);

        assert_eq!(ttt.scores, (1, 1), "Draw shouldn't change the score");

        assert_eq!(
            mock_ui.update_scores_count(),
            3,
            "UI should be notified for every score update (even if it's a draw)"
        );
    }

    #[test]
    fn create_players() {
        let mock_ui = MockUi::builder()
            .expected_names(vec![
                String::from("Steve"),
                String::from("Second Steve"),
                String::from("Llama"),
                String::from("Elon"),
            ])
            .build();

        let mut ttt = TicTacToe::new(&mock_ui);

        let players = ttt.create_players();
        assert!(
            matches!(players, None),
            "create_players should return None for no mode selected"
        );

        ttt.mode = Some(GameMode::PlayerVsPlayer);
        let players = ttt.create_players();
        if let Some((p1, p2)) = players {
            assert_eq!(p1.get_name(), "Steve", "Player 1 should be named Steve");
            assert_eq!(
                p2.get_name(),
                "Second Steve",
                "Player 2 should be named Second Steve"
            );
        }

        ttt.mode = Some(GameMode::PlayerVsCpu);
        let players = ttt.create_players();
        if let Some((p1, p2)) = players {
            assert_eq!(p1.get_name(), "Llama", "Player 1 should be named Llama");
            assert_eq!(
                p2.get_name(),
                "CPU",
                "Player 2 is CPU and should be named CPU"
            );
        }

        ttt.mode = Some(GameMode::CpuVsPlayer);
        let players = ttt.create_players();
        if let Some((p1, p2)) = players {
            assert_eq!(
                p1.get_name(),
                "CPU",
                "Player 1 is CPU and should be named CPU"
            );
            assert_eq!(p2.get_name(), "Elon", "Player 2 should be named Elon");
        }

        ttt.mode = Some(GameMode::CpuVsCpu);
        let players = ttt.create_players();
        if let Some((p1, p2)) = players {
            assert_eq!(
                p1.get_name(),
                "CPU",
                "Player 1 is CPU and should be named CPU"
            );
            assert_eq!(
                p2.get_name(),
                "CPU",
                "Player 2 is CPU and should be named CPU"
            );
        }
    }
}
