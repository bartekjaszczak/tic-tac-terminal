use super::Ui;
use crate::board::{Board, BoardMove, Cell, WINNING_LINES};
use crate::game::GameResult;
use crate::tictactoe::GameMode;
use crossterm::style::{StyledContent, Stylize};
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
        let player_name = TerminalUi::format_text_by_player(
            player_name,
            &self.board.borrow().current_player_symbol(),
        );

        if let Some(msg) = additional_message {
            print!("{PREFIX}{}, {}. Try again: ", player_name, msg);
        } else {
            print!("{PREFIX}{}, your move! Enter a number: ", player_name,);
        }

        io::stdout().flush().unwrap();

        Self::get_move_from_user()
    }

    fn update_board(&self, board: &Board) {
        // Update local board copy
        self.board.replace(board.clone());

        if board.is_empty() {
            // New game - clear previous win
            self.winning_line.replace(None);
        }

        self.draw_board();
    }

    fn notify_result(&self, result: &GameResult) {
        let message = match result {
            GameResult::Draw => format!("{}", "It's a draw!\n".white()),
            GameResult::PlayerWon(_winner_index, winner_name, winning_line_index) => {
                let winning_line = WINNING_LINES[*winning_line_index];
                self.winning_line.replace(Some(winning_line));

                let winner_name = TerminalUi::format_text_by_player(
                    winner_name,
                    &self.board.borrow()[winning_line[0]],
                );
                format!("{} won!\n", winner_name.underlined())
            }
        };

        self.draw_board();

        println!("{PREFIX}{message}");
        io::stdout().flush().unwrap();
    }

    fn get_player_name(&self, player_name: &str) -> String {
        Self::clear_screen();

        print!("{}, enter your name: ", player_name);
        io::stdout().flush().unwrap();

        Self::get_user_input()
    }

    fn select_mode(&self) -> GameMode {
        Self::clear_screen();

        println!("Select game mode!");
        Self::print_game_modes();
        print!("Your choice: ");
        io::stdout().flush().unwrap();

        loop {
            let user_input = Self::get_user_input();

            break match user_input.to_lowercase().as_str() {
                "1" | "[1]" => GameMode::PlayerVsPlayer,
                "2" | "[2]" => GameMode::PlayerVsCpu,
                "3" | "[3]" => GameMode::CpuVsPlayer,
                "4" | "[4]" => GameMode::CpuVsCpu,
                "0" | "q" => GameMode::Quit,
                _ => {
                    println!("Incorrect input! Here are the options again:");
                    Self::print_game_modes();
                    print!("Enter a number between 1 and 4. To quit, enter 0 or q: ");
                    io::stdout().flush().unwrap();

                    continue;
                }
            };
        }
    }

    fn keep_playing(&self) -> bool {
        print!("Again? y/n: ");
        io::stdout().flush().unwrap();

        loop {
            let user_input = Self::get_user_input();

            break match user_input.to_lowercase().as_str() {
                "y" | "yes" => true,
                "n" | "no" => false,
                _ => {
                    print!("Incorrect input! Do you want to play again? Enter [y]es or [no]: ");
                    io::stdout().flush().unwrap();
                    continue;
                }
            };
        }
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
                    Cell::Empty(_) => format!("[{}]", cell).grey(),
                    _ => TerminalUi::format_text_by_player(format!(" {} ", cell).as_str(), cell),
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
                            print!("{PREFIX}Your input must be between 1 and 9! Try again: ");
                            io::stdout().flush().unwrap();

                            continue;
                        }
                    }
                }
                Err(_) => {
                    print!("{PREFIX}Your input must be a number between 1 and 9! Try again: ");
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

    fn format_text_by_player(text: &str, current_player_symbol: &Cell) -> StyledContent<String> {
        match current_player_symbol {
            &Cell::O => text.to_string().bold().blue(),
            &Cell::X => text.to_string().bold().green(),
            _ => text.to_string().grey(),
        }
    }

    fn print_game_modes() {
        println!("[1] Player vs Player");
        println!("[2] Player vs CPU (Player starts)");
        println!("[3] CPU vs Player (CPU starts)");
        println!("[4] CPU vs CPU");
        println!("[0 or q] to quit!")
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
        let result = GameResult::PlayerWon(0, String::from("Steve"), 3);

        tui.notify_result(&result);

        assert_eq!(
            *tui.winning_line.borrow(),
            Some(WINNING_LINES[3]),
            "Winning line should be stored"
        );

        tui.update_board(&Board::new()); // New game starts

        assert_eq!(
            *tui.winning_line.borrow(),
            None,
            "Winning line should be deleted as soon as new game starts"
        );
    }
}
