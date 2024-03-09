use crate::game::Game;
use crate::player::Player;
use crate::ui::Ui;

pub enum GameMode {
    PlayerVsPlayer,
    PlayerVsCpu,
    CpuVsPlayer,
    CpuVsCpu,
    Quit,
}

pub struct TicTacToe<T: Ui> {
    ui: T,
    mode: Option<GameMode>,
}

impl<T: Ui> TicTacToe<T> {
    pub fn new(ui: T) -> TicTacToe<T> {
        TicTacToe { ui, mode: None }
    }

    pub fn start(&mut self) {
        loop {
            self.mode = Some(self.ui.select_mode());

            if let Some((player1, player2)) = self.create_players() {
                loop {
                    Game::new(&player1, &player2, &self.ui).start();

                    if !self.ui.keep_playing() {
                        break;
                    }
                }
            } else {
                break;
            }
        }
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
