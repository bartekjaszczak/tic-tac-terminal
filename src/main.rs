use tic_tac_terminal::{TicTacToe, TerminalUi};

fn main() {
    let ui = TerminalUi::new();
    let mut game = TicTacToe::new(&ui);
    game.start();
}
