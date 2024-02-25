use tic_tac_toe::{Game, Player};

fn main() {
    let player1 = Player::Human(String::from("P1"));
    let player2 = Player::Human(String::from("P2"));
    let mut game = Game::new(player1, player2);
    game.start();
}
