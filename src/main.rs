use tic_tac_toe::{TicTacToe, TerminalUi};

fn main() {
    let ui = TerminalUi::new();
    let mut game = TicTacToe::new(ui);
    game.start();
}
