use super::Ui;
use crate::board::{Board, Cell, Move};
use crate::game::{GameResult, WINNING_LINES};
use crossterm::{
    cursor, execute,
    style::{Print, Stylize},
    terminal,
};
use std::{cell::RefCell, io};

const PREFIX: &str = " > ";

pub struct TerminalUi {
    board: RefCell<Board>,
    winning_line: RefCell<Option<[usize; 3]>>,
}

impl Ui for TerminalUi {
    fn get_move(&self, player_name: &str, additional_message: Option<&str>) -> Move {
        if let Some(msg) = additional_message {
            execute!(
                io::stdout(),
                Print(format!(
                    "{PREFIX}{}, {}. Try again: ",
                    player_name.green().bold(),
                    msg
                ))
            )
            .unwrap();
        } else {
            execute!(
                io::stdout(),
                Print(format!(
                    "{PREFIX}{}, your move! Enter a number: ",
                    player_name.green().bold()
                ))
            )
            .unwrap();
        }

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

        execute!(io::stdout(), Print(format!("{PREFIX}{message}"))).unwrap();
    }
}

impl TerminalUi {
    pub fn new() -> TerminalUi {
        TerminalUi {
            board: RefCell::new([Cell::Empty(0); 9]),
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

        execute!(
            io::stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            Print("\n   +-----+-----+-----+\n"),
            Print("   |     |     |     |\n"),
            Print(format!(
                "   | {} | {} | {} |\n",
                styled_cells[0], styled_cells[1], styled_cells[2]
            )),
            Print("   |     |     |     |\n"),
            Print("   +-----+-----+-----+\n"),
            Print("   |     |     |     |\n"),
            Print(format!(
                "   | {} | {} | {} |\n",
                styled_cells[3], styled_cells[4], styled_cells[5]
            )),
            Print("   |     |     |     |\n"),
            Print("   +-----+-----+-----+\n"),
            Print("   |     |     |     |\n"),
            Print(format!(
                "   | {} | {} | {} |\n",
                styled_cells[6], styled_cells[7], styled_cells[8]
            )),
            Print("   |     |     |     |\n"),
            Print("   +-----+-----+-----+\n\n"),
        )
        .unwrap();
    }

    fn get_move_from_user() -> Move {
        loop {
            let user_input = Self::get_user_input();

            break match user_input.parse() {
                Ok(number) => {
                    let player_move = Move::try_new(number);
                    match player_move {
                        Ok(player_move) => player_move,
                        Err(_) => {
                            execute!(
                                io::stdout(),
                                Print(format!(
                                    "{PREFIX}Your input must be between 1 and 9! Try again: "
                                ))
                            )
                            .unwrap();

                            continue;
                        }
                    }
                }
                Err(_) => {
                    execute!(
                        io::stdout(),
                        Print(format!(
                            "{PREFIX}Your input must be a number between 1 and 9! Try again: "
                        ))
                    )
                    .unwrap();

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
}
