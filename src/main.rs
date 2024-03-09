use tic_tac_toe::{Game, Player, TerminalUi};

fn main() {
    // let player1 = Player::Human(String::from("P1"));
    // let player2 = Player::Human(String::from("P2"));
    let player1 = Player::CPU;
    let player2 = Player::CPU;
    let ui = TerminalUi::new();
    let mut game = Game::new(&player1, &player2, &ui);
    game.start();
}
