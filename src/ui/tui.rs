use super::Ui;
use crate::board::{Board, Cell, BoardMove, WINNING_LINES};
use crate::game::GameResult;
use crossterm::style::Stylize;
use std::{
    cell::RefCell,
    io::{self, Write},
};

pub struct TerminalUi {
    board: RefCell<Board>,
    winning_line: RefCell<Option<[usize; 3]>>,
}

const PREFIX: &str = " > ";

impl Ui for TerminalUi {
    fn get_move(&self, player_name: &str, additional_message: Option<&str>) -> BoardMove {
        if let Some(msg) = additional_message {
            println!(
                "{PREFIX}{}, {}. Try again: ",
                player_name.green().bold(),
                msg
            );
        } else {
            println!(
                "{PREFIX}{}, your move! Enter a number: ",
                player_name.green().bold()
            );
        }

        io::stdout().flush().unwrap();

        Self::get_move_from_user()
    }

    fn update_board(&self, board: &Board) {
        // Update local board copy
        self.board.replace(board.clone());

        self.draw_board();
    }

    fn notify_result(&self, result: &GameResult) {
        let message = match result {
            GameResult::Draw => format!("{}", "It's a draw!\n".white()),
            GameResult::PlayerWon(winner, winning_line_index) => {
                self.winning_line
                    .replace(Some(WINNING_LINES[*winning_line_index]));
                format!("{} won!\n", &winner.clone().green().bold().underlined())
            }
        };

        self.draw_board();

        println!("{PREFIX}{message}");
        io::stdout().flush().unwrap();
    }
}

impl TerminalUi {
    pub fn new() -> TerminalUi {
        TerminalUi {
            board: RefCell::new(Board::new()),
            winning_line: RefCell::new(None),
        }
    }

    fn draw_board(&self) {
        let styled_cells: Vec<_> = self
            .board
            .borrow()
            .iter()
            .enumerate()
            .map(|(index, cell)| {
                let mut styled_cell = match cell {
                    Cell::Empty(position) => format!("[{}]", position).grey(),
                    Cell::O => " O ".to_string().dark_blue().bold(),
                    Cell::X => " X ".to_string().dark_magenta().bold(),
                };

                if let Some(cell_positions) = self.winning_line.borrow().as_ref() {
                    if cell_positions.contains(&index) {
                        styled_cell = styled_cell.reverse()
                    }
                }

                styled_cell
            })
            .collect();

        TerminalUi::clear_screen();

        println!("\n   +-----+-----+-----+");
        println!("   |     |     |     |");
        println!(
            "   | {} | {} | {} |",
            styled_cells[0], styled_cells[1], styled_cells[2]
        );
        println!("   |     |     |     |");
        println!("   +-----+-----+-----+");
        println!("   |     |     |     |");
        println!(
            "   | {} | {} | {} |",
            styled_cells[3], styled_cells[4], styled_cells[5]
        );
        println!("   |     |     |     |");
        println!("   +-----+-----+-----+");
        println!("   |     |     |     |");
        println!(
            "   | {} | {} | {} |",
            styled_cells[6], styled_cells[7], styled_cells[8]
        );
        println!("   |     |     |     |");
        println!("   +-----+-----+-----+\n");

        io::stdout().flush().unwrap();
    }

    fn get_move_from_user() -> BoardMove {
        loop {
            let user_input = Self::get_user_input();

            break match user_input.parse() {
                Ok(number) => {
                    let board_move = BoardMove::try_new(number);
                    match board_move {
                        Ok(board_move) => board_move,
                        Err(_) => {
                            println!("{PREFIX}Your input must be between 1 and 9! Try again: ");
                            io::stdout().flush().unwrap();

                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("{PREFIX}Your input must be a number between 1 and 9! Try again: ");
                    io::stdout().flush().unwrap();

                    continue;
                }
            };
        }
    }

    fn get_user_input() -> String {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        buffer.trim().to_string()
    }

    fn clear_screen() {
        print!("\x1B[2J");
        print!("\x1B[H");
        io::stdout().flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_at_creation() {
        let tui = TerminalUi::new();

        for cell in tui.board.borrow().iter() {
            match cell {
                Cell::Empty(_) => (),
                _ => panic!("Cells should be empty at creation"),
            }
        }

        assert_eq!(
            *tui.winning_line.borrow(),
            None,
            "There should be no winning line at creation"
        );
    }

    #[test]
    fn update_board() {
        let tui = TerminalUi::new();

        let mut fake_board: Board = Board::new();
        fake_board[2] = Cell::O;
        fake_board[4] = Cell::X;
        fake_board[7] = Cell::Empty('8');

        tui.update_board(&fake_board);

        assert_eq!(*tui.board.borrow(), fake_board, "Board should be updated");
    }

    #[test]
    fn draw() {
        let tui = TerminalUi::new();
        let result = GameResult::Draw;

        tui.notify_result(&result);

        assert_eq!(
            *tui.winning_line.borrow(),
            None,
            "There should be no winning line in draw"
        );
    }

    #[test]
    fn player_won() {
        let tui = TerminalUi::new();
        let result = GameResult::PlayerWon(String::from("Steve"), 3);

        tui.notify_result(&result);

        assert_eq!(*tui.winning_line.borrow(), Some(WINNING_LINES[3]));
    }
}
